use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};

use crate::{GoalAccount, errors::ErrorCode};

#[derive(Accounts)]
#[instruction(goal_id: String)]
pub struct CreateGoal<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + GoalAccount::INIT_SPACE,
        seeds = [b"goal", goal_id.as_bytes(), user.key().as_ref()],
        bump
    )]
    pub goal_account: Account<'info, GoalAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"vault", goal_account.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,  // PDA for staking SOL

    pub system_program: Program<'info, System>,
}

pub fn create_user_goal(
    ctx: Context<CreateGoal>,
    goal_id: String,
    description: String,
    stake_amount: u64,
    duration_days: u32,
) -> Result<()> {
    require!(stake_amount > 0, ErrorCode::InvalidStakeAmount);

    let goal = &mut ctx.accounts.goal_account;

    goal.owner = ctx.accounts.user.key();
    goal.goal_id = goal_id;
    goal.description = description;
    goal.stake_amount = stake_amount;
    goal.duration_days = duration_days;
    goal.start_time = Clock::get()?.unix_timestamp;
    goal.is_active = true;
    goal.last_check_in = 0;
    goal.total_reward_pool = 0;

    // Transfer SOL to vault
    let cpi_accounts = Transfer {
        from: ctx.accounts.user.to_account_info(),
        to: ctx.accounts.vault.to_account_info(),
    };

    let cpi_program = ctx.accounts.system_program.to_account_info();
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts);

    transfer(cpi_context, stake_amount)?;

    Ok(())
}