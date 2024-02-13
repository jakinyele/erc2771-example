
use alloy_primitives::Address;
use alloy_sol_types::{Eip712Domain, eip712_domain, SolStruct};
use alloy_primitives::{U256, keccak256};
use alloy_sol_types::sol;
use std::str::FromStr;

sol! {
     struct ForwardRequest {
        address from;
        address to;
        uint256 value;
        uint256 gas;
        uint256 nonce;
        uint48 deadline;
        bytes data;
     }
}

fn main() {

    let meta_tx_domain: Eip712Domain = eip712_domain! {
        name: "Meta Transaction Forwarder",
        version: "1",
        chain_id: 1115111,
        verifying_contract: Address::from_str("0xBE4dcaE032a02110e37c4767B8EC6bFd40bdF5b7").unwrap(),
        // salt: keccak256("my domain salt"),
    };
    
    // vec for 0xa9059cbb000000000000000000000000966ef13ff6974ec7b7018cdbdedd77e9bffc33670000000000000000000000000000000000000000000000000000000000000001
    let data_vec = vec![169, 5, 156, 187, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 150, 110, 241, 63, 246, 151, 78, 199, 183, 1, 140, 219, 222, 221, 119, 233, 191, 252, 51, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
    let hashed_data_vec = keccak256(data_vec.clone());

    println!("Hashed Data: {}", hashed_data_vec);

    let forward_request_message = ForwardRequest {
        from: "0x1d98BF1FE5ae430A98461bAd3b872031767c9634".parse().unwrap(),
        to: "0x59f913ae23172a22Ffc2846a56c0E8d7636B927F".parse().unwrap(),
        value: U256::from(0),
        gas: U256::from(100000),
        nonce: U256::from(0),
        deadline: 1707594318 as u64,
        data: data_vec.to_vec(),
    }; 

    let encoded_type = ForwardRequest::eip712_encode_type();
    let type_hash = keccak256(encoded_type.as_bytes());
    println!("Forward Request Type Hash: {}", type_hash);

    let domain_separator = meta_tx_domain.separator();
    println!("Domain: {:#?}", meta_tx_domain);
    println!("Domain Separator: {}", domain_separator);

    let hash_struct = forward_request_message.eip712_hash_struct();
    println!("Hash struct: {}", hash_struct);

    let message_hash = forward_request_message.eip712_signing_hash(&meta_tx_domain);
    println!("Message hash: {}", message_hash);

    // let message_hash2 = alloy_sol_types::SolStruct::eip712_signing_hash(&forward_request_message, &meta_tx_domain);
    // println!("Message hash 2: {}", message_hash2);
}
