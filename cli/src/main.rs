use initiative_cli as cli;
use initiative_core as core;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let data_store = core::NullDataStore::default();
    cli::run(core::app(data_store, core::NullAccountManager::default())).await?;
    Ok(())
}
