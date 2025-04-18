use anchor_lang::prelude::*;

use crate::GoalAccount;

#[derive(Accounts)]
pub struct PenalizeMissedCheckIn<'info> {
    #[account(mut)]
    pub goal_account: Account<'info, GoalAccount>,

    #[account(
        mut,
        seeds = [b"vault", goal_account.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,
}

#[derive(Accounts)]
pub struct SetLastCheckIn<'info> {
    #[account(mut)]
    pub goal_account: Account<'info, GoalAccount>,
}

pub fn penalize_missed_user_check_in(ctx: Context<PenalizeMissedCheckIn>) -> Result<()> {
    let goal = &mut ctx.accounts.goal_account;
    let current_time = Clock::get()?.unix_timestamp;

    // Check if user missed check-in
    let one_day = 3;
    if current_time >= goal.last_check_in + one_day {
        goal.total_reward_pool += goal.stake_amount;
        goal.is_active = false;
    }

    Ok(())
}

// pub fn test_set_last_check_in(ctx: Context<SetLastCheckIn>, timestamp: i64) -> Result<()> {
//     let goal = &mut ctx.accounts.goal_account;
//     goal.last_check_in = timestamp;
//     Ok(())
// }