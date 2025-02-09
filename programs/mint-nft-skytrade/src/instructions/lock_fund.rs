use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, transfer, Transfer};
use crate::constants::*;
use crate::errors::Errors;
use crate::state::{Asset, TokenWhitelist};

#[derive(Accounts)]
pub struct LockFund<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    /// CHECK: should be vetted from front end
    pub cnft: UncheckedAccount<'info>,

    /// CHECK: This account is modified in the downstream program
    pub asset_info: AccountInfo<'info>,

    #[account(
        mut
    )]
    pub signer_token_ata: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        seeds = [constants::STAKE_VAULT, cnft.key.as_ref()],
        bump,
        payer = signer,
        token::mint = tx_token_mint,
        token::authority = cnft_stake_vault
    )]
    pub cnft_stake_vault: Account<'info, TokenAccount>,

    #[account(
        seeds = [TokenWhitelist::SEED.as_bytes()],
        bump
    )]
    pub whitelist: Account<'info, TokenWhitelist>,

    pub tx_token_mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>
}

pub fn lock_fund(ctx: Context<LockFund>) -> Result<()> {
    require!(ctx.accounts.whitelist.tokens.contains(&ctx.accounts.tx_token_mint.key()), Errors::TokenAlreadyWhitelisted);
    let asset = Asset::from_account_info(&ctx.accounts.asset_info);
    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.signer_token_ata.to_account_info(),
                to: ctx.accounts.cnft_stake_vault.to_account_info(),
                authority: ctx.accounts.signer.to_account_info()
            }
        ),
        asset.price
    )
}