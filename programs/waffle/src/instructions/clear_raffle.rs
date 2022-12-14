use anchor_lang::prelude::*;
use crate::state::Raffle;
use crate::errors::ErrorCode;

// ref: https://en.wikipedia.org/wiki/Lehmer_random_number_generator
// ref: https://en.wikipedia.org/wiki/Fisher%E2%80%93Yates_shuffle
pub fn clear_raffle(ctx: Context<ClearRaffle>, num_shuffles: u32) -> Result<()> {
    let raffle = &mut ctx.accounts.raffle.load_mut()?;
    if raffle.creator != ctx.accounts.creator.key() {
        return err!(ErrorCode::Unauthorized);
    }
    require!(raffle.shuffled_count != raffle.entries_count, ErrorCode::RaffleAlreadyCleared);
    require!(num_shuffles <= raffle.entries_count - raffle.shuffled_count, ErrorCode::ExcessShuffle);

    let mut j: usize;
    for i in (0..(raffle.entries_count - raffle.shuffled_count) as usize).rev().take(num_shuffles as usize) {
        raffle.entropy = raffle.entropy.overflowing_mul(48271).0 % 0x7fffffff;
        j = raffle.entropy as usize % (i + 1);
        raffle.entries.swap(j, i);
    }
    raffle.shuffled_count += num_shuffles;
    Ok(())
}

#[derive(Accounts)]
pub struct ClearRaffle<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(mut,
        constraint = raffle.to_account_info().owner == program_id @ ErrorCode::IncorrectOwner
    )]
    pub raffle: AccountLoader<'info, Raffle>,
}