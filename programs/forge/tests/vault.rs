mod support;

use anchor_lang::{
    prelude::Pubkey,
    solana_program::{
        instruction::error::InstructionError, program_pack::Pack, system_instruction,
        system_program,
    },
    AccountDeserialize, InstructionData,
};
use anchor_spl::token::spl_token::{self, instruction as token_instruction, state};
use solana_keypair::Keypair;
use solana_signer::Signer;
use solana_transaction_error::TransactionError;

use support::*;

#[test]
fn create_and_fund_exact_test_token_balance() {
    let mut f = Fixture::new();
    f.initialize();
    let account = f.svm.get_account(&f.vault).unwrap();
    assert_eq!(account.owner, forge::ID);
    let vault = forge::Vault::try_deserialize(&mut account.data.as_slice()).unwrap();
    assert_eq!(vault.treasury, f.treasury.pubkey());
    assert_eq!(vault.vault_id, VAULT_ID);
    assert_eq!(vault.mint, f.mint);
    assert_eq!(vault.token_account, f.tokens);
    assert_eq!(
        vault.approvers,
        f.approvers.each_ref().map(|key| key.pubkey())
    );
    assert_eq!(vault.treasury_destination, f.source);
    assert_eq!(vault.per_loan_limit, FUNDING / 2);
    assert_eq!(vault.outstanding_limit, FUNDING);
    assert_eq!(vault.outstanding_principal, 0);
    assert!(!vault.disbursement_paused);
    assert_eq!(
        vault.bump,
        Pubkey::find_program_address(
            &[b"vault", f.treasury.pubkey().as_ref(), &VAULT_ID],
            &forge::ID
        )
        .1
    );
    let tokens = f.svm.get_account(&f.tokens).unwrap();
    assert_eq!(tokens.owner, spl_token::ID);
    let token = state::Account::unpack(&tokens.data).unwrap();
    assert_eq!(token.owner, f.vault);
    assert_eq!(token.mint, f.mint);
    assert_eq!(token.amount, 0);
    assert!(token.delegate.is_none());
    assert!(token.close_authority.is_none());
    let funding = f.execute(f.fund(FUNDING)).expect("fund_vault must succeed");
    assert_eq!(f.balance(f.tokens), FUNDING);
    assert_eq!(f.balance(f.source), 0);
    assert_eq!(
        f.svm.get_account(&f.vault).unwrap().data,
        account.data,
        "funding cannot mutate immutable configuration"
    );
    println!(
        "FORGE_TEST_USD fixture: LiteSVM only; no validator, deployment or redemption promise"
    );
    println!("treasury={}", f.treasury.pubkey());
    println!("approver_a={}", f.approvers[0].pubkey());
    println!("approver_b={}", f.approvers[1].pubkey());
    println!(
        "mint={} decimals=6 fixture_supply_base_units={FUNDING}",
        f.mint
    );
    println!("vault={} token_account={}", f.vault, f.tokens);
    println!(
        "funding_signature={} (in-process, not an explorer receipt)",
        funding.signature
    );
    println!(
        "treasury_balance_base_units={} vault_balance_base_units={}",
        f.balance(f.source),
        f.balance(f.tokens)
    );
}

#[test]
fn invalid_approvers_and_limits_are_rejected() {
    let mut f = Fixture::new();
    let a = f.approvers[0].pubkey();
    let b = f.approvers[1].pubkey();
    for (approvers, per_loan_limit, outstanding_limit, code) in [
        ([a, a], 1, 2, 6000),
        ([Pubkey::default(), b], 1, 2, 6000),
        ([a, Pubkey::default()], 1, 2, 6000),
        ([a, b], 0, 2, 6001),
        ([a, b], 1, 0, 6001),
        ([a, b], 3, 2, 6001),
    ] {
        let mut ix = f.create();
        ix.data = forge::instruction::CreateVault {
            vault_id: VAULT_ID,
            approvers,
            per_loan_limit,
            outstanding_limit,
        }
        .data();
        custom_error(f.execute(ix), code);
        assert!(f.svm.get_account(&f.vault).is_none());
        assert!(f.svm.get_account(&f.tokens).is_none());
    }
}

