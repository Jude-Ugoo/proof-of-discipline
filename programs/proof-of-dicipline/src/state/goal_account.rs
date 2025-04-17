use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct GoalAccount {
    pub owner: Pubkey, // User's wallet address
    #[max_len(50)]
    pub goal_id: String, // Unique goal ID (e.g., "30daycode_123")
    #[max_len(200)]
    pub description: String, // Goal description (e.g., "Code 1 hour daily")
    pub stake_amount: u64, // SOL staked (in lamports)
    pub start_time: i64, // Unix timestamp for goal start
    pub duration_days: u32, // Goal duration (e.g., 30 days)
    #[max_len(30)]
    pub check_ins: Vec<i64>, // Timestamps of successful check-ins
    pub is_active: bool, // Goal status
    pub last_check_in: i64, // Last check-in timestamp
    pub total_reward_pool: u64, // Accumulated SOL from failed users
}
