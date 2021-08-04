use super::App;
use std::io;

pub async fn run(mut app: App) -> io::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();

    loop {
        match stdin.read_line(&mut buffer) {
            Ok(0) => return Ok(()),
            Ok(_) => println!("\n{}\n", app.command(&buffer).await),
            Err(e) => return Err(e),
        }

        buffer.clear();
    }
}