#[test]
fn creation_rejects_decimals_and_wrong_destination_owner_or_mint() {
    let mut f = Fixture::new();
    let other_mint = create_mint(&mut f.svm, &f.treasury, 9);
    let other_source =
        create_token_account(&mut f.svm, &f.treasury, other_mint, f.treasury.pubkey());
    let mut ix = f.create();
    ix.accounts[2].pubkey = other_mint;
    ix.accounts[4].pubkey = other_source;
    custom_error(f.execute(ix), 6002);
    let other_owner = create_token_account(&mut f.svm, &f.treasury, f.mint, f.outsider.pubkey());
    let mut ix = f.create();
    ix.accounts[4].pubkey = other_owner;
    custom_error(f.execute(ix), 2015);
    let mut ix = f.create();
    ix.accounts[4].pubkey = other_source;
    custom_error(f.execute(ix), 2014);
}

#[test]
fn create_requires_real_treasury_signature_and_matching_seeds() {
    let mut f = Fixture::new();
    let mut tx = transaction(&f.svm, &f.treasury, &[f.create()], &[]);
    tx.signatures[0] = Default::default();
    assert_eq!(
        f.svm.send_transaction(tx).unwrap_err().err,
        TransactionError::SignatureFailure
    );
    let mut ix = f.create();
    ix.accounts[0].is_signer = false;
    custom_error(send(&mut f.svm, &f.outsider, &[ix], &[]), 3010);
    let mut ix = f.create();
    ix.accounts[0].pubkey = f.outsider.pubkey();
    custom_error(send(&mut f.svm, &f.outsider, &[ix], &[]), 2006);
}

#[test]
fn unexpected_token_program_is_rejected_on_both_instructions() {
    let mut f = Fixture::new();
    for program in [system_program::ID, anchor_spl::token_2022::ID] {
        let mut ix = f.create();
        ix.accounts[5].pubkey = program;
        custom_error(f.execute(ix), 3008);
        assert!(f.svm.get_account(&f.vault).is_none());
        assert!(f.svm.get_account(&f.tokens).is_none());
    }
    f.initialize();
    for program in [system_program::ID, anchor_spl::token_2022::ID] {
        let mut ix = f.fund(1);
        ix.accounts[5].pubkey = program;
        f.reject_funding(ix, 3008);
    }
}

#[test]
fn token_2022_mint_is_rejected_even_with_legacy_token_program() {
    use anchor_spl::token_2022::spl_token_2022;
    let mut f = Fixture::new();
    let mint = Keypair::new();
    let rent = f.svm.minimum_balance_for_rent_exemption(state::Mint::LEN);
    send(
        &mut f.svm,
        &f.treasury,
        &[
            system_instruction::create_account(
                &f.treasury.pubkey(),
                &mint.pubkey(),
                rent,
                state::Mint::LEN as u64,
                &spl_token_2022::ID,
            ),
            spl_token_2022::instruction::initialize_mint2(
                &spl_token_2022::ID,
                &mint.pubkey(),
                &f.treasury.pubkey(),
                None,
                6,
            )
            .unwrap(),
        ],
        &[&mint],
    )
    .unwrap();
    let mut ix = f.create();
    ix.accounts[2].pubkey = mint.pubkey();
    custom_error(f.execute(ix), 3007); // Anchor: AccountOwnedByWrongProgram.
    assert!(f.svm.get_account(&f.vault).is_none());
    assert!(f.svm.get_account(&f.tokens).is_none());
    f.initialize();
    let mut ix = f.fund(1);
    ix.accounts[2].pubkey = mint.pubkey();
    f.reject_funding(ix, 3007);
}

