use anchor_lang::prelude::*;

/// The main state of the current challenge
#[account]
#[derive(InitSpace)]
pub struct ChallengeArena {
    pub arena_id: u8, // The id of the Arena
    pub secret_hash: [u8; 32], // The hash of the real secret/password
    pub vault_token_account: Pubkey, // The ATA for the USDC vault
    pub guess_fee_usdc: u64, // The fee charged to submit password
    pub hint_fee_usdc: u64, // The fee charged to generate a new hint
    pub is_active: bool, // Flag indicating if the challenge is active/inactive
    pub round_id: u64, // The id of current round
    pub bump: u8 // Bump for Challenge Arena PDA
}