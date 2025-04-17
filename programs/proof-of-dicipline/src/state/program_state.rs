use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct ProgramState {
    pub admin: Pubkey,              // Program admin for configuration
    pub total_goals: u64,           // Total active goals
    pub fee_percentage: u8,         // Optional: Program fee (e.g., 1% of stakes)
}