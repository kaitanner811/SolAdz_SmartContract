use anchor_lang::prelude::*;

#[account]
pub struct Admins {
  pub admin_accounts: Vec<Pubkey>,
}