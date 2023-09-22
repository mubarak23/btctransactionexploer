#[warn(unused_imports)]

extern crate bitcoin;
extern  crate reqwest;
extern crate hex;

use bitcoin::util::address::Address;
use bitcoin::blockdata::transaction::Transaction;
use bitcoin::network::constants::Network;
use std::io;

// use bitcoin::util::misc::hex_bytes;

// fn check_network(txid_hex: &str) -> Option<String> {
//     let txid_bytes = hex_bytes(txid_hex).ok()?;

//     match txid_bytes.get(0) {
//         Some(&0x00) | Some(&0x01) => Some("mainet".to_string()),
//         Some(&0x6f) | Some(&0x3a) => Some("testnet".to_string()),
//         _ => None,
//     }
// }

fn check_network(txid_hex: &str) -> Option<String> {
    let txid_bytes = hex::decode(txid_hex).ok()?;

    let network = if txid_bytes.len() == 32 && txid_bytes[0] == 0x00 {
        Network::Bitcoin
    } else if txid_bytes.len() == 32 && txid_bytes[0] == 0x6f {
        Network::Testnet
    } else {
        return  None
    };
    
    Some(network.to_string())
}

fn main() {
    println!("Hello, world!");
}
