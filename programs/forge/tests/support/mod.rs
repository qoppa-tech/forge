#![allow(clippy::result_large_err, dead_code)]

use anchor_lang::{
    prelude::Pubkey,
    solana_program::{
        instruction::{error::InstructionError, Instruction},
        program_pack::Pack,
        system_instruction, system_program,
    },
    InstructionData, ToAccountMetas,
};
use anchor_spl::token::spl_token::{self, instruction as token_instruction, state};
use litesvm::{types::TransactionResult, LiteSVM};
use solana_keypair::Keypair;
use solana_message::{Message, VersionedMessage};
use solana_signer::Signer;
use solana_transaction::versioned::VersionedTransaction;
use solana_transaction_error::TransactionError;

pub const FUNDING: u64 = 10_000_000_000;
pub const VAULT_ID: [u8; 32] = [1; 32];

pub fn transaction(
    svm: &LiteSVM,
    payer: &Keypair,
    ixs: &[Instruction],
    extra: &[&Keypair],
) -> VersionedTransaction {
    let mut signers = vec![payer];
    signers.extend_from_slice(extra);
    let msg = Message::new_with_blockhash(ixs, Some(&payer.pubkey()), &svm.latest_blockhash());
    VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &signers).unwrap()
}

pub fn send(
    svm: &mut LiteSVM,
    payer: &Keypair,
    ixs: &[Instruction],
    extra: &[&Keypair],
) -> TransactionResult {
    let tx = transaction(svm, payer, ixs, extra);
    let result = svm.send_transaction(tx);
    svm.expire_blockhash();
    result
}

pub fn custom_error(result: TransactionResult, code: u32) {
    let failure = result.expect_err("invalid direct call must fail");
    assert_eq!(
        failure.err,
        TransactionError::InstructionError(0, InstructionError::Custom(code)),
        "{}",
        failure.meta.pretty_logs()
    );
}

pub fn create_mint(svm: &mut LiteSVM, treasury: &Keypair, decimals: u8) -> Pubkey {
    let mint = Keypair::new();
    send(
        svm,
        treasury,
        &[
            system_instruction::create_account(
                &treasury.pubkey(),
                &mint.pubkey(),
                svm.minimum_balance_for_rent_exemption(state::Mint::LEN),
                state::Mint::LEN as u64,
                &spl_token::ID,
            ),
            token_instruction::initialize_mint2(
                &spl_token::ID,
                &mint.pubkey(),
                &treasury.pubkey(),
                None,
                decimals,
            )
            .unwrap(),
        ],
        &[&mint],
    )
    .unwrap();
    mint.pubkey()
}

pub fn create_token_account(
    svm: &mut LiteSVM,
    treasury: &Keypair,
    mint: Pubkey,
    owner: Pubkey,
) -> Pubkey {
    let account = Keypair::new();
    send(
        svm,
        treasury,
        &[
            system_instruction::create_account(
                &treasury.pubkey(),
                &account.pubkey(),
                svm.minimum_balance_for_rent_exemption(state::Account::LEN),
                state::Account::LEN as u64,
                &spl_token::ID,
            ),
            token_instruction::initialize_account3(
                &spl_token::ID,
                &account.pubkey(),
                &mint,
                &owner,
            )
            .unwrap(),
        ],
        &[&account],
    )
    .unwrap();
    account.pubkey()
}

pub struct Fixture {
    pub svm: LiteSVM,
    pub treasury: Keypair,
    pub approvers: [Keypair; 2],
    pub outsider: Keypair,
    pub mint: Pubkey,
    pub source: Pubkey,
    pub vault: Pubkey,
    pub tokens: Pubkey,
}

impl Fixture {
    pub fn new() -> Self {
        let mut svm = LiteSVM::new();
        let binary = std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../target/deploy/forge.so"
        ))
        .expect("build SBF first with bash scripts/build.sh");
        svm.add_program(forge::ID, &binary).unwrap();
        let treasury = Keypair::new();
        let approvers = [Keypair::new(), Keypair::new()];
        let outsider = Keypair::new();
        svm.airdrop(&treasury.pubkey(), 10_000_000_000).unwrap();
        svm.airdrop(&outsider.pubkey(), 1_000_000_000).unwrap();
        let mint = create_mint(&mut svm, &treasury, 6);
        let source = create_token_account(&mut svm, &treasury, mint, treasury.pubkey());
        send(
            &mut svm,
            &treasury,
            &[token_instruction::mint_to_checked(
                &spl_token::ID,
                &mint,
                &source,
                &treasury.pubkey(),
                &[],
                FUNDING,
                6,
            )
            .unwrap()],
            &[],
        )
        .unwrap();
        let vault = Pubkey::find_program_address(
            &[b"vault", treasury.pubkey().as_ref(), &VAULT_ID],
            &forge::ID,
        )
        .0;
        let tokens = Pubkey::find_program_address(&[b"tokens", vault.as_ref()], &forge::ID).0;
        Self {
            svm,
            treasury,
            approvers,
            outsider,
            mint,
            source,
            vault,
            tokens,
        }
    }

    pub fn create(&self) -> Instruction {
        Instruction {
            program_id: forge::ID,
            accounts: forge::accounts::CreateVault {
                treasury: self.treasury.pubkey(),
                vault: self.vault,
                mint: self.mint,
                vault_token_account: self.tokens,
                treasury_destination: self.source,
                token_program: spl_token::ID,
                system_program: system_program::ID,
            }
            .to_account_metas(None),
            data: forge::instruction::CreateVault {
                vault_id: VAULT_ID,
                approvers: self.approvers.each_ref().map(|key| key.pubkey()),
                per_loan_limit: FUNDING / 2,
                outstanding_limit: FUNDING,
            }
            .data(),
        }
    }

    pub fn fund(&self, amount: u64) -> Instruction {
        Instruction {
            program_id: forge::ID,
            accounts: forge::accounts::FundVault {
                treasury: self.treasury.pubkey(),
                vault: self.vault,
                mint: self.mint,
                source: self.source,
                vault_token_account: self.tokens,
                token_program: spl_token::ID,
            }
            .to_account_metas(None),
            data: forge::instruction::FundVault { amount }.data(),
        }
    }

    pub fn execute(&mut self, ix: Instruction) -> TransactionResult {
        send(&mut self.svm, &self.treasury, &[ix], &[])
    }

    pub fn execute_as(
        &mut self,
        payer: &Keypair,
        ix: Instruction,
        extra: &[&Keypair],
    ) -> TransactionResult {
        send(&mut self.svm, payer, &[ix], extra)
    }

    pub fn execute_as_approver(
        &mut self,
        index: usize,
        ix: Instruction,
        extra_index: Option<usize>,
    ) -> TransactionResult {
        let payer = &self.approvers[index];
        let extra = extra_index.map(|extra_i| &self.approvers[extra_i]);
        let extra = extra.into_iter().collect::<Vec<_>>();
        send(&mut self.svm, payer, &[ix], &extra)
    }

    pub fn initialize(&mut self) {
        self.execute(self.create())
            .expect("create_vault must succeed");
    }

    pub fn balance(&self, account: Pubkey) -> u64 {
        state::Account::unpack(&self.svm.get_account(&account).unwrap().data)
            .unwrap()
            .amount
    }

    pub fn reject_funding(&mut self, ix: Instruction, code: u32) {
        let before = (self.balance(self.source), self.balance(self.tokens));
        custom_error(self.execute(ix), code);
        assert_eq!(
            (self.balance(self.source), self.balance(self.tokens)),
            before
        );
    }
}
