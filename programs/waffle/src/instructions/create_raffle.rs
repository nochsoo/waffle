use anchor_lang::prelude::*;
use crate::state::Raffle;
use crate::errors::ErrorCode;

pub fn create_raffle(ctx: Context<CreateRaffle>) -> Result<()> {
    let creator = &mut ctx.accounts.creator;
    let raffle = &mut ctx.accounts.raffle.load_init()?;

    raffle.creator = creator.key();
    Ok(())
}

#[derive(Accounts)]
pub struct CreateRaffle<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(zero,
        constraint = raffle.to_account_info().owner == program_id @ ErrorCode::IncorrectOwner,
        constraint = raffle.to_account_info().data_len() == 8 + std::mem::size_of::<Raffle>() @ ErrorCode::InvalidAccountSize
    )]
    pub raffle: AccountLoader<'info, Raffle>,
}