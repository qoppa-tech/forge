use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Debug, PartialEq, Eq)]
pub struct Loan {
    pub vault: Pubkey,
    pub loan_id: [u8; 32],
    pub borrower: Pubkey,
    pub destination: Pubkey,
    pub principal: u64,
    pub term_rate_bps: u16,
    pub fixed_interest: u64,
    pub fixed_payoff: u64,
    pub term_seconds: i64,
    pub offer_expiry: i64,
    pub approvals: [bool; 2],
    pub state: LoanState,
    pub proposed_at: i64,
    pub disbursed_at: i64,
    pub repaid_at: i64,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, InitSpace, Debug, PartialEq, Eq)]
pub enum LoanState {
    Proposed,
    Approved,
    Active,
    Repaid,
}
