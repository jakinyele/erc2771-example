use clap::Parser;
use ethers::prelude::*;
use ethers::utils::keccak256;
use std::str::FromStr;
use serde::{Serialize, Deserialize};
use std::time::Duration;
use ethers::{
    // contract::{Eip712, EthAbiType},
    core::{
        types::{TransactionRequest, transaction::eip712::{Eip712, TypedData}, Address, U256},
        utils::hex,
    },    
    providers::{Http, Middleware, Provider},
};
use rlp::Rlp; 
use ethers::abi::struct_def::StructFieldType::Type;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    nonce: u64,
    // typed structured data as json
    json: std::path::PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let current_nonce = args.nonce;
    // let private_key_hex = args.private_key;
    let json = args.json;
    let mut file = File::open(json).expect("Failed to open json file");

    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read json file");

    let mut typed_data: TypedData = serde_json::from_str(&contents).expect("Failed to parse JSON");
    println!("Typed Data Structure: \n{:#?}\n", typed_data);

    // update with a replacement none if current_nonce > 0 && nonce key exists
    if (current_nonce > 0 && typed_data.message.contains_key("nonce")) {
        typed_data.message.insert("nonce".to_string(), current_nonce.into());
    }

    println!("Here is the final message:");
    println!("=======================");
    for (k, v) in &typed_data.message {
        println!("{}: \"{}\"", k, v);
    }    
    println!("=======================");

    let struct_hash = typed_data.struct_hash().unwrap();
    println!("Struct hash: 0x{}", hex::encode(&struct_hash));

    let message_hash = typed_data.encode_eip712().unwrap();
    println!("Message hash: 0x{}", hex::encode(&message_hash));

    Ok(())
}
