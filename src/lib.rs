use std::sync::Arc;

use ethers::providers::Http;
use ethers::providers::Provider;
use ethers::types::Address;
use eyre::Result;

#[rustfmt::skip]
mod abi;

pub async fn get_price(pair_address: &str) -> Result<f64> {
    // uniswap pair
    let client: Provider<Http> = Provider::<Http>::try_from(
        "https://mainnet.infura.io/v3/c60b0bb42f8a4c6481ecd229eddaca27",
    )?;
    let client = Arc::new(client);
    let uniswap_price = get_uniswap_price(client, pair_address).await.unwrap();
    Ok(uniswap_price)
}

async fn get_uniswap_price(client: Arc<Provider<Http>>, pair_address: &str) -> Result<f64> {
    let pair_address = pair_address.parse::<Address>()?;
    let pair = abi::uniswap_v2_pair::UniswapV2Pair::new(pair_address, Arc::clone(&client));
    let (reserve0, reserve1, _timestamp) = pair.get_reserves().call().await?;
    let mid_price = f64::powi(10.0, 18 - 6) * reserve1 as f64 / reserve0 as f64;
    Ok(mid_price)
}
