use dotenv::dotenv;
use ethers::prelude::*;
use eyre::Result;
use std::string::String;
use std::{convert::TryFrom, sync::Arc};

#[tokio::main]
async fn main() -> Result<()> {
    // uint256 blockValue = uint256(blockhash(block.number - 1));

    // if (lastHash == blockValue) {
    //     revert();
    // }

    // lastHash = blockValue;
    // uint256 coinFlip = blockValue / FACTOR;
    // bool side = coinFlip == 1 ? true : false;
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

    let Blocknumber = client.get_block_number().await.unwrap();

    let block = client.get_block(Blocknumber).await?.unwrap();

    let hash = block.hash.unwrap();

    let j = BigEndianHash::into_uint(&hash);

    print!("{:?}", j);

    Ok(())
}
