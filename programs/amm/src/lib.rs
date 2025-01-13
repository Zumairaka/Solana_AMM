pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("28ADxZuHrV92hBV6CQ9gug4zASrEvosVe2Qm918NqBbm");

#[program]
pub mod amm {
    use super::*;

    pub fn initialize_pool(
        context: Context<InitializePool>,
        id: u64,
        amount_a: u64,
        amount_b: u64,
    ) -> Result<()> {
        instructions::initialize_pool::send_tokens_to_vaults(&context, &amount_a, &amount_b)?;
        instructions::initialize_pool::save_pool_and_mint_liquidity(context, id, amount_a, amount_b)
    }

    pub fn add_liquidity(
        context: Context<AddLiquidity>,
        _id: u64,
        amount_a: u64,
        amount_b: u64,
    ) -> Result<()> {
        instructions::add_liquidity::send_liquidity_tokens_to_vaults(
            &context, &amount_a, &amount_b,
        )?;
        instructions::add_liquidity::save_liquidity_and_mint_liquidity_tokens(
            context, amount_a, amount_b,
        )
    }

    pub fn swap_tokens(context: Context<Swap>, _id: u64, amount_a: u64) -> Result<()> {
        let pool = &mut context.accounts.pool;
        let amount_a_u128 = amount_a as u128;
        let pool_amount_a = pool.token_a_amount as u128;
        let pool_amount_b = pool.token_b_amount as u128;
        let amount_b = (pool_amount_b * amount_a_u128) / (pool_amount_a + amount_a_u128);
        let amount_b = amount_b as u64;

        // update pool
        pool.token_a_amount += amount_a;
        pool.token_b_amount -= amount_b;

        // transfer swapped tokens
        instructions::swap::transfer_swapped_tokens(context, amount_a, amount_b)?;

        Ok(())
    }
}
