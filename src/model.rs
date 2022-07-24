use borsh::{BorshSerialize, BorshDeserialize};
use solana_sdk::{
  pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
pub struct NewUser {
  pub sol_address: Pubkey,
}
