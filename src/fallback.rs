use ethers::prelude::*;
use eyre::Result;
use std::{convert::TryFrom, sync::Arc};

use dotenv::dotenv;

abigen!(Fallback, "src/fallbackABI.json");
pub async fn done() -> Result<()> {
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

    let address = "0x2D10B088cd3F6281Be528C2905129755368E0ad1".parse::<Address>()?;

    let conntract = Fallback::new(address, client.clone());

    let contribute = conntract.contribute().value(1).send().await?.await?;

    let contribution = conntract.get_contribution().call().await?;

    let send_eth = TransactionRequest::new().to(address).value(1000);

    let send_eth_tx = client.send_transaction(send_eth, None).await?.await?;

    let widraw = conntract.withdraw().send().await?.await?;

    print!("{:?}", widraw);

    Ok(())
}
