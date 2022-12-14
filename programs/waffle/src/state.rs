use anchor_lang::prelude::*;
use crate::constants::*;

#[account(zero_copy)]
pub struct Raffle {
    pub creator: Pubkey,
    pub entries_count: u32,
    pub max_entries: u32,
    pub entries: [u32; MAX_ENTRIES],
    pub entropy: u64,
    pub shuffled_count: u32,
    pub ticket_claimed: u32,
}

#[account]
pub struct Entry {
    pub owner: Pubkey,
    pub tickets: Vec<u32>,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default, Debug)]
pub struct RaffleConfig {
    /// amount of fees required for each stake and unstake
    pub process_fee: u64,
    /// processing fees are transferred in this account.
    pub fee_account_address: Option<Pubkey>,
    /// daily rewards per token
    pub rewards_per_token: u64,
    /// lock-in seconds of the deposited token, can't be unstake during this time
    pub token_lock_time: u32,
    pub retain_authority: bool,
    pub is_mutable: bool,
    /// timestamp of last config update
    pub updated_at: i64,
}