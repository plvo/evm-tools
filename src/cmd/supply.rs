use std::{
    fs::File,
    io::{BufRead, BufReader},
};
use crate::{signer::evm_signer, utils::is_file_exists};

pub async fn cmd(rpc_url:&String, private_key: &String, wallets_path: &String, amount: &f64) {
    let supplier_signer = evm_signer(rpc_url, private_key).await;

    println!("🔑 First line of private key file: {}", private_key);

    println!("rpc_url: {}", rpc_url);
}
