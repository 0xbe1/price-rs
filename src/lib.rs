use std::ops::Div;
use std::sync::Arc;

use ethers::providers::Http;
use ethers::providers::Provider;
use ethers::types::Address;
use ethers::types::U256;
use eyre::Result;

#[rustfmt::skip]
mod abi;

const USDC: &str = "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48";
const UNISWAPV2_ROUTE02: &str = "0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D";
const USDC_DECIMALS: usize = 6;

pub async fn get_price(token_address: &str) -> Result<U256> {
    let client: Provider<Http> = Provider::<Http>::try_from(
        "https://mainnet.infura.io/v3/c60b0bb42f8a4c6481ecd229eddaca27",
    )?;
    let client = Arc::new(client);
    let uniswap_price = via_uniswapv2_router(&client, token_address).await.unwrap();
    Ok(uniswap_price)
}

async fn via_uniswapv2_router(client: &Arc<Provider<Http>>, token_address: &str) -> Result<U256> {
    let route = abi::uniswap_v2_route_02::UniswapV2Route02::new(
        UNISWAPV2_ROUTE02.parse::<Address>()?,
        Arc::clone(&client),
    );
    let token_address = token_address.parse::<Address>()?;
    let token_decimals = get_token_decimals(client, token_address).await.unwrap();
    let amounts_out = route
        .get_amounts_out(
            U256::exp10(token_decimals as usize),
            vec![token_address, USDC.parse::<Address>()?],
        )
        .call()
        .await?;
    Ok(amounts_out.last().unwrap().div(U256::exp10(USDC_DECIMALS)))
}

async fn get_token_decimals(client: &Arc<Provider<Http>>, token_address: Address) -> Result<u8> {
    let erc20 = abi::erc20::ERC20::new(token_address, Arc::clone(client));
    let decimals = erc20.decimals().call().await?;
    Ok(decimals)
}
