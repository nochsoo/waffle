use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    keccak,
    sysvar
};
use arrayref::array_ref;
use crate::state::{
    Raffle,
    Entry
};
use crate::errors::ErrorCode;

pub fn enter_raffle(ctx: Context<EnterRaffle>, num_tickets: u32) -> Result<()> {
    let clock = Clock::get().unwrap();
    let payer = &mut ctx.accounts.payer;
    let entry = &mut ctx.accounts.entry;
    let raffle = &mut ctx.accounts.raffle.load_mut()?;
    for n in raffle.entries_count..raffle.entries_count + num_tickets {
        raffle.entries[n as usize] = n;
        entry.tickets.push(n);
    }
    let bytes = ctx.accounts.slot_hashes.data.borrow();
    if u64::from_le_bytes(*array_ref![bytes, 0, 8]) == 0 {
        return err!(ErrorCode::InvalidSlotHashAccountData);
    }
    let hash = keccak::hashv(&[
        &raffle.entropy.to_le_bytes(),
        array_ref![bytes, 16, 32],
        &clock.unix_timestamp.to_le_bytes(),
    ]).0;

    raffle.entropy = u64::from_le_bytes(*array_ref![hash, 0, 8]);
    raffle.entries_count += num_tickets;
    Ok(())
}

#[derive(Accounts)]
#[instruction(num_tickets: u32)]
pub struct EnterRaffle<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut,
        constraint = raffle.to_account_info().owner == program_id @ ErrorCode::IncorrectOwner,
    )]
    pub raffle: AccountLoader<'info, Raffle>,
    #[account(
        mut,
        seeds = [
            "entry".as_bytes(),
            payer.key().as_ref(),
            raffle.key().as_ref()
        ],
        bump = entry.bump,
        realloc = entry.to_account_info().data_len() + (num_tickets * 4) as usize,
        realloc::payer = payer,
        realloc::zero = false,
    )]
    pub entry: Account<'info, Entry>,
    /// CHECK:
    #[account(address = sysvar::slot_hashes::ID)]
    pub slot_hashes: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}