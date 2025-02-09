use anchor_lang::prelude::*;

#[account]
pub struct Asset {
    pub id: Pubkey,
    pub name: String,
    pub metadata_url: String,
    pub price: u64,
    pub last_updated: u64,
    pub reputation: Reputation,
    pub authority: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub enum Reputation {
    Low,
    Medium,
    High,
}

impl Asset {
    pub fn from_account_info(account_info: &AccountInfo) -> Self {
        let asset_info_data = &mut &**account_info.try_borrow_data().unwrap();
        let asset = Asset::try_deserialize(asset_info_data).unwrap();
         asset
    }
}