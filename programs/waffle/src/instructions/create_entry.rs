use anchor_lang::prelude::*;
use crate::state::{
    Entry,
};
use crate::errors::ErrorCode;

pub fn create_entry(ctx: Context<CreateEntry>) -> Result<()> {
    let entry = &mut ctx.accounts.entry;

    entry.owner = ctx.accounts.payer.key();
    entry.bump = *ctx.bumps.get("entry").unwrap();
    Ok(())
}

#[derive(Accounts)]
pub struct CreateEntry<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK:
    #[account(mut,
        constraint = raffle.to_account_info().owner == program_id @ ErrorCode::IncorrectOwner,
    )]
    pub raffle: UncheckedAccount<'info>,
    #[account(
        init,
        seeds = [
            "entry".as_bytes(),
            payer.key().as_ref(),
            raffle.key().as_ref()
        ],
        bump,
        payer = payer,
        space = 8 + 32 + 4 + 1
    )]
    pub entry: Account<'info, Entry>,
    pub system_program: Program<'info, System>,
}