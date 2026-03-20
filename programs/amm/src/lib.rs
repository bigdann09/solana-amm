use anchor_lang::prelude::*;

pub mod state;
pub mod constants;

pub use constants::*;
pub mod instructions {
    pub mod initialize_protocol;
}

pub use instructions::{
    initialize_protocol::*,
};

declare_id!("3Khr5BhYMT5nXWp3E9EXPFwdt3KWJq8WyLErvRaZ1zyt");

#[program]
pub mod amm {
    use super::*;

    pub fn initialize_protocol(ctx: Context<InitializeProtocol>, fee_bps: u16) -> Result<()> {
        instructions::initialize_protocol::handler(ctx, fee_bps)
    }
}
