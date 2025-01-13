use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    mint_to, transfer_checked, Mint, MintTo, TokenAccount, TokenInterface, TransferChecked,
};

pub fn transfer_tokens<'info>(
    from: &InterfaceAccount<'info, TokenAccount>,
    to: &InterfaceAccount<'info, TokenAccount>,
    amount: &u64,
    mint: &InterfaceAccount<'info, Mint>,
    authority: &Signer<'info>,
    token_program: &Interface<'info, TokenInterface>,
) -> Result<()> {
    let transfer_checked_accounts = TransferChecked {
        from: from.to_account_info(),
        to: to.to_account_info(),
        mint: mint.to_account_info(),
        authority: authority.to_account_info(),
    };

    let cpi_context = CpiContext::new(token_program.to_account_info(), transfer_checked_accounts);

    transfer_checked(cpi_context, *amount, mint.decimals)
}

pub fn mint_tokens<'info>(
    to: &InterfaceAccount<'info, TokenAccount>,
    amount: &u64,
    mint: &InterfaceAccount<'info, Mint>,
    authority: &Signer<'info>,
    token_program: &Interface<'info, TokenInterface>,
) -> Result<()> {
    let mint_accounts = MintTo {
        mint: mint.to_account_info(),
        to: to.to_account_info(),
        authority: authority.to_account_info(),
    };

    let cpi_context = CpiContext::new(token_program.to_account_info(), mint_accounts);

    mint_to(cpi_context, *amount)
}
