use anchor_lang::{prelude::*, solana_program::entrypoint::ProgramResult};
use std::str::FromStr;

use crate::state::{StakePool, RELAYER_ADDRESS};

pub fn handler(ctx: Context<SetRewards>, rewards_per_second: u64) -> ProgramResult {
    let stake_pool = &mut ctx.accounts.stake_pool;

    stake_pool.rewards_per_second = rewards_per_second;

    Ok(())
}

#[derive(Accounts)]
pub struct SetRewards<'info> {
    #[account(
        mut,
        constraint = relayer.key() == Pubkey::from_str(RELAYER_ADDRESS).unwrap()
    )]
    pub relayer: Signer<'info>,

    #[account(mut)]
    pub stake_pool: Account<'info, StakePool>,
}
