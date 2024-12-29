use ethers::{
    middleware::SignerMiddleware,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
};

use crate::utils::get_rpc_from_config;

pub async fn evm_signer(
    rpc_url: &String,
    private_key: &String,
) -> SignerMiddleware<Provider<Http>, LocalWallet> {

    let provider = Provider::<Http>::try_from(rpc_url).unwrap();
    let chain_id = provider.get_chainid().await.unwrap();
    
    let wallet = private_key
        .parse::<LocalWallet>()
        .unwrap()
        .with_chain_id(chain_id.as_u64());

    SignerMiddleware::new(provider, wallet)
}
