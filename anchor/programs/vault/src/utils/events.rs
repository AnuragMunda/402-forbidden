use anchor_lang::prelude::*;

#[event]
pub struct ChallengeArenaInitialized {
    pub arena_id: u8,
    pub arena: Pubkey,
    pub is_active: bool
}

#[event]
pub struct ChallengeArenaReset {
    pub arena_id: u8
}

#[event]
pub struct GuessConfirmed {
    pub arena_id: u8,
    pub round_id: u64,
    pub winner: Pubkey,
    pub prize_money: u64
}

#[event]
pub struct GuessRejected {
    pub arena_id: u8,
    pub player: Pubkey
}