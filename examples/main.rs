use eyre::Result;

use price_rs::{get_price, Network};

#[tokio::main]
async fn main() -> Result<()> {
    let price = get_price(
        Network::Ethereum,
        "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2",
    )
    .await
    .unwrap();
    println!("{:.2}", price);
    Ok(())
}
