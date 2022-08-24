use ethers::contract::Abigen;
use eyre::Result;

fn main() -> Result<()> {
    Abigen::new("ERC20", "abi/ERC20.json")?
        .generate()?
        .write_to_file("src/abi/erc20.rs")?;
    Abigen::new("UniswapV2Pair", "abi/UniswapV2Pair.json")?
        .generate()?
        .write_to_file("src/abi/uniswap_v2_pair.rs")?;
    Abigen::new("UniswapV2Route02", "abi/UniswapV2Route02.json")?
        .generate()?
        .write_to_file("src/abi/uniswap_v2_route_02.rs")?;
    Ok(())
}
