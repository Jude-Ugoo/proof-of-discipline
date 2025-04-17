use anchor_lang::prelude::*;

use crate::{errors::ErrorCode, GoalAccount};

#[derive(Accounts)]
pub struct CheckIn<'info> {
    #[account(
        mut,
        has_one = owner,
        constraint = goal_account.is_active @ ErrorCode::GoalInactive
    )]
    pub goal_account: Account<'info, GoalAccount>,

    #[account(mut)]
    pub owner: Signer<'info>,
}

pub fn user_check_in(ctx: Context<CheckIn>) -> Result<()> {
    let goal = &mut ctx.accounts.goal_account;
    let current_time = Clock::get()?.unix_timestamp;

    // Ensure goal is within duration
    require!(
        current_time < goal.start_time + (goal.duration_days as i64 * 86400),
        ErrorCode::GoalExpired
    );

    // Prevent same-day check-in
    let last_check_in = goal.last_check_in;
    let one_day = 86400;
    require!(
        current_time >= last_check_in + one_day,
        ErrorCode::AlreadyCheckedIn
    );

    // Record check-in
    goal.check_ins.push(current_time);
    goal.last_check_in = current_time;

    Ok(())
}