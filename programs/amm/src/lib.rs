use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod state;

pub mod instructions {
    pub mod create_pool;
    pub mod initialize_protocol;
}

use instructions::{create_pool::*, initialize_protocol::*};

declare_id!("3Khr5BhYMT5nXWp3E9EXPFwdt3KWJq8WyLErvRaZ1zyt");

#[program]
pub mod amm {
    use anchor_spl::associated_token::Create;

    use super::*;

    pub fn initialize_protocol(ctx: Context<InitializeProtocol>, fee_bps: u16) -> Result<()> {
        instructions::initialize_protocol::handler(ctx, fee_bps)
    }

    pub fn create_pool(ctx: Context<CreateLiquidityPool>, fee_bps: u16) -> Result<()> {
        instructions::create_pool::handler(ctx, fee_bps)
    }
}
