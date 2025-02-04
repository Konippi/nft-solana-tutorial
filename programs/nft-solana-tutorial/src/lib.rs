use anchor_lang::prelude::*;

declare_id!("GMEM4oFc9kZnvWQqocEfNxVyiaGE2yARZcYW2oaucXd7");

#[program]
pub mod nft_solana_tutorial {
    use anchor_spl::{
        metadata::{
            create_master_edition_v3, create_metadata_accounts_v3,
            mpl_token_metadata::types::{Creator, DataV2},
            CreateMasterEditionV3, CreateMetadataAccountsV3,
        },
        token::{self, MintTo, Token},
    };

    use super::*;

    pub fn mint_nft(
        ctx: Context<MintNFTAccount>,
        creator_key: Pubkey,
        uri: String,
        title: String,
    ) -> Result<()> {
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint_account.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.mint_authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::mint_to(cpi_ctx, 1)?;
        msg!("Token minted!");

        let metadata_accounts = CreateMetadataAccountsV3 {
            metadata: ctx.accounts.metadata.to_account_info(),
            mint: ctx.accounts.mint_account.to_account_info(),
            mint_authority: ctx.accounts.mint_authority.to_account_info(),
            payer: ctx.accounts.payer.to_account_info(),
            update_authority: ctx.accounts.mint_authority.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        };
        let creators = vec![
            Creator {
                address: creator_key,
                verified: false,
                share: 100,
            },
            Creator {
                address: ctx.accounts.mint_authority.key(),
                verified: false,
                share: 100,
            },
        ];
        let symbol = "SYMB".to_string();
        let metadata_data = DataV2 {
            name: title,
            symbol,
            uri,
            seller_fee_basis_points: 0,
            creators: Some(creators),
            collection: None,
            uses: None,
        };
        create_metadata_accounts_v3(
            CpiContext::new(
                ctx.accounts.token_metadata_program.to_account_info(),
                metadata_accounts,
            ),
            metadata_data,
            true,
            true,
            None,
        )?;
        msg!("Metadata account created!");

        let master_edition_accounts = CreateMasterEditionV3 {
            edition: ctx.accounts.master_edition.to_account_info(),
            mint: ctx.accounts.mint_account.to_account_info(),
            update_authority: ctx.accounts.mint_authority.to_account_info(),
            mint_authority: ctx.accounts.mint_authority.to_account_info(),
            payer: ctx.accounts.payer.to_account_info(),
            metadata: ctx.accounts.metadata.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        };
        create_master_edition_v3(
            CpiContext::new(
                ctx.accounts.token_metadata_program.to_account_info(),
                master_edition_accounts,
            ),
            Some(0), // Token is one of a kind
        )?;
        msg!("Master edition NFT minted!");

        Ok(())
    }

    #[derive(Accounts)]
    pub struct MintNFTAccount<'info> {
        // The signer executing the minting process
        #[account(mut)]
        pub mint_authority: Signer<'info>,
        // The mint account used to create the NFT
        /// CHECK: This is not dangerous because we don't read or write from this account
        #[account(mut)]
        pub mint_account: UncheckedAccount<'info>,
        // The SPL token program
        pub token_program: Program<'info, Token>,
        // The Metaplex metadata account
        /// CHECK: This is not dangerous because we don't read or write from this account
        #[account(mut)]
        pub metadata: UncheckedAccount<'info>,
        // The Token account holding the minted NFT
        /// CHECK: This is not dangerous because we don't read or write from this account
        #[account(mut)]
        pub token_account: UncheckedAccount<'info>,
        // The token metadata program of Metaplex
        /// CHECK: This is not dangerous because we don't read or write from this account
        pub token_metadata_program: AccountInfo<'info>,
        // The payer account responsible for transaction fees
        /// CHECK: This is not dangerous because we don't read or write from this account
        #[account(mut)]
        pub payer: AccountInfo<'info>,
        // The system program of Solana
        pub system_program: Program<'info, System>,
        // The rent account
        /// CHECK: This is not dangerous because we don't read or write from this account
        pub rent: AccountInfo<'info>,
        // The Metaplex Master Edition account
        /// CHECK: This is not dangerous because we don't read or write from this account
        #[account(mut)]
        pub master_edition: UncheckedAccount<'info>,
    }
}
