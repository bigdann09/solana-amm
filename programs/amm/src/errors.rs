use anchor_lang::prelude::*;

#[error_code]
pub enum Error {
    #[msg("Fee too high")]
    FeeTooHigh,
    #[msg("Token mints identical")]
    IdenticalMints,
}
