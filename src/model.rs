use borsh::{BorshSerialize, BorshDeserialize};

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
pub struct AirdropData {
  pub sol_address: String,
  pub age: u8,
}
