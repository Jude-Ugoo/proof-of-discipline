use anchor_lang::prelude::*;

use crate::ProgramState;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = admin,
        space = 8 + ProgramState::INIT_SPACE,
    )]
    pub program_state: Account<'info, ProgramState>,

    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_program(ctx: Context<Initialize>) -> Result<()> {
    let program_state = &mut ctx.accounts.program_state;

    program_state.admin = ctx.accounts.admin.key();
    program_state.total_goals = 0;
    program_state.fee_percentage = 1;

    Ok(())
}
