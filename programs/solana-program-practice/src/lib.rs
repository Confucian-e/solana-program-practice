use anchor_lang::prelude::*;

declare_id!("9u81MKiqMR7oRyM5M9v666eKoWqEMRiR9bF5amjmyEug");

#[program]
pub mod solana_program_practice {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
