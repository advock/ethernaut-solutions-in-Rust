use ethers::{abi::Address, prelude::*};
use eyre::Result;
use std::{convert::TryFrom, sync::Arc};

use dotenv::dotenv;
abigen!(Vault, "src/Vault.json");
#[tokio::main]

pub async fn vau() -> Result<()> {
    dotenv().ok();

    let url = dotenv::var("URl").unwrap();
    let Private_key = dotenv::var("PRIVATE_KEY").unwrap();
    let client = Arc::new({
        let client = Provider::<Http>::try_from(url)?;
        let chain_id = client.get_chainid().await?;

        let wallet = Private_key
            .parse::<LocalWallet>()?
            .with_chain_id(chain_id.as_u64());

        SignerMiddleware::new(client, wallet)
    });

    let addr = "0x77a2EB328323DF6EdB19Fc782d1DCA0d321Ec165".parse::<Address>()?;

    let cont = Vault::new(addr, client.clone());

    let location = H256::from_low_u64_be(1);

    let Blocknumber = client.get_block_number().await.unwrap();

    let t = BlockId::from(Blocknumber);

    let x = client.get_storage_at(addr, location, Some(t)).await?;

    let y = H256::to_fixed_bytes(x);

    let tx = cont.unlock(y).send().await?.await?;

    Ok(())
}
