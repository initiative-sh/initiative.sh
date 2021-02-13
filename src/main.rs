use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    let mut world = initiative::World::default();

    loop {
        io::stdin().read_line(&mut buffer)?;
        println!("{}", world.run(&buffer));
        buffer.clear();
    }
}
