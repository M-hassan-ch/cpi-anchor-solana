use anchor_lang::prelude::*;

declare_id!("76XHb6msPokzjY4EZYMuXvJa3Arjr6BGi1qDUr1g9kYL");

#[program]
pub mod program_b {
    use super::*;

    pub fn hello(ctx: Context<Hello>) -> Result<()> {
        msg!("Greetings from Program B with signer: {:?}", ctx.accounts.signer.key());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Hello<'info> {
    #[account(mut)]
    pub signer: Signer<'info>
}
