#![allow(non_upper_case_globals)]

use smart_contract::payload::Parameters;
use smart_contract::debug;
use smart_contract::transaction::{Transaction, Transfer};
use smart_contract_macros::smart_contract;

pub struct Contract;

#[smart_contract]
impl Contract {
    fn init(_params: &mut Parameters) -> Self {
        Self {}
    }

    fn on_money_received(&mut self, params: &mut Parameters) -> Result<(), String> {
        if params.amount < 1500 {
            return Err("A minimum of 1500 PERLs must be sent.".into());
        }
        debug!("Hello world!");
    
        // Create and send transaction.
        Transfer {
            destination: params.sender,
            amount: (params.amount + 1) / 2,
            invocation: None,
        }
        .send_transaction();

        Ok(())
    }
}
