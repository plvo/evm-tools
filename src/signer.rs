use ethers::{
    etherscan::gas,
    middleware::SignerMiddleware,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
    types::{
        transaction::eip2718::TypedTransaction, Eip1559TransactionRequest, NameOrAddress, H160, U256
    },
    utils::parse_ether,
};

use crate::{
    constant::GWEI,
    utils::{get_rpc_from_config, wei_to_gwei},
};

pub async fn evm_signer(
    rpc_url: &String,
    private_key: &String,
) -> (SignerMiddleware<Provider<Http>, LocalWallet>, H160) {
    let provider = Provider::<Http>::try_from(rpc_url).unwrap_or_else(|_| {
        log::error!("Unable to create provider, check your RPC URL");
        std::process::exit(1);
    });

    let chain_id = provider.get_chainid().await.unwrap_or_else(|_| {
        log::error!("Unable to get chain ID from provider");
        std::process::exit(1);
    });

    let wallet = private_key
        .parse::<LocalWallet>()
        .unwrap_or_else(|_| {
            log::error!("Unable to parse private key, check your private key");
            std::process::exit(1);
        })
        .with_chain_id(chain_id.as_u64());

    (SignerMiddleware::new(provider, wallet.clone()), wallet.address())
}

// Returns information about the transaction (base fee, max fee per gas, nonce)
pub async fn info_tx(client: &SignerMiddleware<Provider<Http>, LocalWallet>) -> (U256, U256, U256) {
    let base_fee = client.get_gas_price().await.unwrap_or_else(|_| {
        log::error!("Unable to get base fee");
        std::process::exit(1);
    });

    let priority_fee = U256::from(GWEI as u64); // Add 1 GWEI to the base fee
    let max_fee_per_gas = base_fee + priority_fee;

    let nonce: U256 = client
        .get_transaction_count(client.address(), None)
        .await
        .unwrap_or_else(|_| {
            log::error!("Unable to get nonce");
            std::process::exit(1);
        });

    (max_fee_per_gas, priority_fee, nonce)
}

// Sends ETH to the specified address
pub async fn send_eth(
    client: &SignerMiddleware<Provider<Http>, LocalWallet>,
    to: NameOrAddress,
    amount: &f64,
    nonce: U256,
    max_fee_per_gas: U256,
    max_priority_fee_per_gas: U256,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create a new EIP-1559 transaction
    let tx = TypedTransaction::Eip1559(Eip1559TransactionRequest {
        to: Some(to),
        value: Some(U256::from(parse_ether(amount).unwrap())),
        nonce: Some(nonce),
        data: None,
        gas: Some(U256::from(21000)),
        max_fee_per_gas: Some(max_fee_per_gas),
        max_priority_fee_per_gas: Some(max_priority_fee_per_gas),
        ..Default::default()
    });

    // Send the transaction
    let pending_tx = client.send_transaction(tx, None).await.map_err(|e| {
        log::error!("Unable to send transaction: {:?}", e);
        e
    })?;

    let receipt = pending_tx.await?.ok_or_else(|| {
        log::error!("Transaction failed");
        std::io::Error::new(std::io::ErrorKind::Other, "Transaction failed")
    })?;

    let to_address = receipt.to.unwrap_or_default();
    let tx_hash = receipt.transaction_hash;
    let gwei_used = wei_to_gwei(receipt.effective_gas_price.unwrap_or_default().as_u64() as f64);

    log::info!("🏹 Sent {amount:.5} ETH | TO: {to_address} | TX: {tx_hash} | {gwei_used:.3} GWEI");

    Ok(())
}

pub async fn get_balance_eth(client: &SignerMiddleware<Provider<Http>, LocalWallet>) -> f64 {
    let balance = client
        .get_balance(client.address(), None)
        .await
        .unwrap_or_else(|_| {
            log::error!("Unable to get balance");
            std::process::exit(1);
        });
    let balance_eth = ethers::utils::format_units(balance, "ether")
        .expect("Unable to format balance")
        .parse::<f64>()
        .expect("Unable to parse balance");

    balance_eth
}
