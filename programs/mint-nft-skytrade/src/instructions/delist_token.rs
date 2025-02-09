use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use crate::state::{TokenWhitelist, WhitelistTokenAccount};

#[derive(Accounts)]
pub struct DelistToken<'info> {
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [TokenWhitelist::SEED.as_bytes()],
        bump
    )]
    pub whitelist: Account<'info, TokenWhitelist>,
    pub mint: Account<'info, Mint>
}

pub fn delist_token(ctx: Context<DelistToken>) -> Result<()> {
    ctx.accounts.whitelist.remove_token(&ctx.accounts.mint)
}