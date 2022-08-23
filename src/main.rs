use eyre::Result;

use price_rs::get_price;

#[tokio::main]
async fn main() -> Result<()> {
    let price = get_price("0x0d4a11d5EEaaC28EC3F61d100daF4d40471f1852").await.unwrap();
    println!("ETH/USDT price: {:.2}", price);
    Ok(())
}
