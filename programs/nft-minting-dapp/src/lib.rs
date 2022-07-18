use anchor_lang::{prelude::*, solana_program::program::invoke, system_program};
use anchor_spl::{associated_token, token};
use mpl_token_metadata::{ID as TOKEN_METADATA_ID, instruction as token_instruction};


declare_id!("2ffVV9CD17CoJvFW2J9QVRMZQnRterYhLLZVioiMWzYS");

#[program]
pub mod nft_minting_dapp {
    use super::*;

    pub fn mint(ctx: Context<Mint>,
        metadata_title: String,
        metadata_symbol: String,
        metadata_uri: String,
    ) -> Result<()> {
        
        msg!("Creating a mint account...");
        msg!("Mint: {}", &ctx.accounts.mint.key());

        system_program::create_account(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                system_program::CreateAccount {
                    from: ctx.accounts.mint_authority.to_account_info(),
                    to: ctx.accounts.mint.to_account_info(),
                },
            ),
            10_000_000,
            82,
            &ctx.accounts.token_program.key(),
        )?;


        msg!("Initializing the mint account...");
        msg!("Mint: {}", &ctx.accounts.mint.key());
        token::initialize_mint(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::InitializeMint {
                    mint: ctx.accounts.mint.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                }
            ),
            0,
            &ctx.accounts.mint_authority.key(),
            Some(&ctx.accounts.mint_authority.key()),
        )?;


        msg!("Creating a token account...");
        msg!("Token Address: {}", &ctx.accounts.token_account.key());

        associated_token::create(
            CpiContext::new(
                ctx.accounts.associated_token_program.to_account_info(),
                associated_token::Create {
                    payer: ctx.accounts.mint_authority.to_account_info(),
                    associated_token: ctx.accounts.token_account.to_account_info(),
                    authority: ctx.accounts.mint_authority.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    token_program: ctx.accounts.token_program.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                }
            ),
        )?;


        msg!("Minting token to the token account...");
        msg!("Mint: {}", &ctx.accounts.mint.to_account_info().key());
        msg!("Token Address: {}", &ctx.accounts.token_account.key());

        token::mint_to(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.token_account.to_account_info(),
                    authority: ctx.accounts.mint_authority.to_account_info(),
                }
            ),
            1
        )?;

        msg!("Creating a metadata account...");
        msg!("Metadata Account Address: {}", &ctx.accounts.metadata.to_account_info().key());
        invoke(
            &token_instruction::create_metadata_accounts_v2(
                TOKEN_METADATA_ID,
                ctx.accounts.metadata.key(),
                ctx.accounts.mint.key(),
                ctx.accounts.mint_authority.key(),
                ctx.accounts.mint_authority.key(),
                ctx.accounts.mint_authority.key(),
                metadata_title,
                metadata_symbol,
                metadata_uri,
                None,
                1,
                true,
                false,
                None,
                None
            ),
            &[
                ctx.accounts.metadata.to_account_info(),
                ctx.accounts.mint.to_account_info(),
                ctx.accounts.token_account.to_account_info(),
                ctx.accounts.mint_authority.to_account_info(),
                ctx.accounts.rent.to_account_info(),
            ]
        )?;


        msg!("Creating a master edition metadata account...");
        msg!("Master Edition Metadata Account Address: {}", &ctx.accounts.master_edition.to_account_info().key);

        invoke(
            &token_instruction::create_master_edition_v3(
                TOKEN_METADATA_ID,
                ctx.accounts.master_edition.key(),
                ctx.accounts.mint.key(),
                ctx.accounts.mint_authority.key(),
                ctx.accounts.mint_authority.key(),
                ctx.accounts.metadata.key(),
                ctx.accounts.mint_authority.key(),
                Some(0),
            ),
            &[
                ctx.accounts.master_edition.to_account_info(),
                ctx.accounts.metadata.to_account_info(),
                ctx.accounts.mint.to_account_info(),
                ctx.accounts.token_account.to_account_info(),
                ctx.accounts.mint_authority.to_account_info(),
                ctx.accounts.rent.to_account_info(),
            ]
        )?;

        msg!("Token Minting process is completed successfully...");
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

    /// CHECK: This program will be checked by Metaplex
    pub token_metadata_program: UncheckedAccount<'info>,
}
