use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct ProtocolConfig {
    pub protocol_authority: Pubkey,
    pub protocol_fee_bps: u16,
    pub treasury: Pubkey,
    pub bump: u8
}