use anchor_lang::prelude::*;

#[account]
pub struct NFTBadge {
    pub owner: Pubkey,              // User who earned the badge
    pub goal_id: String,            // Associated goal
    pub milestone: u32,             // E.g., 30-day streak
    pub mint: Pubkey,               // SPL token mint address for NFT
}