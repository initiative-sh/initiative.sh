use std::error::Error;

use command::Command;

mod command;

pub fn run_command(input: &str) -> Result<String, Box<dyn Error>> {
    let command: Command = input.parse()?;

    let output = format!("{:?}", command);

    Ok(output)
}
