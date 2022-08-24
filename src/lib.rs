use std::ops::Div;
use std::str::FromStr;
use std::sync::Arc;

use bigdecimal::BigDecimal;
use ethers::providers::Http;
use ethers::providers::Provider;
use ethers::types::Address;
use ethers::types::U256;
use eyre::Result;
use serde::Deserialize;
use toml;

#[rustfmt::skip]
mod abi;

pub enum Network {
    Ethereum,
}

#[derive(Deserialize)]
struct Config {
    ethereum: NetworkConfig,
}

#[derive(Deserialize)]
struct NetworkConfig {
    yearn_lens_oracle: String,
    uniswapv2_route02: String,
    usdc: String,
    usdc_decimals: usize,
}

pub async fn get_price(network: Network, token_address: &str) -> Result<BigDecimal> {
    let toml_content = r#"
          [ethereum]
          yearn_lens_oracle = "0x83d95e0d5f402511db06817aff3f9ea88224b030"
          uniswapv2_route02 = "0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D"
          usdc = "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48"
          usdc_decimals = 6
          "#;
    let config: Config = toml::from_str(toml_content)?;
    let network_config = match network {
        Network::Ethereum => config.ethereum,
    };
    let client: Provider<Http> = Provider::<Http>::try_from(
        "https://mainnet.infura.io/v3/c60b0bb42f8a4c6481ecd229eddaca27",
    )?;
    let client = Arc::new(client);
    // let uniswap_price = via_uniswapv2_router(&client, &network_config, token_address).await.unwrap();
    // Ok(uniswap_price)
    let yearn_lens_price = via_yearn_lens_oracle(&client, &network_config, token_address)
        .await
        .unwrap();
    Ok(yearn_lens_price)
}

async fn via_yearn_lens_oracle(
    client: &Arc<Provider<Http>>,
    network_config: &NetworkConfig,
    token_address: &str,
) -> Result<BigDecimal> {
    let oracle = abi::yearn_lens_oracle::YearnLensOracle::new(
        network_config.yearn_lens_oracle.parse::<Address>()?,
        Arc::clone(&client),
    );
    let usdc_mantissa = oracle
        .get_price_usdc_recommended(token_address.parse::<Address>()?)
        .call()
        .await?;
    Ok(u256_to_bd(usdc_mantissa).div(u256_to_bd(U256::exp10(network_config.usdc_decimals))))
}

async fn via_uniswapv2_router(
    client: &Arc<Provider<Http>>,
    network_config: &NetworkConfig,
    token_address: &str,
) -> Result<BigDecimal> {
    let route = abi::uniswap_v2_route_02::UniswapV2Route02::new(
        network_config.uniswapv2_route02.parse::<Address>()?,
        Arc::clone(&client),
    );
    let token_address = token_address.parse::<Address>()?;
    let token_decimals = get_token_decimals(client, token_address).await.unwrap();
    let usdc_mantissa = *route
        .get_amounts_out(
            U256::exp10(token_decimals as usize),
            vec![token_address, network_config.usdc.parse::<Address>()?],
        )
        .call()
        .await?
        .last()
        .unwrap();
    Ok(u256_to_bd(usdc_mantissa).div(u256_to_bd(U256::exp10(network_config.usdc_decimals))))
}

async fn get_token_decimals(client: &Arc<Provider<Http>>, token_address: Address) -> Result<u8> {
    let erc20 = abi::erc20::ERC20::new(token_address, Arc::clone(client));
    let decimals = erc20.decimals().call().await?;
    Ok(decimals)
}

fn u256_to_bd(n: U256) -> BigDecimal {
    BigDecimal::from_str(n.to_string().as_str()).unwrap()
}
