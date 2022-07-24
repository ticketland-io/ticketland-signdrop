use std::env;

pub struct Config {
  pub solana_rpc: String,
  pub fund_priv_key: String,
  pub rabbitmq_uri: String,
  pub airdrop_amount: u64,
}

impl Config {
  pub fn new() -> Result<Self, env::VarError> {
    Result::Ok(
      Self {
        solana_rpc: env::var("SOLANA_RPC").unwrap(),
        fund_priv_key: env::var("FUND_PRIV_KEY").unwrap(),
        rabbitmq_uri: env::var("RABBITMQ_URI").unwrap(),
        airdrop_amount: env::var("AIRDROP_AMOUNT").unwrap().parse::<u64>().unwrap(),
      }
    )
  }
}
