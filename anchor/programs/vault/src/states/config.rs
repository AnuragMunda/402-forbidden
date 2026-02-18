use anchor_lang::prelude::*;

/// Long-term configuration of the game
#[account]
#[derive(InitSpace)]
pub struct Config {
    pub admin: Pubkey, // Address of the Admin
    pub arena_count: u8, // Total arena count 
    pub usdc_mint: Pubkey, // USDC token address
    pub platform_fee_account: Pubkey, // ATA of the platform for USDC
    pub platform_fee_bps: u16, // Percentage the platform will cut from the fees
    pub bump: u8 // Config PDA bump
}