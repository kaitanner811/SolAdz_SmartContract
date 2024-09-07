use anchor_lang::prelude::*;

use crate::{AppStats, Investor, APP_STATS_SEED, INVESTOR_SEED};

#[derive(Accounts)]
pub struct RemoveAdmin<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [APP_STATS_SEED],
        bump,
        has_one = owner,
    )]
    pub app_stats: Box<Account<'info, AppStats>>,

    pub admin: SystemAccount<'info>,

    #[account(
      mut,
      seeds = [INVESTOR_SEED, admin.key().as_ref()],
      bump
    )]
    pub admin_account: Box<Account<'info, Investor>>,
}


pub fn remove_admin_handler(ctx:Context<RemoveAdmin>) -> Result<()> {
  ctx.accounts.admin_account.is_admin = false;
  Ok(())
}