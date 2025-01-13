use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::Pool;

use super::transfer_tokens;

#[derive(Accounts)]
#[instruction(id:u64)]
pub struct Swap<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mint::token_program = token_program)]
    pub token_a_mint: InterfaceAccount<'info, Mint>,

    #[account(mint::token_program = token_program)]
    pub token_b_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = token_a_mint,
        associated_token::authority = user,
        associated_token::token_program = token_program
    )]
    pub user_accout_a: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = token_b_mint,
        associated_token::authority = user,
        associated_token::token_program = token_program
    )]
    pub user_account_b: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        has_one = token_a_mint,
        has_one = token_b_mint,
        seeds = [b"pool", token_a_mint.key().as_ref(), token_b_mint.key().as_ref(), id.to_le_bytes().as_ref()],
        bump = pool.bump
    )]
    pub pool: Account<'info, Pool>,

    #[account(
        mut,
        associated_token::mint = token_a_mint,
        associated_token::authority = pool,
        associated_token::token_program = token_program
    )]
    pub pool_account_a: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = token_b_mint,
        associated_token::authority = pool,
        associated_token::token_program = token_program
    )]
    pub pool_account_b: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
}

pub fn transfer_swapped_tokens(context: Context<Swap>, amount_a: u64, amount_b: u64) -> Result<()> {
    transfer_tokens(
        &context.accounts.user_accout_a,
        &context.accounts.pool_account_a,
        &amount_a,
        &context.accounts.token_a_mint,
        &context.accounts.user,
        &context.accounts.token_program,
    )?;

    transfer_tokens(
        &context.accounts.pool_account_b,
        &context.accounts.user_account_b,
        &amount_b,
        &context.accounts.token_b_mint,
        &context.accounts.user,
        &context.accounts.token_program,
    )
}
