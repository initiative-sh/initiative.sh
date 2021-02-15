use rand::thread_rng;
use std::error::Error;
use std::io;

use initiative_macros::RandomTable;

fn main() -> Result<(), Box<dyn Error>> {
    let mut rng = thread_rng();
    println!(
        "{:?}",
        Building::get_random(&mut rng, &Demographics::default())
    );
    Ok(())
    /*
    let mut buffer = String::new();
    let mut world = initiative::World::default();

    loop {
        io::stdin().read_line(&mut buffer)?;
        println!("{}", world.run(&buffer));
        buffer.clear();
    }
    */
}

#[derive(Debug, RandomTable)]
pub enum Building {
    Residence,
    Religious,
    Tavern,
    Warehouse,
    Shop,
}

pub trait RandomTable {
    fn get_random(rng: &mut impl rand::Rng, demographics: &Demographics) -> Self;
}

#[derive(Default)]
pub struct Demographics {}
