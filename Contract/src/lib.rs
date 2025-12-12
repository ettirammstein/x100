use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::near_bindgen;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    value: u64,
}

impl Default for Contract {
    fn default() -> Self {
        Self { value: 0 }
    }
}

#[near_bindgen]
impl Contract {
    pub fn get_value(&self) -> u64 {
        self.value
    }

    pub fn set_value(&mut self, value: u64) {
        self.value = value;
    }
}
