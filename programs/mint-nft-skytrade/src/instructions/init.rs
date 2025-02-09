use anchor_lang::prelude::*;
use crate::state::TokenWhitelist;

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    #[account(
        init,
        seeds = [TokenWhitelist::SEED.as_bytes()],
        bump,
        payer = signer,
        space = TokenWhitelist::SIZE
    )]
    whitelist: Account<'info, TokenWhitelist>,
    system_program: Program<'info, System>
}
pub fn init(_ctx: Context<Init>) -> Result<()> {Ok(())}