use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();

    loop {
        io::stdin().read_line(&mut buffer)?;

        match initiative::run_command(&buffer) {
            Ok(output) => println!("{}", output),
            Err(e) => println!("{}", e),
        }

        buffer.clear();
    }
}
