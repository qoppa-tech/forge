use anchor_lang::prelude::*;

/// Fixed-payoff loan proposed from a vault to one borrower destination.
///
/// Amount fields are denominated in the vault mint's base units. Rate fields use
/// basis points (`10_000` bps = 100%). Time fields use Unix timestamps/seconds.
/// `fixed_interest` and `fixed_payoff` are computed once at proposal time from
/// `principal` and `term_rate_bps` and are not recomputed later.
#[account]
#[derive(InitSpace, Debug, PartialEq, Eq)]
pub struct Loan {
    /// Vault account that owns this loan and defines limits, mint, and approvers.
    pub vault: Pubkey,
    /// Vault-scoped identifier used with `vault` to derive the loan PDA.
    pub loan_id: [u8; 32],
    /// Signer allowed to draw and repay this loan.
    pub borrower: Pubkey,
    /// Borrower-owned token account that receives principal on draw.
    pub destination: Pubkey,
    /// Loan principal in mint base units.
    pub principal: u64,
    /// Fixed term interest rate in basis points (`10_000` bps = 100%).
    pub term_rate_bps: u16,
    /// Fixed interest amount in mint base units, computed at proposal time.
    pub fixed_interest: u64,
    /// Total repayment amount in mint base units: `principal + fixed_interest`.
    pub fixed_payoff: u64,
    /// Contractual loan term duration in seconds.
    pub term_seconds: i64,
    /// Unix timestamp after which an approved offer can no longer be drawn.
    pub offer_expiry: i64,
    /// Per-approver approval flags aligned with `Vault::approvers`.
    pub approvals: [bool; 2],
    /// Current lifecycle state for instruction gating.
    pub state: LoanState,
    /// Unix timestamp when the loan account was proposed.
    pub proposed_at: i64,
    /// Unix timestamp when principal was disbursed; zero until drawn.
    pub disbursed_at: i64,
    /// Unix timestamp when fixed payoff was repaid; zero until repaid.
    pub repaid_at: i64,
    /// PDA bump for `seeds = [b"loan", vault, loan_id]`.
    pub bump: u8,
}

/// Loan lifecycle state.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, InitSpace, Debug, PartialEq, Eq)]
pub enum LoanState {
    /// Proposed by an approver and waiting for both approvals.
    Proposed,
    /// Approved by both configured vault approvers and eligible to draw before expiry.
    Approved,
    /// Principal has been disbursed and remains outstanding.
    Active,
    /// Fixed payoff has been repaid and principal removed from vault outstanding total.
    Repaid,
}
