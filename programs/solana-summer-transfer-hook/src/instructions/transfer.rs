use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022::Token2022,
    token_interface::{self, Mint, TokenAccount},
};

#[derive(Accounts)]
pub struct Transfer<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK: This account is only used as the authority for the receiver's ATA, safe to be unchecked.
    pub receiver: UncheckedAccount<'info>,
    // mint
    pub mint: InterfaceAccount<'info, Mint>,
    // sender_ata
    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = mint,
        associated_token::authority = authority,
    )]
    pub sender_ata: InterfaceAccount<'info, TokenAccount>,
    // receiver_ata
    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = mint,
        associated_token::authority = receiver,
    )]
    pub receiver_ata: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn handler<'info>(ctx: Context<'info, Transfer<'info>>, amount: u64) -> Result<()> {
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_accounts = token_interface::TransferChecked {
        from: ctx.accounts.sender_ata.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.receiver_ata.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };
    // get remaining accounts
    let cpi_ctx = CpiContext::new(cpi_program.key(), cpi_accounts)
        .with_remaining_accounts(ctx.remaining_accounts.to_vec());
    token_interface::transfer_checked(cpi_ctx, amount, ctx.accounts.mint.decimals)?;
    Ok(())
}
