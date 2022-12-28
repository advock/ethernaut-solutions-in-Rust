use dotenv::dotenv;
use ethers::prelude::*;
use eyre::Result;
use std::string::String;
use std::{convert::TryFrom, sync::Arc};

abigen!(DestroyTelephone, "src/DestroyTelephone.json");
#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let url = dotenv::var("URL").unwrap();
    let Private_key = dotenv::var("PRIVATE_KEY").unwrap();

    let client = Arc::new({
        let client = Provider::<Http>::try_from(url)?;
        let chain_id = client.get_chainid().await?;

        let wallet = Private_key
            .parse::<LocalWallet>()?
            .with_chain_id(chain_id.as_u64());

        SignerMiddleware::new(client, wallet)
    });

    let addr = "0x18104F37EfD8c51E04a9dc18133bd0Be72Aa9612".parse::<Address>()?;

    let con = DestroyTelephone::new(addr, client.clone());

    let tx = con.call().send().await?.await?;

    Ok(())
}
