use initiative_cli as cli;
use initiative_core as core;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let data_store = core::NullDataStore::default();
    cli::run(core::app(data_store, &event_dispatcher)).await?;
    Ok(())
}

fn event_dispatcher(event: core::Event) {
    println!("Dispatched event: {:?}", event);
}
