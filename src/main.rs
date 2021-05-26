use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let context = initiative::Context::default();
    initiative::run(context)?;
    Ok(())
}
