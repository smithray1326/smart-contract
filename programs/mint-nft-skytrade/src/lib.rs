mod instructions;
mod state;
mod constants;
mod errors;

use anchor_lang::prelude::*;
use crate::instructions::*;

declare_id!("6J3F9s5R11gAg6CQUjYfTKDi2WStJ73nmrcfKfWTYWKY");

#[derive(Clone)]
pub struct MplBubblegum;
impl Id for MplBubblegum {
    fn id() -> Pubkey {
        mpl_bubblegum::ID
    }
}

#[program]
pub mod mint_nft_skytrade {
    use super::*;

    pub fn init(ctx: Context<Init>) -> Result<()> {
        instructions::init(ctx)
    }

    pub fn create_tree(ctx: Context<CreateTree>, max_depth: u32, max_buffer_size: u32) -> Result<()> {
        instructions::create_tree(ctx, max_depth, max_buffer_size)
    }

    pub fn transfer_nft<'info>(ctx: Context<'_, '_, '_, 'info, Transfer<'info>>,
    root: [u8; 32],
    data_hash: [u8; 32],
    creator_hash: [u8; 32],
    nonce: u64,
    index: u32) -> Result<()> {
        instructions::transfer_cnft(ctx, root, data_hash,creator_hash,nonce,index)
    }

    pub fn whitelist_token(ctx: Context<WhitelistToken>) -> Result<()> {
        instructions::whitelist_token(ctx)
    }

    pub fn delist_token(ctx: Context<DelistToken>) -> Result<()> {
        instructions::delist_token(ctx)
    }

    pub fn mint_cnft(ctx: Context<MintCNFT>,name: String,symbol: String, uri: String,seller_fee_basis_points: u16) -> Result<()> {
        instructions::mint_cnft(ctx,name, symbol,uri,seller_fee_basis_points,)
    }

    pub fn burn_cnft<'info>(ctx: Context<'_, '_, '_, 'info, BurnCNFT<'info>>,
                     root: [u8; 32],
                     data_hash: [u8; 32],
                     creator_hash: [u8; 32],
                     nonce: u64,
                     index: u32) -> Result<()> {
        instructions::burn_cnft(ctx, root, data_hash, creator_hash, nonce, index)
    }

    pub fn lock_fund(ctx: Context<LockFund>) -> Result<()> {
        instructions::lock_fund(ctx)
    }
}