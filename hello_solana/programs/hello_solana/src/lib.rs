use anchor_lang::prelude::*;

declare_id!("7qLbpVFruV2pnpCrDa7tym6ba6DmT6vExoK5XGqxJc2E");

#[program]
pub mod hello_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("hello solana");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
