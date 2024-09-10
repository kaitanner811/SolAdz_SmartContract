use anchor_lang::prelude::*;
use crate::{ Admins, AppStats, ADMIN_SEED, APP_STATS_SEED, MAX_ADMINS, error::ErrorCode };

#[derive(Accounts)]
pub struct AddAdmin<'info> {
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
        space = 32 * 10 + 8,
        seeds = [ADMIN_SEED],
        bump
    )]
    pub admins: Box<Account<'info, Admins>>,
    pub system_program: Program<'info, System>,
}

pub fn add_admin_handler(ctx: Context<AddAdmin>) -> Result<()> {
    let admins: &mut Box<Account<'_, Admins>> = &mut ctx.accounts.admins;
    if admins.admin_accounts.len() == MAX_ADMINS {
        return err!(ErrorCode::OverflowMaxAdmin);
    }
    admins.admin_accounts.push(ctx.accounts.admin.key());
    Ok(())
}
