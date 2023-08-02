use riva::terminal::input;

use std::error::Error;
use std::process::exit;

fn main() -> Result<(), Box<dyn Error>> {
    let term = input::Terminal;

    if let Err(err) = input::Terminal::run(&term) {
        eprintln!("Error running terminal: {:#?}", err);
        exit(1);
    }

    Ok(())
}
