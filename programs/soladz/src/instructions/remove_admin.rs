use anchor_lang::prelude::*;
use std::mem::size_of;
use crate::{ Admins, AppStats, ADMIN_SEED, APP_STATS_SEED };

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
        init_if_needed,
        payer = owner,
        space = size_of::<Admins>() + 8,
        seeds = [ADMIN_SEED],
        bump
    )]
    pub admins: Box<Account<'info, Admins>>,
    pub system_program: Program<'info, System>,
}

pub fn remove_admin_handler(ctx: Context<RemoveAdmin>) -> Result<()> {
  let admins: &mut Box<Account<'_, Admins>> = &mut ctx.accounts.admins;
  let index = admins.admin_accounts.iter().position(|x| *x == ctx.accounts.admin.key()).unwrap();
  admins.admin_accounts.remove(index);
    Ok(())
}
