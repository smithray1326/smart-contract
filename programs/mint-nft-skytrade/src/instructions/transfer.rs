
use anchor_lang::prelude::*;
use spl_account_compression::{program::SplAccountCompression, Noop};
use crate::{MplBubblegum};
use solana_program::program::invoke_signed;
use solana_program::instruction::Instruction;


const TRANSFER_DISCRIMINATOR: &[u8; 8] = &[163, 52, 200, 231, 140, 3, 69, 186];

pub fn transfer_cnft<'info>(
    ctx: Context<'_, '_, '_, 'info, Transfer<'info>>,
    root: [u8; 32],
    data_hash: [u8; 32],
    creator_hash: [u8; 32],
    nonce: u64,
    index: u32,
) -> Result<()> {
    msg!(
        "attempting to send nft {} from tree {}",
        index,
        ctx.accounts.merkle_tree.key()
    );

    let mut accounts: Vec<solana_program::instruction::AccountMeta> = vec![
        AccountMeta::new_readonly(ctx.accounts.tree_authority.key(), false),
        AccountMeta::new_readonly(ctx.accounts.leaf_owner.key(), true),
        AccountMeta::new_readonly(ctx.accounts.leaf_owner.key(), false),
        AccountMeta::new_readonly(ctx.accounts.new_leaf_owner.key(), false),
        AccountMeta::new(ctx.accounts.merkle_tree.key(), false),
        AccountMeta::new_readonly(ctx.accounts.log_wrapper.key(), false),
        AccountMeta::new_readonly(ctx.accounts.compression_program.key(), false),
        AccountMeta::new_readonly(ctx.accounts.system_program.key(), false),
    ];

    let mut data: Vec<u8> = vec![];
    data.extend(TRANSFER_DISCRIMINATOR);
    data.extend(root);
    data.extend(data_hash);
    data.extend(creator_hash);
    data.extend(nonce.to_le_bytes());
    data.extend(index.to_le_bytes());

    let mut account_infos: Vec<AccountInfo> = vec![
        ctx.accounts.tree_authority.to_account_info(),
        ctx.accounts.leaf_owner.to_account_info(),
        ctx.accounts.leaf_owner.to_account_info(),
        ctx.accounts.new_leaf_owner.to_account_info(),
        ctx.accounts.merkle_tree.to_account_info(),
        ctx.accounts.log_wrapper.to_account_info(),
        ctx.accounts.compression_program.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
    ];

    // add "accounts" (hashes) that make up the merkle proof
    for acc in ctx.remaining_accounts.iter() {
        accounts.push(AccountMeta::new_readonly(acc.key(), false));
        account_infos.push(acc.to_account_info());
    }

    msg!("manual cpi call");
     invoke_signed(
      &Instruction {
            program_id: ctx.accounts.bubblegum_program.key(),
            accounts,
            data,
        },
        &account_infos[..],
        &[&[
            ctx.accounts.merkle_tree.key().as_ref(),
            &[ctx.bumps.tree_authority]
        ]],
    )
    .map_err(Into::into)
}

#[derive(Accounts)]
pub struct Transfer<'info> {
    #[account(
        seeds = [merkle_tree.key().as_ref()],
        bump,
        seeds::program = bubblegum_program.key(),
    )]
         /// CHECK: unsafe
    pub tree_authority: UncheckedAccount<'info>,
    #[account(mut)] // Add mut here to fix the payer error
    /// CHECK: This account is the intended recipient of the NFT transfer
    pub leaf_owner:Signer<'info>, // sender (the vault in our case)
    /// CHECK: This account is neither written to nor read from.
    pub new_leaf_owner: UncheckedAccount<'info>, // receiver
    #[account(mut)]
    /// CHECK: This account is modified in the downstream program
    pub merkle_tree: UncheckedAccount<'info>,
    pub log_wrapper: Program<'info, Noop>,
    pub compression_program: Program<'info, SplAccountCompression>,
    pub bubblegum_program: Program<'info, MplBubblegum>,
    pub system_program: Program<'info, System>,
}