#[test]
fn funding_rejects_unauthorized_or_missing_treasury_signer() {
    let mut f = Fixture::new();
    f.initialize();
    let mut ix = f.fund(1);
    ix.accounts[0].pubkey = f.outsider.pubkey();
    custom_error(send(&mut f.svm, &f.outsider, &[ix], &[]), 2001);
    let mut ix = f.fund(1);
    ix.accounts[0].is_signer = false;
    custom_error(send(&mut f.svm, &f.outsider, &[ix], &[]), 3010);
    assert_eq!(f.balance(f.tokens), 0);
    assert_eq!(f.balance(f.source), FUNDING);
}

#[test]
fn funding_rejects_wrong_mint_source_owner_and_amounts() {
    let mut f = Fixture::new();
    f.initialize();
    let other_mint = create_mint(&mut f.svm, &f.treasury, 6);
    let other_source =
        create_token_account(&mut f.svm, &f.treasury, other_mint, f.treasury.pubkey());
    let outsider_source =
        create_token_account(&mut f.svm, &f.treasury, f.mint, f.outsider.pubkey());
    let mut ix = f.fund(1);
    ix.accounts[2].pubkey = other_mint;
    f.reject_funding(ix, 2001);
    let mut ix = f.fund(1);
    ix.accounts[3].pubkey = other_source;
    f.reject_funding(ix, 2014);
    let mut ix = f.fund(1);
    ix.accounts[3].pubkey = outsider_source;
    f.reject_funding(ix, 2015);
    f.reject_funding(f.fund(0), 6003);
    f.reject_funding(f.fund(FUNDING + 1), 1);
}

#[test]
fn vaults_are_isolated_and_ids_cannot_be_reused() {
    let mut f = Fixture::new();
    f.initialize();
    let original_vault = f.vault;
    let original_tokens = f.tokens;
    let second_id = [2; 32];
    f.vault = Pubkey::find_program_address(
        &[b"vault", f.treasury.pubkey().as_ref(), &second_id],
        &forge::ID,
    )
    .0;
    f.tokens = Pubkey::find_program_address(&[b"tokens", f.vault.as_ref()], &forge::ID).0;
    let mut ix = f.create();
    ix.data = forge::instruction::CreateVault {
        vault_id: second_id,
        approvers: f.approvers.each_ref().map(|k| k.pubkey()),
        per_loan_limit: FUNDING / 2,
        outstanding_limit: FUNDING,
    }
    .data();
    f.execute(ix.clone()).unwrap();
    custom_error(f.execute(ix), 0); // System Program: AccountAlreadyInUse.
    let mut ix = f.fund(1);
    ix.accounts[4].pubkey = original_tokens;
    f.reject_funding(ix, 2012);
    let mut ix = f.fund(1);
    ix.accounts[1].pubkey = original_vault;
    f.reject_funding(ix, 2012);
    assert_eq!(f.balance(original_tokens), 0);
}

#[test]
fn direct_spl_transfer_cannot_drain_pda_vault() {
    let mut f = Fixture::new();
    f.initialize();
    f.execute(f.fund(FUNDING)).unwrap();
    let destination = create_token_account(&mut f.svm, &f.treasury, f.mint, f.outsider.pubkey());
    let theft = token_instruction::transfer_checked(
        &spl_token::ID,
        &f.tokens,
        &f.mint,
        &destination,
        &f.outsider.pubkey(),
        &[],
        1,
        6,
    )
    .unwrap();
    custom_error(send(&mut f.svm, &f.outsider, &[theft], &[]), 4); // SPL Token: OwnerMismatch.
    let mut unsigned_pda = token_instruction::transfer_checked(
        &spl_token::ID,
        &f.tokens,
        &f.mint,
        &destination,
        &f.vault,
        &[],
        1,
        6,
    )
    .unwrap();
    unsigned_pda.accounts[3].is_signer = false;
    assert_eq!(
        send(&mut f.svm, &f.outsider, &[unsigned_pda], &[])
            .unwrap_err()
            .err,
        TransactionError::InstructionError(0, InstructionError::MissingRequiredSignature)
    );
    assert_eq!(f.balance(f.tokens), FUNDING);
    assert_eq!(f.balance(destination), 0);
}
