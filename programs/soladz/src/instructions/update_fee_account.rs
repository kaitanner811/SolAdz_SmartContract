use anchor_lang::prelude::*;
use crate::{ AppStats, APP_STATS_SEED };

#[derive(Accounts)]
pub struct UpdateFeeAccount<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [APP_STATS_SEED],
        bump,
        has_one = owner,
    )]
    pub app_stats: Box<Account<'info, AppStats>>,

    pub fee_account: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn update_fee_account_handler(ctx: Context<UpdateFeeAccount>) -> Result<()> {
    let app_stats: &mut Box<Account<'_, AppStats>> = &mut ctx.accounts.app_stats;
    app_stats.fee_account = ctx.accounts.fee_account.key();
    Ok(())
}
