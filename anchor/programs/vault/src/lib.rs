#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;

pub mod states;
pub mod utils;
pub mod instructions;

use instructions::*;

declare_id!("98oS6C5ndeS1HjSKhr4nVnD4G4qpnT65WgbDp9nEsZo2");

#[program]
pub mod vault {
    use super::*;
    
    pub fn initialize_config(
        ctx: Context<InitializeConfig>,
        usdc_mint: Pubkey,
        platform_fee_account: Pubkey,
        platform_fee_bps: u16,
    ) -> Result<()> {
        ctx.accounts.init_config(usdc_mint, platform_fee_account, platform_fee_bps, ctx.bumps)
    }
}
