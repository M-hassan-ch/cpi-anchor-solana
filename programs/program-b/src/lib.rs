use anchor_lang::prelude::*;

declare_id!("76XHb6msPokzjY4EZYMuXvJa3Arjr6BGi1qDUr1g9kYL");

#[program]
pub mod program_b {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
