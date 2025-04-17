use anchor_lang::prelude::*;

pub mod instructions;
pub use instructions::*;

pub mod state;
pub use state::*;

declare_id!("CBahuq4hPsMgdgqqM7f63BC1spvAvANWHhuhmCcz9F3u");

#[program]
pub mod proof_of_dicipline {
    use super::*;

    pub fn initialize_program(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize::initialize(ctx)
    }
}
