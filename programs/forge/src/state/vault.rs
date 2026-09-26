use anchor_lang::prelude::*;

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
