use anchor_lang::prelude::*;

pub mod instructions;
pub use instructions::*;

pub mod state;
pub use state::*;

pub mod errors;

declare_id!("CBahuq4hPsMgdgqqM7f63BC1spvAvANWHhuhmCcz9F3u");

#[program]
pub mod proof_of_dicipline {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::initialize_program(ctx)
    }

    pub fn create_goal(
        ctx: Context<CreateGoal>,
        goal_id: String,
        description: String,
        stake_amount: u64,
        duration_days: u32,
    ) -> Result<()> {
        create_goal::create_user_goal(ctx, goal_id, description, stake_amount, duration_days)
    }

    pub fn check_in(ctx: Context<CheckIn>) -> Result<()> {
        check_in::user_check_in(ctx)
    }

    pub fn penalize_missed_check_in(ctx: Context<PenalizeMissedCheckIn>) -> Result<()> {
        penalize_missed_check_in::penalize_missed_user_check_in(ctx)
    }

    // pub fn test_set_last_check_in(ctx: Context<SetLastCheckIn>, timestamp: i64) -> Result<()> {
    //     penalize_missed_check_in::test_set_last_check_in(ctx, timestamp)
    // }

    pub fn distribute_rewards(ctx: Context<DistributeRewards>) -> Result<()> {
        distribute_rewards::distribute_users_rewards(ctx)
    }
}
