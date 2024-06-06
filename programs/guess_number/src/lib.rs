use anchor_lang::prelude::*;

declare_id!("4GXP1uFLNxwgLjfCf3WbT3stTLjunr1rxepukF77YxG6");

#[program]
pub mod guess_number {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
