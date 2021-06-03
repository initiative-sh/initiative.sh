use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    initiative::run()?;
    Ok(())
}
