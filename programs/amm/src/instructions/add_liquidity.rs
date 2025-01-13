use std::cmp;

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{error::ErrorCode, Pool};

use super::{mint_tokens, transfer_tokens};

#[derive(Accounts)]
#[instruction(id:u64)]
pub struct AddLiquidity<'info> {
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

    #[account(
        init_if_needed,
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

pub fn send_liquidity_tokens_to_vaults(
    context: &Context<AddLiquidity>,
    amount_a: &u64,
    amount_b: &u64,
) -> Result<()> {
    transfer_tokens(
        &context.accounts.user_accout_a,
        &context.accounts.pool_account_a,
        &amount_a,
        &context.accounts.token_a_mint,
        &context.accounts.user,
        &context.accounts.token_program,
    )?;

    transfer_tokens(
        &context.accounts.user_account_b,
        &context.accounts.pool_account_b,
        &amount_b,
        &context.accounts.token_b_mint,
        &context.accounts.user,
        &context.accounts.token_program,
    )
}

pub fn save_liquidity_and_mint_liquidity_tokens(
    context: Context<AddLiquidity>,
    amount_a: u64,
    amount_b: u64,
) -> Result<()> {
    let pool = &mut context.accounts.pool;

    // calculate liquidity amount
    let reserve_a = pool.token_a_amount;
    let reserve_b = pool.token_b_amount;
    let total_supply = pool.total_liquidity;

    let liquidity_amount = cmp::min(
        match amount_a.checked_mul(total_supply) {
            Some(product) => product / reserve_a,
            None => return Err(ErrorCode::MultiplicationOverflow.into()),
        },
        match amount_b.checked_mul(total_supply) {
            Some(product) => product / reserve_b,
            None => return Err(ErrorCode::MultiplicationOverflow.into()),
        },
    );

    // mint liquidity
    mint_tokens(
        &context.accounts.user_liquidity_account,
        &liquidity_amount,
        &context.accounts.liquidity_mint,
        &context.accounts.user,
        &context.accounts.token_program,
    )?;

    pool.token_a_amount += amount_a;
    pool.token_b_amount += amount_b;
    pool.total_liquidity += liquidity_amount;

    Ok(())
}
