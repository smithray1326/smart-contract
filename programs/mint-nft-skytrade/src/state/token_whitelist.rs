use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_spl::token::Mint;
use crate::errors::Errors;

#[account]
pub struct TokenWhitelist {
    pub tokens: Vec<Pubkey>
}

impl TokenWhitelist {
    pub const SEED: &'static str = "token_whitelist";
    pub const SIZE: usize = std::mem::size_of::<TokenWhitelist>();
}

pub trait WhitelistTokenAccount<'info> {
    fn insert_token(&mut self,
                    token: &Account<'info, Mint>,
                    payer: &Signer<'info>,
                    system_program: &Program<'info, System>) -> Result<()>;
    fn remove_token(&mut self, token: &Account<'info, Mint>) -> Result<()>;

    fn check_token_exist(&mut self, token: &Account<'info, Mint>) -> Result<()>;
    fn realloc(&mut self,
               space: usize,
               payer: &Signer<'info>,
               system: &Program<'info, System>) -> Result<()>;
}
impl<'info> WhitelistTokenAccount<'info> for Account<'info, TokenWhitelist> {
    fn insert_token(&mut self,
                    token: &Account<'info, Mint>,
                    payer: &Signer<'info>,
                    system_program: &Program<'info, System>) -> Result<()> {
        match self.check_token_exist(token) {
            Ok(_) => {
                Err(Errors::TokenAlreadyWhitelisted.into())
            }
            Err(_) => {
                let space = std::mem::size_of::<Pubkey>();
                self.realloc(space, payer, system_program)?;
                self.tokens.push(token.key());
                Ok(())
            }
        }
    }

    fn remove_token(&mut self, token: &Account<'info, Mint>) -> Result<()> {
        match self.check_token_exist(token) {
            Ok(_) => {
                self.tokens.retain(|key| key != &token.key());
                Ok(())
            }
            Err(error) => {
                Err(error)
            }
        }
    }

    fn check_token_exist(&mut self, token: &Account<'info, Mint>) -> Result<()> {
        if self.tokens.contains(&token.key()) {
            Ok(())
        } else {
            Err(Errors::InSufficientSol.into())
        }
    }

    fn realloc(
        &mut self,
        space: usize,
        payer: &Signer<'info>,
        system_program: &Program<'info, System>
    ) -> Result<()> {
        let account_info = self.to_account_info();
        let new_account_size = account_info.data_len() + space;
        let lamport_required = (Rent::get())?.minimum_balance(new_account_size);
        let additional_rent_to_pay = lamport_required - account_info.lamports();
        transfer_lamports(
            payer,
            account_info.clone(),
            additional_rent_to_pay,
            system_program,
        )?;
        account_info.realloc(new_account_size, false)?;
        Ok(())
    }
}

fn transfer_lamports<'info>(
    from: &Signer<'info>,
    to: AccountInfo<'info>,
    amount: u64,
    system_program: &Program<'info, System>,
) -> Result<()> {
    system_program::transfer(
        CpiContext::new(
            system_program.to_account_info(),
            system_program::Transfer {
                from: from.to_account_info(),
                to,
            },
        ),
        amount,
    )
}