use crate::utils::{is_file_exists, open_or_create_file, timestamp};
use ethers::{
    signers::{LocalWallet, Signer},
    utils::hex,
};
use std::io::Write;

pub async fn cmd(count: &u32) {
    let ts = timestamp();    
    let path = format!("./wallets/created_evm_{ts}.csv");
    println!("⏳ Generating {count} wallets and saving them to {path}");

    let start = std::time::Instant::now();

    let mut file = open_or_create_file(&path);
    let mut writer = csv::Writer::from_writer(&file);
    if file.metadata().unwrap().len() == 0 {
        writer.write_record(&["Public Key", "Private Key"]).unwrap();
    }

    for i in 0..*count {
        let new_wallet: LocalWallet = LocalWallet::new(&mut rand::thread_rng());

        let public_key = format!("{:?}", new_wallet.address());

        let private_key_bytes = new_wallet.signer().to_bytes();
        let private_key = hex::encode(private_key_bytes);

        writer.write_record(&[&public_key, &private_key]).unwrap();

        print!("\r🔑 Wallet {} created: {public_key}",i+1);
    }
    writer.flush().unwrap();

    let elapsed = start.elapsed();

    println!("\n⌛ {count} new wallets saved to {path} in {}sec",elapsed.as_secs());
}
