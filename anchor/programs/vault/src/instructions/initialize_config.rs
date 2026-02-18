use anchor_lang::prelude::*;

use crate::states::Config;

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        space = Config::DISCRIMINATOR.len() + Config::INIT_SPACE,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, Config>,
    
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeConfig<'info> {
    pub fn init_config(
        &mut self,
        usdc_mint: Pubkey,
        platform_fee_account: Pubkey,
        platform_fee_bps: u16,
        bumps: InitializeConfigBumps,
    ) -> Result<()> {
        self.config.set_inner(Config {
            admin: self.admin.key(),
            arena_count: 0u8,
            usdc_mint,
            platform_fee_account,
            platform_fee_bps,
            bump: bumps.config,
        });

        Ok(())
    }
}