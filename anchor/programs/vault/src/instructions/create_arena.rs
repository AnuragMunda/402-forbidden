use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, Token, TokenAccount, Transfer, transfer}};

use crate::{states::{ChallengeArena, Config}, utils::GameError};

#[derive(Accounts)]
pub struct CreateArena<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        has_one = admin,
        seeds = [b"config"],
        bump = config.bump
    )]
    pub config: Account<'info, Config>,
    
    #[account(
        init,
        payer = admin,
        space = ChallengeArena::DISCRIMINATOR.len() + ChallengeArena::INIT_SPACE,
        seeds = [b"arena", config.arena_count.to_le_bytes().as_ref()],
        bump
    )]
    pub arena: Account<'info, ChallengeArena>,

    #[account(address = config.mint)]
    pub mint: Account<'info, Mint>,

    #[account(
        init,
        payer = admin,
        associated_token::mint = mint,
        associated_token::authority = arena
    )]
    pub vault_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        address = config.treasury_ata,
        associated_token::mint = mint,
        associated_token::authority = config
    )]
    pub treasury_ata: Account<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateArena<'info> {
    pub fn create_arena(
        &mut self,
        initial_prize: u64,
        secret_hash: [u8; 32],
        guess_fee: u64,
        hint_fee: u64,
        bumps: CreateArenaBumps
    ) -> Result<()> {
        require!(initial_prize > 0 || guess_fee > 0 || hint_fee > 0, GameError::InvalidAmount);
        require!(self.treasury_ata.amount > initial_prize, GameError::InsufficientTreasuryFunds);

        // Transfer tokens from treasury -> vault
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.treasury_ata.to_account_info(),
            to: self.vault_ata.to_account_info(),
            authority: self.arena.to_account_info()
        };

        let seed = self.config.arena_count.to_le_bytes();

        let signer_seeds: &[&[&[u8]]] = &[&[b"arena", seed.as_ref(), &[bumps.arena]]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer(cpi_ctx, initial_prize)?;

        self.arena.set_inner(ChallengeArena {
            arena_id: self.config.arena_count,
            round_id: 1,
            initial_prize,
            secret_hash,
            vault_ata: self.vault_ata.key(),
            guess_fee,
            hint_fee,
            is_active: true,
            bump: bumps.arena
        });

        Ok(())
    }
}
