use ethers::prelude::*;
use eyre::Result;
use std::{convert::TryFrom, sync::Arc};

use dotenv::dotenv;

abigen!(Acak, "src/preservation.json");
abigen!(Fallout, "src/fallout.json");
#[tokio::main]
pub async fn main() -> Result<()> {
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

    let ad = "0xFd9c431E4be49988048E8C690d7b47F67317C436".parse::<Address>()?;

    let tract = Acak::new(ad, client.clone());
    let falout = Fallout::new(ad, client.clone());

    let tx = tract.setTime(0x70bb59C0f2ead11E352287F4834f03b3148510F8).send().await?.await?;



    Ok(())
}
