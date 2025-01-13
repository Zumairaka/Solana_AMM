use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Pool {
    pub id: u64,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub liquidity_mint: Pubkey,
    pub token_a_amount: u64,
    pub token_b_amount: u64,
    pub total_liquidity: u64,
    pub bump: u8,
}
