#![allow(non_upper_case_globals)]

use std::error::Error;

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
fn a_smart_contract_function(params: &mut Parameters) -> Result<(), String> {
    let _a: u8 = params.read(); // Read a single unsigned byte.
    let _b: i8 = params.read(); // Read a single signed byte.
    
    let _c: u16 = params.read(); // Read a single unsigned 16-bit integer.
    let _d: i16 = params.read(); // Read a single signed 16-bit integer.
    
    let _e: u32 = params.read(); // Read a single unsigned 32-bit integer.
    let _f: i32 = params.read(); // Read a single signed 32-bit integer.
        
    let _g: u64 = params.read(); // Read a single unsigned 64-bit integer.
    let _h: i64 = params.read(); // Read a single signed 64-bit integer.
        
    let _i: u128 = params.read(); // Read a single unsigned 128-bit integer.
    let _j: i128 = params.read(); // Read a single signed 128-bit integer.
    
    let _k: bool = params.read(); // Read a single byte as a boolean. 0 is false, 1 is true.
    
    let _l: String = params.read(); // Read a single string prefixed by an unsigned 32-bit integer.
    
    let _m: Vec<u8> = params.read(); // Read a vector of bytes prefixed by an unsigned 32-bit integer.
    
    let _n: [u8; 32] = params.read(); // Read exactly 32 bytes.
    let _wallet_address: [u8; 32] = params.read(); // Wallet addresses in Wavelet are 32 bytes.
    
    // Note that the `read()` function may be type-postfixed as well.
    // For example: `let wallet_address = params.read::<[u8; 32>();`
    
    Ok(())
}

