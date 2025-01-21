use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};

use crate::Pool;

use super::transfer_tokens;

#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mint::token_program = token_program)]
    pub token_a_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(mint::token_program = token_program)]
    pub token_b_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        mut,
        associated_token::mint = token_a_mint,
        associated_token::authority = user,
        associated_token::token_program = token_program
    )]
    pub user_accout_a: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = token_b_mint,
        associated_token::authority = user,
        associated_token::token_program = token_program
    )]
    pub user_account_b: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        has_one = token_a_mint,
        has_one = token_b_mint,
        seeds = [b"pool", token_a_mint.key().as_ref(), token_b_mint.key().as_ref()],
        bump = pool.bump
    )]
    pub pool: Box<Account<'info, Pool>>,

    #[account(
        mut,
        associated_token::mint = token_a_mint,
        associated_token::authority = pool,
        associated_token::token_program = token_program
    )]
    pub pool_account_a: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = token_b_mint,
        associated_token::authority = pool,
        associated_token::token_program = token_program
    )]
    pub pool_account_b: Box<InterfaceAccount<'info, TokenAccount>>,

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

    // create a transfer function for the pool as it is the pda
    let seeds = &[
        b"pool",
        context.accounts.token_a_mint.to_account_info().key.as_ref(),
        context.accounts.token_b_mint.to_account_info().key.as_ref(),
        &[context.accounts.pool.bump],
    ];

    let signer_seeds = [&seeds[..]];

    let accounts = TransferChecked {
        from: context.accounts.pool_account_b.to_account_info(),
        mint: context.accounts.token_b_mint.to_account_info(),
        to: context.accounts.user_account_b.to_account_info(),
        authority: context.accounts.pool.to_account_info(),
    };

    let cpi_context = CpiContext::new_with_signer(
        context.accounts.token_program.to_account_info(),
        accounts,
        &signer_seeds,
    );

    transfer_checked(
        cpi_context,
        amount_b,
        context.accounts.token_b_mint.decimals,
    )
}
