use ethers::types::NameOrAddress;

use crate::{
    constant::{ETH_TRANSFER, WEI},
    signer::{evm_signer, get_balance_eth, info_tx, send_eth},
    utils::{self, is_file_exists, wei_to_eth, wei_to_gwei},
};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub async fn cmd(rpc_url: &String, private_key: &String, wallets_path: &String, amount: &f64) {
    let wallets_to_supply = utils::read_lines(wallets_path).unwrap_or_else(|_| {
        log::error!("Unable to read wallets file");
        std::process::exit(1);
    });

    if wallets_to_supply.is_empty() {
        log::error!("No wallets found in file");
        std::process::exit(1);
    };

    let len_wallets = wallets_to_supply.len();
    let (supplier_signer, supplier_wallet) = evm_signer(rpc_url, private_key).await;
    let supplier_balance = get_balance_eth(&supplier_signer).await;

    let (max_fee_per_gas, max_priority_fee_per_gas, nonce) = info_tx(&supplier_signer).await;
    
    let amount_per_wallet = (amount * WEI / len_wallets as f64) + (max_fee_per_gas.as_u128() as f64 * ETH_TRANSFER);
    let total_amount = amount_per_wallet * len_wallets as f64;
    
    log::info!("⛽ Base fee: {:.5} GWEI", wei_to_gwei(max_fee_per_gas.as_u128() as f64));
    log::info!("💰 Supplier {supplier_wallet} | Balance {supplier_balance:.5} ETH");
    log::info!("📦 Sending {amount:.5} ETH to {len_wallets} wallets ({:.5} ETH per wallet)", wei_to_eth(amount_per_wallet));

    if supplier_balance < wei_to_eth(total_amount) {
        log::error!("Insufficient balance to supply wallets ({supplier_balance:.5} < {total_amount:.5})");
        std::process::exit(1);
    };

    println!("{amount_per_wallet} ETH per wallet");

    for wallet in wallets_to_supply {
        let wallet = wallet.parse::<NameOrAddress>().unwrap_or_else(|_| {
            log::error!("Unable to parse wallet");
            std::process::exit(1);
        });

        send_eth(&supplier_signer, wallet, &amount_per_wallet, nonce, max_fee_per_gas,max_priority_fee_per_gas).await;
    }
}
