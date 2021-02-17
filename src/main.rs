use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    let mut context = initiative::Context::default();

    loop {
        io::stdin().read_line(&mut buffer)?;
        println!("{}", context.run(&buffer));
        buffer.clear();
    }
}
