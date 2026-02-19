use anchor_lang::prelude::*;

pub const MAX_BPS: u16 = 10_000; // Max basis point
pub const BPS_CAP: u16 = 5000;   // Max cap on platform fee

/// Long-term configuration of the game
#[account]
#[derive(InitSpace)]
pub struct Config {
    pub admin: Pubkey, // Address of the Admin
    pub arena_count: u8, // Total arena count 
    pub mint: Pubkey, // Mint token address
    pub treasury_ata: Pubkey, // ATA to hold the platform treasury
    pub platform_fee_bps: u16, // Percentage the platform will cut from the fees
    pub bump: u8 // Config PDA bump
}