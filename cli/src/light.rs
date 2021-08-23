use initiative_core::App;
use std::io;

pub async fn run(mut app: App) -> io::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();

    loop {
        match stdin.read_line(&mut buffer) {
            Ok(0) => return Ok(()),
            Ok(_) => match app.command(&buffer).await {
                Ok(s) => println!("\n{}\n", s),
                Err(e) => eprintln!("\n{}\n", e),
            },
            Err(e) => return Err(e),
        }

        buffer.clear();
    }
}
