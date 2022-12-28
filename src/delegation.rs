use ethers::prelude::*;
use eyre::Result;
use std::{convert::TryFrom, sync::Arc};

use dotenv::dotenv;

abigen!(Delegation, "src/delegation.json");
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

    let add = "0xA6B23fF688208Dad9390C509d4500c006c373686".parse::<Address>()?;

    let cont = Delegation::new(add, client.clone());

    let d = "dhcbdh".parse::<Bytes>()?;

    let tx = TransactionRequest::new().data(d as Bytes).to(add);

    Ok(())
}
