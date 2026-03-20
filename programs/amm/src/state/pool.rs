use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Pool {
    pub token_mint_a: Pubkey,
    pub token_mint_a: Pubkey,
    pub vault_a: Pubkey,
    pub vault_b: Pubkey,
    pub lp_mint: Pubkey,
    pub vault_authority: Pubkey,
    pub authority: Pubkey, // for updating fees
    pub fee_bps: u16,
    pub vault_authority_bump: u8,
    pub pool_bump: u8,
    pub reserve_a: u64,
    pub reserve_b: u64,
    pub lp_supply: u64,
    pub total_volume_a: u64,
    pub total_volume_b: u64,
    pub total_fees_a: u64,
    pub total_fees_b: u64,
    pub total_protocol_fees_a: u64,
    pub total_protocol_fees_b: u64,
}