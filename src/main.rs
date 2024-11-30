use std::collections::HashMap;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let resp:HashMap<String, String>= reqwest::get("https://httpbin.org/ip")
    .await?
    .json::<HashMap<String, String>>()
    .await?;

    log::info!("{:#?}",resp);

    Ok(())
}
