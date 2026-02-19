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
        platform_fee_bps: u16,
    ) -> Result<()> {
        ctx.accounts.init_config(platform_fee_bps, ctx.bumps)
    }

    pub fn fund_treasury(ctx: Context<FundTreasury>, amount: u64) -> Result<()> {
        ctx.accounts.fund_treasury(amount)
    }
}
