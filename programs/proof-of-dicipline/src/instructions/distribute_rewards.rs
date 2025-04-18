use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};

use crate::{errors::ErrorCode, GoalAccount};

#[derive(Accounts)]
pub struct DistributeRewards<'info> {
    #[account(
        mut,
        constraint = goal_account.is_active @ ErrorCode::GoalInactive
    )]
    pub goal_account: Account<'info, GoalAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"vault", goal_account.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>
}

pub fn distribute_users_rewards(ctx: Context<DistributeRewards>) -> Result<()> {
    let goal = &mut ctx.accounts.goal_account;
    let current_time = Clock::get()?.unix_timestamp;

    // Ensure goal is complete
    require!(
        current_time >= goal.start_time + (goal.duration_days as i64 * 86400),
        ErrorCode::GoalNotComplete
    );

    // Calculate reward
    let reward = goal.stake_amount + (goal.total_reward_pool / goal.check_ins.len() as u64);
    goal.is_active = false;

    // Secure SOL transfer to user
    let cpi_accounts = Transfer {
        from: ctx.accounts.vault.to_account_info(),
        to: ctx.accounts.user.to_account_info(),
    };
    let cpi_program = ctx.accounts.system_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    transfer(cpi_ctx, reward)?;

    Ok(())
}