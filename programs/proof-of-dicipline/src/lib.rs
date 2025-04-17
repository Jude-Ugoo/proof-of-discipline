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
}
