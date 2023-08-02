use std::io::{
    Write, 
    stdout,
};
use std::time::Duration;
use crossterm::{
    execute,
    Result,
    terminal::{
        enable_raw_mode,
        disable_raw_mode,
    },
    event::{
        poll,
        read,
        Event,
        KeyCode,
        KeyModifiers,
    },
}; 

fn main() -> Result<()> {
    while let Ok(run) = enable_raw_mode() {
        if poll(Duration::from_millis(500))? {
            match read()? {
                Event::Key(event) => {
                    writeln!(stdout(), "{:?}\n", event);
                    match (event.code, event.modifiers) {
                        (KeyCode::Char('c'), KeyModifiers::CONTROL) => break,
                        _ => continue,
                    };
                },
                _ => continue,
            };
        } else {
            // nothing
        }
    }

    Ok(())
}
