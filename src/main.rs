use ethers::prelude::*;
use eyre::Result;
use std::{convert::TryFrom, sync::Arc};
use vec;

use dotenv::dotenv;

abigen!(Privacy, "src/Privicy.json");

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let url = dotenv::var("URL").unwrap();
    let private_key = dotenv::var("PRIVATE_KEY").unwrap();

    let client = Arc::new({
        let client = Provider::<Http>::try_from(url)?;
        let chain_id = client.get_chainid().await?;

        let wallet = private_key
            .parse::<LocalWallet>()?
            .with_chain_id(chain_id.as_u64());

        SignerMiddleware::new(client, wallet)
    });

    let from = "0xff9425eabb4D9885ACB444A63E125695e8D9Fd76".parse::<Address>()?;

    let r = &from;

    let c: [u8; 16] = [
        210, 242, 123, 48, 165, 206, 143, 35, 91, 223, 18, 125, 242, 60, 150, 206,
    ];

    let con = Privacy::new(*r, client.clone());

    let location = H256::from_low_u64_be(5);

    let blocknumber = client.get_block_number().await.unwrap();

    let block = BlockId::from(blocknumber);

    let storage = client.get_storage_at(from, location, Some(block)).await?;

    let tx = con.unlock(c).send().await?.await?;

    println!("{:?}", tx);

    Ok(())
}
