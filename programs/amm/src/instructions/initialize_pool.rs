use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{error::ErrorCode, Pool, ANCHOR_DISCRIMINATOR};

use super::{calculate_sqrt, mint_tokens, transfer_tokens};

#[derive(Accounts)]
#[instruction(id:u64)]
pub struct InitializePool<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mint::token_program = token_program)]
    pub token_a_mint: InterfaceAccount<'info, Mint>,

    #[account(mint::token_program = token_program)]
    pub token_b_mint: InterfaceAccount<'info, Mint>,

    #[account(mint::token_program = token_program)]
    pub liquidity_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = token_a_mint,
        associated_token::authority = user,
        associated_token::token_program = token_program

    )]
    pub user_account_a: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = token_b_mint,
        associated_token::authority = user,
        associated_token::token_program = token_program
    )]
    pub user_account_b: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = user,
        space = ANCHOR_DISCRIMINATOR + Pool::INIT_SPACE,
        seeds = [b"pool", token_a_mint.key().as_ref(), token_b_mint.key().as_ref(), id.to_le_bytes().as_ref()],
        bump
    )]
    pub pool: Account<'info, Pool>,

    #[account(
        init,
        payer = user,
        associated_token::mint = token_a_mint,
        associated_token::authority = pool,
        associated_token::token_program = token_program
    )]
    pub pool_account_a: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = user,
        associated_token::mint = token_b_mint,
        associated_token::authority = pool,
        associated_token::token_program = token_program
    )]
    pub pool_account_b: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = user,
        associated_token::mint = liquidity_mint,
        associated_token::authority = user,
        associated_token::token_program = token_program
    )]
    pub user_liquidity_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn send_tokens_to_vaults(
    context: &Context<InitializePool>,
    amount_a: &u64,
    amount_b: &u64,
) -> Result<()> {
    // transfer token a
    transfer_tokens(
        &context.accounts.user_account_a,
        &context.accounts.pool_account_a,
        &amount_a,
        &context.accounts.token_a_mint,
        &context.accounts.user,
        &context.accounts.token_program,
    )?;

    // transfer token b
    transfer_tokens(
        &context.accounts.user_account_b,
        &context.accounts.pool_account_b,
        &amount_b,
        &context.accounts.token_b_mint,
        &context.accounts.user,
        &context.accounts.token_program,
    )
}

pub fn save_pool_and_mint_liquidity(
    context: Context<InitializePool>,
    id: u64,
    amount_a: u64,
    amount_b: u64,
) -> Result<()> {
    // calculate liquidity amount
    let liquidity_amount = match amount_a.checked_mul(amount_b) {
        Some(product) => calculate_sqrt(product)?,
        None => return Err(ErrorCode::MultiplicationOverflow.into()),
    };

    // mint liquidity
    mint_tokens(
        &context.accounts.user_liquidity_account,
        &liquidity_amount,
        &context.accounts.liquidity_mint,
        &context.accounts.user,
        &context.accounts.token_program,
    )?;

    context.accounts.pool.set_inner(Pool {
        id,
        token_a_mint: context.accounts.token_a_mint.key(),
        token_b_mint: context.accounts.token_b_mint.key(),
        liquidity_mint: context.accounts.liquidity_mint.key(),
        token_a_amount: amount_a,
        token_b_amount: amount_b,
        total_liquidity: liquidity_amount,
        bump: context.bumps.pool,
    });

    Ok(())
}
