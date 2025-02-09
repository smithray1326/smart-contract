use anchor_lang::prelude::*;
use mpl_bubblegum::instructions::MintToCollectionV1CpiBuilder;
use mpl_bubblegum::types::{Collection, MetadataArgs, TokenProgramVersion, TokenStandard};
use spl_account_compression::{program::SplAccountCompression, Noop};
use crate::{MplBubblegum};
use anchor_spl::{
    metadata::{Metadata}
};



#[derive(Accounts)]
pub struct MintCNFT<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: This account is checked in the instruction
    #[account(mut)]
    pub tree_config: UncheckedAccount<'info>,
    /// CHECK: This account is neither written to nor read from.
    pub leaf_owner: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: unsafe
    pub merkle_tree: UncheckedAccount<'info>,
    pub tree_delegate: Signer<'info>,

    #[account(
        seeds = [b"tree_owner", merkle_tree.key().as_ref()],
        bump
    )]
         /// CHECK: unsafe
    pub central_authority: UncheckedAccount<'info>,
    /// CHECK: This account is checked in the instruction
    pub collection_mint: UncheckedAccount<'info>,
    /// CHECK:
    #[account(mut)]
    pub collection_metadata: UncheckedAccount<'info>,

    /// CHECK: This account is checked in the instruction
    pub edition_account: UncheckedAccount<'info>,

    /// CHECK: This is just used as a signing PDA.
    pub bubblegum_signer: UncheckedAccount<'info>,
    pub log_wrapper: Program<'info, Noop>,
    pub compression_program: Program<'info, SplAccountCompression>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub bubblegum_program: Program<'info, MplBubblegum>,
    pub system_program: Program<'info, System>,
}

pub fn mint_cnft(ctx: Context<MintCNFT>, name: String,symbol: String, uri: String,seller_fee_basis_points: u16) -> Result<()> {
    msg!("<<<<<minting nft");
    // require!(ctx.accounts.central_authority.merkle_tree_address.is_some(), MyError::InvalidMerkleTree);
    // require_keys_eq!(*ctx.accounts.merkle_tree.key, ctx.accounts.central_authority.merkle_tree_address.unwrap(), MyError::InvalidMerkleTree);
    // require_keys_eq!(*ctx.accounts.collection_mint.key, ctx.accounts.central_authority.collection_address, MyError::InvalidMerkleTree);
  
    MintToCollectionV1CpiBuilder::new(
        &ctx.accounts.bubblegum_program.to_account_info(),
    )
        .tree_config(&ctx.accounts.tree_config.to_account_info())
        .leaf_owner(&ctx.accounts.leaf_owner.to_account_info())
        .leaf_delegate(&ctx.accounts.leaf_owner.to_account_info())
        .merkle_tree(&ctx.accounts.merkle_tree.to_account_info())
        .payer(&ctx.accounts.payer.to_account_info())
        .tree_creator_or_delegate(&ctx.accounts.central_authority.to_account_info())
        .collection_authority(&ctx.accounts.central_authority.to_account_info())
        .collection_authority_record_pda(Some(&ctx.accounts.bubblegum_program.to_account_info()))
        .collection_mint(&ctx.accounts.collection_mint.to_account_info())
        .collection_metadata(&ctx.accounts.collection_metadata.to_account_info())
        .collection_edition(&ctx.accounts.edition_account.to_account_info())
        .bubblegum_signer(&ctx.accounts.bubblegum_signer.to_account_info())
        .log_wrapper(&ctx.accounts.log_wrapper.to_account_info())
        .compression_program(&ctx.accounts.compression_program.to_account_info())
        .token_metadata_program(&ctx.accounts.token_metadata_program.to_account_info())
        .system_program(&ctx.accounts.system_program.to_account_info())
        .metadata(
            MetadataArgs {
                name,
                symbol,
                uri,
                creators: vec![],
                seller_fee_basis_points,
                primary_sale_happened: false,
                is_mutable: false,
                edition_nonce: Some(0),
                uses: None,
                collection: Some(Collection {
                    verified: true,
                    key: ctx.accounts.collection_mint.key(),
                }),
                token_program_version: TokenProgramVersion::Original,
                token_standard: Some(TokenStandard::NonFungible),
            }
        )
        .invoke_signed(&[&[
            b"tree_owner",
            ctx.accounts.merkle_tree.key().as_ref(),
            &[ctx.bumps.central_authority]
        ]])?;
    Ok(())
}