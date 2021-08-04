use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let data_store = initiative_core::NullDataStore::default();
    initiative_cli::run(initiative_core::app(data_store)).await?;
    Ok(())
}
