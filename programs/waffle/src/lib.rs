mod errors;
mod instructions;
mod state;
mod constants;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("86oWnMivppjGV2CDiESKjfTSuNra1kRgPY82QxNgjZJs");

#[program]
pub mod waffle {
    use super::*;

    pub fn create_raffle(ctx: Context<CreateRaffle>) -> Result<()> {
        instructions::create_raffle(ctx)
    }
    pub fn clear_raffle(ctx: Context<ClearRaffle>, num_raffles: u32) -> Result<()> {
        instructions::clear_raffle(ctx, num_raffles)
    }
    pub fn create_entry(ctx: Context<CreateEntry>) -> Result<()> {
        instructions::create_entry(ctx)
    }
    pub fn enter_raffle(ctx: Context<EnterRaffle>, num_tickets: u32) -> Result<()> {
        instructions::enter_raffle(ctx, num_tickets)
    }
    pub fn close_raffle(ctx: Context<CloseRaffle>) -> Result<()> {
        instructions::close_raffle(ctx)
    }
}

