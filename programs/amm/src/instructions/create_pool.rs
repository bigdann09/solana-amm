use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{
    constants::{LP_SEED, MAX_FEE_BPS, POOL_SEED, VAULT_AUTHORITY_SEED},
    errors::Error,
    state::Pool,
};

#[derive(Accounts)]
pub struct CreateLiquidityPool<'info> {
    #[account(
        init,
        seeds=[POOL_SEED, token_mint_a.key().as_ref(), token_mint_b.key().as_ref()],
        space= 8 + Pool::INIT_SPACE,
        payer=authority,
        bump
    )]
    pub pool: Account<'info, Pool>,

    #[account(
        init,
        payer=authority,
        mint::decimals=6,
        mint::authority=vault_authority,
        seeds=[LP_SEED, pool.key().as_ref()],
        bump,
    )]
    pub lp_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer=authority,
        associated_token::mint=token_mint_a,
        associated_token::authority=vault_authority
    )]
    pub vault_a: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer=authority,
        associated_token::mint=token_mint_b,
        associated_token::authority=vault_authority
    )]
    pub vault_b: InterfaceAccount<'info, TokenAccount>,

    pub token_mint_a: InterfaceAccount<'info, Mint>,
    pub token_mint_b: InterfaceAccount<'info, Mint>,

    /// CHECK: PDA — no private key, validated by seeds constraint.
    #[account(
        seeds = [VAULT_AUTHORITY_SEED, pool.key().as_ref()],
        bump,
    )]
    pub vault_authority: UncheckedAccount<'info>,

    #[account(mut, signer)]
    pub authority: Signer<'info>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateLiquidityPool>, fee_bps: u16) -> Result<()> {
    require!(fee_bps < MAX_FEE_BPS, Error::FeeTooHigh);
    require!(
        ctx.accounts.token_mint_a.key() < ctx.accounts.token_mint_b.key(),
        Error::IdenticalMints
    );

    ctx.accounts.pool.set_inner(Pool {
        token_mint_a: ctx.accounts.token_mint_a.key(),
        token_mint_b: ctx.accounts.token_mint_b.key(),
        vault_a: ctx.accounts.vault_a.key(),
        vault_b: ctx.accounts.vault_b.key(),
        lp_mint: ctx.accounts.lp_mint.key(),
        vault_authority: *ctx.accounts.vault_authority.key,
        authority: *ctx.accounts.authority.key,
        fee_bps: fee_bps,
        vault_authority_bump: ctx.bumps.vault_authority,
        pool_bump: ctx.bumps.pool,
        reserve_a: 0,
        reserve_b: 0,
        lp_supply: 0,
        total_volume_a: 0,
        total_volume_b: 0,
        total_fees_a: 0,
        total_fees_b: 0,
        total_protocol_fees_a: 0,
        total_protocol_fees_b: 0,
    });
    Ok(())
}
