use ticketland_signdrop::{
	config::Config,
	airdroper::Airdroper,
};

#[tokio::main]
async fn main() {
	let config = Config::new().unwrap();

	let mut aidroper = Airdroper::new(
		config.solana_rpc,
    config.fund_priv_key,
    config.rabbitmq_uri,
    config.airdrop_amount,
	).await;

	aidroper.start().await;
}
