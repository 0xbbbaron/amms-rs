use std::{error::Error, str::FromStr, sync::Arc};

use ethers::{
    providers::{Http, Provider},
    types::H160,
};

use damms::{
    amm::{
        factory::Factory, uniswap_v2::factory::UniswapV2Factory,
        uniswap_v3::factory::UniswapV3Factory,
    },
    sync,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    //Add rpc endpoint here:
    let rpc_endpoint =
        std::env::var("ETHEREUM_RPC_ENDPOINT").expect("Could not get ETHEREUM_RPC_ENDPOINT");
    let provider = Arc::new(Provider::<Http>::try_from(rpc_endpoint).unwrap());

    let factories = vec![
        //UniswapV2
        Factory::UniswapV2Factory(UniswapV2Factory::new(
            H160::from_str("0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f").unwrap(),
            2638438,
            300,
        )),
        //Add Sushiswap
        Factory::UniswapV2Factory(UniswapV2Factory::new(
            H160::from_str("0xC0AEe478e3658e2610c5F7A4A2E1777cE9e4f2Ac").unwrap(),
            10794229,
            300,
        )),
        //Add UniswapV3
        Factory::UniswapV3Factory(UniswapV3Factory::new(
            H160::from_str("0x1F98431c8aD98523631AE4a59f267346ea31F984").unwrap(),
            12369621,
        )),
    ];

    //Sync pairs
    sync::sync_amms(factories, provider, None).await?;

    Ok(())
}