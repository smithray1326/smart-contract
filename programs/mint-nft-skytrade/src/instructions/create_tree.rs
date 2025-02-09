use anchor_lang::prelude::*;
use mpl_bubblegum::instructions::{CreateTreeConfigCpiBuilder};
use spl_account_compression::{program::SplAccountCompression, Noop};
use crate::{MplBubblegum};

pub fn create_tree(ctx: Context<CreateTree>, max_depth: u32, max_buffer_size: u32) -> Result<()> {
    CreateTreeConfigCpiBuilder::new(&ctx.accounts.mpl_bubblegum_program)
        .tree_config(&ctx.accounts.tree_config)
        .merkle_tree(&ctx.accounts.merkle_tree)
        .payer(&ctx.accounts.signer)
        .tree_creator(&ctx.accounts.tree_owner)
        .log_wrapper(&ctx.accounts.log_wrapper)
        .compression_program(&ctx.accounts.compression_program)
        .system_program(&ctx.accounts.system_program)
        .max_depth(max_depth)
        .max_buffer_size(max_buffer_size)
        .invoke_signed(&[&[
            b"tree_owner",
            ctx.accounts.merkle_tree.key().as_ref(),
            &[ctx.bumps.tree_owner]
        ]])?;

    Ok(())
}

#[derive(Accounts)]
pub struct CreateTree<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    /// CHECK: This account is modified in the downstream program
    pub tree_config: UncheckedAccount<'info>,

    #[account(zero)]
    /// CHECK: This account must be all zeros
    pub merkle_tree: UncheckedAccount<'info>,

    #[account(
        seeds = [b"tree_owner", merkle_tree.key().as_ref()],
        bump
    )]
    /// CHECK: This account used as a signing PDA only
    pub tree_owner: UncheckedAccount<'info>,
    pub mpl_bubblegum_program: Program<'info, MplBubblegum>,
    pub log_wrapper: Program<'info, Noop>,
    pub compression_program: Program<'info, SplAccountCompression>,
    pub system_program: Program<'info, System>
}