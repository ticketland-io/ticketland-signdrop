use solana_sdk::{
  system_instruction,
  pubkey::Pubkey,
  signature::{Keypair, Signer, Signature},
  transaction::Transaction,
  instruction::Instruction,
};
use solana_client::{
  rpc_client::RpcClient,
  client_error::Result as ClientResult,
};

pub struct Airdroper {
  rpc_client: RpcClient,
  pub payer: Keypair,
}

impl  Airdroper {
  pub fn new(
    solana_rpc: String,
    payer_key: String,
  ) -> Self {
    let rpc_client = RpcClient::new(solana_rpc);
    let payer = Keypair::from_base58_string(&payer_key);

    Self {
      rpc_client,
      payer,
    }
  }
  
  async fn process_transaction(
    &mut self,
    instructions: &[Instruction],
  ) -> ClientResult<Signature> {
    let tx = Transaction::new_with_payer(instructions, Some(&self.payer.pubkey()));

    self.rpc_client.send_and_confirm_transaction(&tx)
  }
  
  pub async fn transfer_sol(&mut self, to_account: &Pubkey, lamports: u64) {
    let transfer_ix = system_instruction::transfer(
      &self.payer.pubkey(),
      to_account,
      lamports
    );
  
    self.process_transaction(&[transfer_ix])
      .await
      .unwrap();
  } 
}

