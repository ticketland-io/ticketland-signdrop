use std::env;

pub struct Config {
  pub solana_rpc: String,
  pub fund_priv_key: String,
}

impl Config {
  pub fn new() -> Result<Self, env::VarError> {
    Result::Ok(
      Self {
        solana_rpc: env::var("SOLANA_RPC").unwrap(),
        fund_priv_key: env::var("FUND_PRIV_KEY").unwrap(),
      }
    )
  }
}
