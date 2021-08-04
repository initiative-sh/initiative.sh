use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    initiative_cli::run(initiative_core::app()).await?;
    Ok(())
}
