use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Goal is not active")]
    GoalInactive,
    #[msg("Goal has expired")]
    GoalExpired,
    #[msg("Already checked in today")]
    AlreadyCheckedIn,
    #[msg("Goal is not complete")]
    GoalNotComplete,
    #[msg("Proof is already validated")]
    ProofAlreadyValidated,
    #[msg("Validator has already voted")]
    AlreadyVoted,
    #[msg("Challenge is not active")]
    ChallengeInactive,
    #[msg("Participant is not active")]
    ParticipantInactive,
    #[msg("Challenge is not complete")]
    ChallengeNotComplete,
    #[msg("Invalid stake amount")]
    InvalidStakeAmount,
}