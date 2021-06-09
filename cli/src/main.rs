use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    initiative_cli::run(initiative_core::app())?;
    Ok(())
}
