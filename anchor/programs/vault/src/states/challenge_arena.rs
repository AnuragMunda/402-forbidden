use anchor_lang::prelude::*;

/// The main state of the current challenge
#[account]
#[derive(InitSpace)]
pub struct ChallengeArena {
    pub arena_id: u8, // The id of the Arena
    pub round_id: u64, // The id of current round
    pub initial_prize: u64, // The initial amount stored in vault as prize money
    pub secret_hash: [u8; 32], // The hash of the real secret/password
    pub vault_ata: Pubkey, // The ATA for the vault
    pub guess_fee: u64, // The fee charged to submit password
    pub hint_fee: u64, // The fee charged to generate a new hint
    pub is_active: bool, // Flag indicating if the challenge is active/inactive
    pub bump: u8 // Bump for Challenge Arena PDA
}