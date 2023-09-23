#[warn(unused_imports)]

extern crate bitcoin;
extern  crate reqwest;
extern crate hex;

use bitcoin::util::address::Address;
use bitcoin::blockdata::transaction::Transaction;
use bitcoin::network::constants::Network;
use std::error::Error;
use std::fmt;
use std::io;

use tokio::runtime;

#[derive(Debug)]
struct CustomReqwestError {
    message: String,
}

impl CustomReqwestError {
    fn new(message: &str) -> Self {
        CustomReqwestError {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for CustomReqwestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for CustomReqwestError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

// use bitcoin::util::misc::hex_bytes;

// fn check_network(txid_hex: &str) -> Option<String> {
//     let txid_bytes = hex_bytes(txid_hex).ok()?;

//     match txid_bytes.get(0) {
//         Some(&0x00) | Some(&0x01) => Some("mainet".to_string()),
//         Some(&0x6f) | Some(&0x3a) => Some("testnet".to_string()),
//         _ => None,
//     }
// }


async fn fetch_transaction_details(txid_hex: &str, network: &str) -> Result<String, Box<dyn Error>> {
    let base_url = match network {
        "Bitcoin" => "https://blockstream.info/api/tx/",
        "Testnet" => "https://blockstream.info/testnet/api/tx/",
        _ => return Err(Box::new(CustomReqwestError::new("Invalid network"))),
    };
// Create the full explorer URL.
    let explorer_url = format!("{}{}", base_url, txid_hex);

    // Send an HTTP GET request to the blockchain explorer API.
    let response = reqwest::get(&explorer_url).await?;

    if response.status().is_success() {
        // Read and return the response body (transaction details).
        let body = response.text().await?;
        Ok(body)
    } else {
        Err(Box::new(CustomReqwestError::new("Failed to fetch transaction details")))
    }

}
fn main() {
    // Prompt the user for the transaction ID.
    println!("Enter the Bitcoin transaction ID (TxID) in hexadecimal format:");
    let mut txid_hex = String::new();
    io::stdin().read_line(&mut txid_hex).expect("Failed to read line");

    // Remove leading/trailing whitespace and newline characters.
    let txid_hex = txid_hex.trim();
    println!("User input at this point: {}", txid_hex);
    // Check the network and display the result.
    let network = check_network(&txid_hex);
    let result = runtime::Runtime::new().unwrap().block_on(fetch_transaction_details(&txid_hex, &network));

    match result {
        Ok(details) => {
            println!("Transaction Details for TxID {} on the {} network:", txid_hex, network);
            println!("{}", details);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}

fn check_network(txid_hex: &str) -> String {
    let txid_bytes = hex::decode(txid_hex).unwrap_or_default();
    println!("Hex data for transaction: {}", txid_bytes.len());
    if txid_bytes.len() == 32 && txid_bytes[0] == 0x00 {
        Network::Bitcoin.to_string()
    } else if txid_bytes.len() == 32 && txid_bytes[0] == 0x6f {
        Network::Testnet.to_string()
    } else {
        "Invalid or unrecognized Bitcoin network".to_string()
    }
}