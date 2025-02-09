use anchor_lang::prelude::*;
use anchor_spl::token::{close_account, CloseAccount, Token, TokenAccount, transfer, Transfer};
use mpl_bubblegum::instructions::BurnCpiBuilder;
use spl_account_compression::Noop;
use spl_account_compression::program::SplAccountCompression;
use crate::MplBubblegum;
// use crate::state::{StakeInfo, StakeInfoAccount};
use crate::constants::*;

#[derive(Accounts)]
pub struct BurnCNFT<'info> {
    #[account(
        mut,
        address = cnft.owner.key()
    )]
    pub signer: Signer<'info>,

    // Todo: remember to close this account
    #[account(
        mut,
        seeds = [constants::STAKE_VAULT, cnft.key().as_ref()],
        bump,
    )]
    pub cnft_stake_vault: Account<'info, TokenAccount>,

    pub signer_token_account: Account<'info, TokenAccount>,

    /// CHECK: should be vetted from front end
    /// ensure this nft is owned by the signer
    pub cnft: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECKED: this account is checked in the instruction
    pub merkle_tree: UncheckedAccount<'info>,
    /// CHECKED: this account is checked in the instruction
    pub tree_config: UncheckedAccount<'info>,
    pub log_wrapper: Program<'info, Noop>,
    pub compression_program: Program<'info, SplAccountCompression>,
    pub bubblegum_program: Program<'info, MplBubblegum>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>
}

pub fn burn_cnft<'info>(ctx: Context<'_, '_, '_, 'info, BurnCNFT<'info>>,
                        root: [u8; 32],
                        data_hash: [u8; 32],
                        creator_hash: [u8; 32],
                        nonce: u64, index: u32) -> Result<()> {
    //withdraw funds from vault
    let cnft_key = ctx.accounts.cnft_stake_vault.key();
    let seed: &[&[&[u8]]] = &[&[cnft_key.as_ref(), &[ctx.bumps.cnft_stake_vault]]];
    transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.cnft_stake_vault.to_account_info(),
                to: ctx.accounts.signer_token_account.to_account_info(),
                authority: ctx.accounts.cnft_stake_vault.to_account_info()
            },
            seed
        ),
        ctx.accounts.cnft_stake_vault.amount
    )?;

    //close vault account
    close_account(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            CloseAccount {
                account: ctx.accounts.cnft_stake_vault.to_account_info(),
                destination: ctx.accounts.signer_token_account.to_account_info(),
                authority: ctx.accounts.cnft_stake_vault.to_account_info()
            },
            seed
        )
    )?;

    //burn nft
    let remaining_accounts: Vec<(&AccountInfo, bool, bool)> = ctx.remaining_accounts
        .iter()
        .map(|account| (account, account.is_signer, account.is_writable))
        .collect();

    BurnCpiBuilder::new(&ctx.accounts.bubblegum_program.to_account_info())
        .tree_config(&ctx.accounts.tree_config.to_account_info())
        .merkle_tree(&ctx.accounts.merkle_tree.to_account_info())
        .leaf_owner(&ctx.accounts.signer.to_account_info(), true)
        .leaf_delegate(&ctx.accounts.signer.to_account_info(), true)
        .log_wrapper(&ctx.accounts.log_wrapper.to_account_info())
        .compression_program(&ctx.accounts.compression_program.to_account_info())
        .system_program(&ctx.accounts.system_program.to_account_info())
        .add_remaining_accounts(&remaining_accounts)
        .root(root)
        .data_hash(data_hash)
        .creator_hash(creator_hash)
        .nonce(nonce)
        .index(index)
        .invoke()?;

    Ok(())
}