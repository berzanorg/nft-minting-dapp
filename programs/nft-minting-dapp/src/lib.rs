use anchor_lang::{prelude::*, solana_program::program::invoke, system_program};
use anchor_spl::{associated_token, mint, token};
use mpl_token_metadata::{ID as TOKEN_METADATA_ID, instruction as token_instruction};


declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod nft_minting_dapp {
    use super::*;

    pub fn mint(ctx: Context<Mint>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Mint<'info> {
    /// CHECK: This account will be created with Metaplex
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,

    /// CHECK: This account will also be created with Metaplex
    #[account(mut)]
    pub master_edition: UncheckedAccount<'info>,

    #[account(mut)]
    pub mint: Signer<'info>,

    /// CHECK: This account will be created with Anchor
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,

    #[account(mut)]
    pub mint_authority: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,

    // CHECK: This program will be checked by Metaplex
    pub token_metadata_program: UncheckedAccount<'info>,
}
