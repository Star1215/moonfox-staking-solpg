use anchor_lang::{solana_program::entrypoint::ProgramResult, prelude::*};

use crate::state::StakePool;

pub fn handler(ctx: Context<SetStakePoolLocks>, lock_period: u64) -> ProgramResult {
    let stake_pool = &mut ctx.accounts.stake_pool;
    
    stake_pool.lock_period = lock_period;

    Ok(())
}

#[derive(Accounts)]
pub struct SetStakePoolLocks<'info> {
    #[account(
        mut,
        constraint = admin.key() == stake_pool.creator.key()
    )]
    pub admin: Signer<'info>,

    #[account(mut)]
    pub stake_pool: Account<'info, StakePool>
}