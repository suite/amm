pub mod errors;
pub mod instructions;
pub mod state;
pub mod helpers;

use anchor_lang::prelude::*;

pub use instructions::*;
pub use state::*;

declare_id!("9ToweKkCye2iuGeAwsUBGqz6E6LEgc1uHVJ48sV2ZZwR");

#[program]
pub mod amm {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        seed: u64,
        fee: u16,
    ) -> Result<()> {
        ctx.accounts.init_config(seed, fee, ctx.bumps.config, ctx.bumps.mint_lp)?;
        Ok(())
    }
    
    pub fn deposit(
        ctx: Context<Deposit>,
        amount: u64,
        max_x: u64,
        max_y: u64,
        expiration: i64
    ) -> Result<()> {
        ctx.accounts.deposit(amount, max_x, max_y, expiration)?;
        Ok(())
    }

    pub fn swap(ctx: Context<Swap>, is_x: bool, amount: u64, min_receive: u64) -> Result<()> {
        ctx.accounts.swap(is_x, amount, min_receive)?;
        Ok(())
    }

    // TOOD: Withdraw, burn LP tokens to withdraw liquid
}
