use riva::terminal::Terminal;

use std::error::Error;
use std::process::exit;

fn main() -> Result<(), Box<dyn Error>> {
    let mut term = Terminal;

    if let Err(err) = Terminal::run(&mut term) {
        eprintln!("Error running terminal: {:#?}", err);
        exit(1);
    }

    Ok(())
}
