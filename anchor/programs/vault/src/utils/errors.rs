use anchor_lang::prelude::*;

#[error_code]
pub enum GameError {
    #[msg("Invalid Amount")]
    InvalidAmount,
    #[msg("Insufficient Balance available")]
    InsufficientFunds,
    #[msg("Invalid fee")]
    InvalidFee,
}