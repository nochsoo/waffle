use anchor_lang::prelude::*;
use crate::state::Raffle;
use crate::errors::ErrorCode;

pub fn close_raffle(ctx: Context<CloseRaffle>) -> Result<()> {
    let raffle = &mut ctx.accounts.raffle.load_mut()?;
    if raffle.creator != ctx.accounts.creator.key() {
        return err!(ErrorCode::Unauthorized);
    }
    Ok(())
}

#[derive(Accounts)]
pub struct CloseRaffle<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(mut,
        constraint = raffle.to_account_info().owner == program_id @ ErrorCode::IncorrectOwner,
        close = creator
    )]
    pub raffle: AccountLoader<'info, Raffle>,
}