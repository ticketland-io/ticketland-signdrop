use std::{env, panic, process};
use ticketland_signdrop::{
	config::Config,
	airdroper::Airdroper,
};

#[tokio::main]
async fn main() {
	let orig_hook = panic::take_hook();
  panic::set_hook(Box::new(move |panic_info| {
    orig_hook(panic_info);
    process::exit(1);
  }));

  if env::var("ENV").unwrap() == "development" {
    dotenv::from_filename(".env").expect("cannot load env from a file");
  }

	let config = Config::new().unwrap();

	let mut aidroper = Airdroper::new(
		config.solana_rpc,
    config.fund_priv_key,
    config.rabbitmq_uri,
    config.airdrop_amount,
	).await;

	aidroper.start().await;
}
