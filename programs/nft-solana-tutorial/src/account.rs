use anchor_lang::{
    prelude::{AccountInfo, Program, Signer, System, UncheckedAccount},
    Accounts,
};
use anchor_spl::token::Token;

#[derive(Accounts)]
pub struct MintNFTAccount<'info> {
    // The signer executing the minting process
    #[account(mut)]
    pub signer: Signer<'info>,
    // The mint account used to create the NFT
    #[account(mut)]
    pub mint_account: UncheckedAccount<'info>,
    // The SPL token program
    pub token_program: Program<'info, Token>,
    // The Metaplex metadata account
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    // The Token account holding the minted NFT
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,
    // The token metadata program of Metaplex
    pub token_metadata_program: AccountInfo<'info>,
    // The payer account responsible for transaction fees
    #[account(mut)]
    pub payer: AccountInfo<'info>,
    // The system program of Solana
    pub system_program: Program<'info, System>,
    // The rent account
    pub rent: AccountInfo<'info>,
    // The Metaplex Master Edition account
    #[account(mut)]
    pub master_edition: UncheckedAccount<'info>,
}
