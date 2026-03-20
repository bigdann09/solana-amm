use anchor_lang::prelude::*;

use crate::{constants::PROTOCOL_SEED, state::ProtocolConfig};

#[derive(Accounts)]
pub struct InitializeProtocol<'info> {
    #[account(
        init,
        payer=payer,
        space=8 + ProtocolConfig::INIT_SPACE,
        seeds=[PROTOCOL_SEED],
        bump
    )]
    pub protocol: Account<'info, ProtocolConfig>,

    #[account(mut, signer)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub treasury: SystemAccount<'info>,
    #[account(mut)]
    pub protocol_authority: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializeProtocol>, fee_bps: u16) -> Result<()> {
    ctx.accounts.protocol.set_inner(ProtocolConfig {
        protocol_authority: *ctx.accounts.protocol_authority.key,
        protocol_fee_bps: fee_bps,
        treasury: *ctx.accounts.treasury.key,
        bump: ctx.bumps.protocol,
    });
    Ok(())
}
