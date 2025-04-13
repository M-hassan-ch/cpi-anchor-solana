use anchor_lang::prelude::*;
use program_b::program::ProgramB;

declare_id!("88NkftFqLEDTVHQm3Z6xW6THd8wiNBoLzM7LeVB5iq94");

#[program]
pub mod program_a {
    use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

    use super::*;

    pub fn cpi_system_program(ctx: Context<SystemProgramCpi>) -> Result<()> {
        msg!(
            "Greetings from Program A with signer: {:?}",
            ctx.accounts.signer.key()
        );

        let pda_account = &mut ctx.accounts.pda_account;
        let signer_key = &ctx.accounts.signer.key();

        let instruction =
            &system_instruction::transfer(&pda_account.key(), &signer_key, 1_000_000_000);

        let account_infos = &[
            pda_account.to_account_info(),
            ctx.accounts.signer.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ];

        let signers_seeds: &[&[&[u8]]] =
            &[&[b"pda", signer_key.as_ref(), &[ctx.bumps.pda_account]]];

        let _ = invoke_signed(instruction, account_infos, signers_seeds);

        Ok(())
    }

    pub fn cpi_custom_program(ctx: Context<CustomProgramCpi>) -> Result<()> {
        msg!(
            "Greetings from Program A with signer: {:?}",
            ctx.accounts.signer.key()
        );

        let pda_account = &mut ctx.accounts.pda_account;
        let signer_key = &ctx.accounts.signer.key();

        let signer_seeds: &[&[&[u8]]] =
            &[&[b"pda", signer_key.as_ref(), &[ctx.bumps.pda_account]]];

        let cpi_context = CpiContext::new_with_signer(
            ctx.accounts.program_b.to_account_info(),
            program_b::cpi::accounts::Hello{signer: pda_account.to_account_info()},
            signer_seeds,
        );

        let _ = program_b::cpi::hello(cpi_context);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SystemProgramCpi<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    /// CHECK: This is a PDA account
    #[account(
        mut,
        seeds = [b"pda", signer.key().as_ref()],
        bump
    )]
    pub pda_account: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CustomProgramCpi<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    /// CHECK: This is a PDA account
    #[account(
        mut,
        seeds = [b"pda", signer.key().as_ref()],
        bump
    )]
    pub pda_account: AccountInfo<'info>,
    pub program_b: Program<'info, ProgramB>,
    pub system_program: Program<'info, System>,
}
