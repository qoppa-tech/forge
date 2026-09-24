use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, TransferChecked};

pub mod loan;
pub(crate) use loan::{
    __client_accounts_approve_loan, __client_accounts_draw_loan, __client_accounts_propose_loan,
    __client_accounts_repay_loan, __client_accounts_set_disbursement_paused,
    __client_accounts_withdraw_available,
};
pub use loan::{
    ApproveLoan, DrawLoan, Loan, LoanState, ProposeLoan, RepayLoan, SetDisbursementPaused,
    WithdrawAvailable,
};

declare_id!("4niyjcxN6ySLBVePpUaUHrUEQZvyKDyNYfiQ8N4FUTgP");

pub const TEST_TOKEN_DECIMALS: u8 = 6;

#[program]
pub mod forge {
    use super::*;

    pub fn create_vault(
        ctx: Context<CreateVault>,
        vault_id: [u8; 32],
        approvers: [Pubkey; 2],
        per_loan_limit: u64,
        outstanding_limit: u64,
    ) -> Result<()> {
        require!(
            approvers[0] != approvers[1] && approvers.iter().all(|key| *key != Pubkey::default()),
            ForgeError::InvalidApprovers
        );
        require!(
            per_loan_limit > 0 && outstanding_limit >= per_loan_limit,
            ForgeError::InvalidLimits
        );
        ctx.accounts.vault.set_inner(Vault {
            treasury: ctx.accounts.treasury.key(),
            vault_id,
            mint: ctx.accounts.mint.key(),
            token_account: ctx.accounts.vault_token_account.key(),
            approvers,
            treasury_destination: ctx.accounts.treasury_destination.key(),
            per_loan_limit,
            outstanding_limit,
            outstanding_principal: 0,
            disbursement_paused: false,
            bump: ctx.bumps.vault,
        });
        Ok(())
    }

    pub fn fund_vault(ctx: Context<FundVault>, amount: u64) -> Result<()> {
        require!(amount > 0, ForgeError::InvalidAmount);
        token::transfer_checked(
            CpiContext::new(
                ctx.accounts.token_program.key(),
                TransferChecked {
                    from: ctx.accounts.source.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.vault_token_account.to_account_info(),
                    authority: ctx.accounts.treasury.to_account_info(),
                },
            ),
            amount,
            ctx.accounts.mint.decimals,
        )
    }

    pub fn propose_loan(
        ctx: Context<ProposeLoan>,
        loan_id: [u8; 32],
        principal: u64,
        term_rate_bps: u16,
        term_seconds: i64,
        offer_expiry: i64,
    ) -> Result<()> {
        loan::propose_loan(
            ctx,
            loan_id,
            principal,
            term_rate_bps,
            term_seconds,
            offer_expiry,
        )
    }

    pub fn approve_loan(ctx: Context<ApproveLoan>) -> Result<()> {
        loan::approve_loan(ctx)
    }

    pub fn draw_loan(ctx: Context<DrawLoan>) -> Result<()> {
        loan::draw_loan(ctx)
    }

    pub fn repay_loan(ctx: Context<RepayLoan>) -> Result<()> {
        loan::repay_loan(ctx)
    }

    pub fn withdraw_available(ctx: Context<WithdrawAvailable>, amount: u64) -> Result<()> {
        loan::withdraw_available(ctx, amount)
    }

    pub fn set_disbursement_paused(
        ctx: Context<SetDisbursementPaused>,
        paused: bool,
    ) -> Result<()> {
        loan::set_disbursement_paused(ctx, paused)
    }
}

#[derive(Accounts)]
#[instruction(vault_id: [u8; 32])]
pub struct CreateVault<'info> {
    #[account(mut)]
    pub treasury: Signer<'info>,
    #[account(
        init,
        payer = treasury,
        space = 8 + Vault::INIT_SPACE,
        seeds = [b"vault", treasury.key().as_ref(), &vault_id],
        bump
    )]
    pub vault: Account<'info, Vault>,
    #[account(constraint = mint.decimals == TEST_TOKEN_DECIMALS @ ForgeError::InvalidMintDecimals)]
    pub mint: Account<'info, Mint>,
    #[account(
        init,
        payer = treasury,
        seeds = [b"tokens", vault.key().as_ref()],
        bump,
        token::mint = mint,
        token::authority = vault,
        token::token_program = token_program
    )]
    pub vault_token_account: Account<'info, TokenAccount>,
    #[account(token::mint = mint, token::authority = treasury)]
    pub treasury_destination: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct FundVault<'info> {
    pub treasury: Signer<'info>,
    #[account(
        seeds = [b"vault", vault.treasury.as_ref(), &vault.vault_id],
        bump = vault.bump,
        has_one = treasury,
        has_one = mint
    )]
    pub vault: Account<'info, Vault>,
    pub mint: Account<'info, Mint>,
    #[account(mut, token::mint = mint, token::authority = treasury)]
    pub source: Account<'info, TokenAccount>,
    #[account(
        mut,
        address = vault.token_account,
        token::mint = mint,
        token::authority = vault
    )]
    pub vault_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[account]
#[derive(InitSpace, Debug, PartialEq)]
pub struct Vault {
    pub treasury: Pubkey,
    pub vault_id: [u8; 32],
    pub mint: Pubkey,
    pub token_account: Pubkey,
    pub approvers: [Pubkey; 2],
    pub treasury_destination: Pubkey,
    pub per_loan_limit: u64,
    pub outstanding_limit: u64,
    pub outstanding_principal: u64,
    pub disbursement_paused: bool,
    pub bump: u8,
}

#[error_code]
pub enum ForgeError {
    #[msg("Approvers must be distinct, nondefault public keys")]
    InvalidApprovers,
    #[msg("Limits must be positive and per-loan limit cannot exceed portfolio limit")]
    InvalidLimits,
    #[msg("FORGE_TEST_USD requires six decimals")]
    InvalidMintDecimals,
    #[msg("Funding amount must be positive")]
    InvalidAmount,
    #[msg("The signer is not a configured approver")]
    UnauthorizedApprover,
    #[msg("Loan term must be positive")]
    InvalidTerm,
    #[msg("Offer expiry must be in the future")]
    InvalidExpiry,
    #[msg("Loan principal exceeds the per-loan limit")]
    PerLoanLimitExceeded,
    #[msg("Loan arithmetic overflowed")]
    MathOverflow,
    #[msg("Loan arithmetic underflowed")]
    MathUnderflow,
    #[msg("Loan is in an invalid state for this instruction")]
    InvalidLoanState,
    #[msg("This approver has already approved the loan")]
    AlreadyApproved,
    #[msg("Disbursement is paused")]
    DisbursementPaused,
    #[msg("Loan offer has expired")]
    LoanExpired,
    #[msg("Outstanding principal limit exceeded")]
    OutstandingLimitExceeded,
    #[msg("Vault has insufficient liquidity")]
    InsufficientLiquidity,
    #[msg("Borrower does not match the loan")]
    InvalidBorrower,
    #[msg("Destination does not belong to the borrower")]
    InvalidDestination,
}
