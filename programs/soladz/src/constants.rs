use anchor_lang::prelude::*;

#[constant]
pub const SEED: &str = "anchor";

pub const APP_STATS_SEED: &[u8] = b"app-stats";
pub const INVESTOR_SEED: &[u8] = b"investor";
pub const VAULT_SEED: &[u8] = b"vault";
pub const ADMIN_SEED: &[u8] = b"admin";
pub const SEVER_SIGNER: Pubkey = pubkey!("Gaj7cGbQ3CCWkqn8QsnLXEVaBaTN98GRxkX1pPsC4yNS");