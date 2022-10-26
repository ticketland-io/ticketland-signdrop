use std::sync::Arc;
use eyre::Result;
use solana_sdk::{
  system_transaction,
  pubkey::Pubkey,
  signature::{Keypair, Signature},
};
use solana_client::{
  nonblocking::rpc_client::RpcClient,
  client_error::Result as ClientResult,
};
use borsh::BorshDeserialize;
use lapin::{
  Result as LapinResult,
  message::{Delivery},
  options::{BasicAckOptions, BasicNackOptions},
};
use amqp_helpers::{
  consumer::retry_consumer::RetryConsumer,
};
use crate::model::NewUser;

pub struct Airdroper {
  rpc_client: Arc<RpcClient>,
  payer: Arc<Keypair>,
  retry_consumer: RetryConsumer,
  airdrop_amount: u64,
}

impl Airdroper {
  pub async fn new(
    solana_rpc: String,
    fund_priv_key: String,
    rabbitmq_uri: String,
    airdrop_amount: u64,
  ) -> Self {
    let rpc_client = Arc::new(RpcClient::new(solana_rpc));
    let payer = Arc::new(Keypair::from_base58_string(&fund_priv_key));
    let retry_consumer = RetryConsumer::new(
      &rabbitmq_uri,
      "user",
      "user",
    ).await;

    Self {
      rpc_client,
      payer,
      retry_consumer,
      airdrop_amount,
    }
  }
  
  pub async fn start(&mut self) -> Result<()> {
    println!("Running...");

    let airdrop_amount = self.airdrop_amount;
    let rpc_client = Arc::clone(&self.rpc_client);
    let payer = Arc::clone(&self.payer);

    self.retry_consumer.consume(Box::new(move |delivery: LapinResult<Delivery>, _| {
      let rpc_client = Arc::clone(&rpc_client);
      let payer = Arc::clone(&payer);

      async move {  
        if let Ok(delivery) = delivery {
          let user = NewUser::try_from_slice(&delivery.data).unwrap();
          let sol_address = user.sol_address.as_str().try_into().unwrap();

          match Self::transfer_sol(rpc_client, payer, &sol_address, airdrop_amount).await {
            Ok(tx_hash) => {
              println!("Airdrop success {} for {} SOL txHash {}", user.sol_address, airdrop_amount, tx_hash);
  
              delivery
              .ack(BasicAckOptions::default())
              .await
              .expect("Failed to ack send_webhook_event message");
            },
            Err(error) => {
              println!("Airdrop to {} for {} SOL failed {}", user.sol_address, airdrop_amount, error);
  
              delivery
              .nack(BasicNackOptions::default())
              .await
              .expect("Failed to ack send_webhook_event message");
            }
          }
        }
      }
    })).await?;

    Ok(())
  }

  pub async fn transfer_sol(
    rpc_client: Arc<RpcClient>,
    payer: Arc<Keypair>,
    to_account: &Pubkey,
    lamports: u64
  ) -> ClientResult<Signature> {
    println!("Transferring {} SOL to {}", lamports, to_account);

    rpc_client.send_and_confirm_transaction(
      &system_transaction::transfer(
        &payer,
        to_account,
        lamports,
        rpc_client.get_latest_blockhash().await?
      )
    ).await
  } 
}

