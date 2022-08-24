use ethers::contract::Abigen;
use eyre::Result;

fn main() -> Result<()> {
    Abigen::new("UniswapV2Pair", "abi/UniswapV2Pair.json")?
        .generate()?
        .write_to_file("src/abi/uniswap_v2_pair.rs")?;
    Ok(())
}
