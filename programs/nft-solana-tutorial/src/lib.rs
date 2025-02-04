use anchor_lang::prelude::*;

declare_id!("GMEM4oFc9kZnvWQqocEfNxVyiaGE2yARZcYW2oaucXd7");

#[program]
pub mod nft_solana_tutorial {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
