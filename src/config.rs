use std::env;

pub struct Config {
  pub solana_rpc: String,
}

impl Config {
  pub fn new() -> Result<Self, env::VarError> {
    Result::Ok(
      Self {
        solana_rpc: env::var("SOLANA_RPC").unwrap(),
      }
    )
  }
}
