use anchor_lang::prelude::*;

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

pub(crate) use instructions::{
    __client_accounts_approve_loan, __client_accounts_create_vault, __client_accounts_draw_loan,
    __client_accounts_fund_vault, __client_accounts_propose_loan, __client_accounts_repay_loan,
    __client_accounts_set_disbursement_paused, __client_accounts_withdraw_available,
};
pub use instructions::{
    ApproveLoan, CreateVault, DrawLoan, FundVault, ProposeLoan, RepayLoan, SetDisbursementPaused,
    WithdrawAvailable,
};
pub use state::{Loan, LoanState, Vault};

pub use constants::TEST_TOKEN_DECIMALS;
pub use error::ForgeError;

declare_id!("4niyjcxN6ySLBVePpUaUHrUEQZvyKDyNYfiQ8N4FUTgP");

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
        instructions::create_vault(ctx, vault_id, approvers, per_loan_limit, outstanding_limit)
    }

    pub fn fund_vault(ctx: Context<FundVault>, amount: u64) -> Result<()> {
        instructions::fund_vault(ctx, amount)
    }

    pub fn propose_loan(
        ctx: Context<ProposeLoan>,
        loan_id: [u8; 32],
        principal: u64,
        term_rate_bps: u16,
        term_seconds: i64,
        offer_expiry: i64,
    ) -> Result<()> {
        instructions::propose_loan(
            ctx,
            loan_id,
            principal,
            term_rate_bps,
            term_seconds,
            offer_expiry,
        )
    }

    pub fn approve_loan(ctx: Context<ApproveLoan>) -> Result<()> {
        instructions::approve_loan(ctx)
    }

    pub fn draw_loan(ctx: Context<DrawLoan>) -> Result<()> {
        instructions::draw_loan(ctx)
    }

    pub fn repay_loan(ctx: Context<RepayLoan>) -> Result<()> {
        instructions::repay_loan(ctx)
    }

    pub fn withdraw_available(ctx: Context<WithdrawAvailable>, amount: u64) -> Result<()> {
        instructions::withdraw_available(ctx, amount)
    }

    pub fn set_disbursement_paused(
        ctx: Context<SetDisbursementPaused>,
        paused: bool,
    ) -> Result<()> {
        instructions::set_disbursement_paused(ctx, paused)
    }
}
