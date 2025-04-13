use anchor_lang::prelude::*;

declare_id!("88NkftFqLEDTVHQm3Z6xW6THd8wiNBoLzM7LeVB5iq94");

#[program]
pub mod program_a {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
