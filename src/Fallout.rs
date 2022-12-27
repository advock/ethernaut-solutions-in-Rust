use ethers::prelude::*;
use eyre::Result;
use std::{convert::TryFrom, sync::Arc};

use dotenv::dotenv;

abigen!(Fallout, "src/fallout.json");

pub async fn fallo() -> Result<()> {
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

    let address: H160 = "0x87878F04b3ba9e85034A32f1BC5d8CcA17841076".parse::<Address>()?;

    let falout = Fallout::new(address, client.clone());

    let tx = falout.fal_1out().value(1).send().await?.await?;

    Ok(())
}
