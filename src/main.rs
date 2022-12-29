use ethers::prelude::*;
use eyre::Result;
use std::{convert::TryFrom, ops::Add, sync::Arc};

use dotenv::dotenv;

abigen!(Hack, "src/hack.json");
#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let url = dotenv::var("URl").unwrap();
    let key = dotenv::var("PRIVATE_KEY").unwrap();

    let client = Arc::new({
        let client = Provider::<Http>::try_from(url)?;
        let chain_id = client.get_chainid().await?;
        let wallet = key.parse::<LocalWallet>()?.with_chain_id(chain_id.as_u64());

        SignerMiddleware::new(client, wallet)
    });

    let add = "0x93e8cd3B1b3fd0D463E7471A9ccf7a4D152be699".parse::<Address>()?;

    let con = Hack::new(add, client.clone());

    let tx = con.wit().send().await?.await?;
    Ok(())
}
