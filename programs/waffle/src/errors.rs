use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("You are not authorized to perform this action")]
    Unauthorized,
    #[msg("Account does not have correct owner")]
    IncorrectOwner,

    #[msg("Account size is invalid")]
    InvalidAccountSize,

    #[msg("Raffle has already been cleared")]
    RaffleAlreadyCleared,
    #[msg("Excess indices to shuffle")]
    ExcessShuffle,

    #[msg("Invalid SlotHash account data")]
    InvalidSlotHashAccountData,
}