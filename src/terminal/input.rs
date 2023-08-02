use std::io::{
    Write, 
    stdout,
};
use std::time::Duration;
use crossterm::{
    Result,
    execute,
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
    cursor::{
        SavePosition,
        RestorePosition,
        EnableBlinking,
    }
}; 

pub struct Terminal;

impl Terminal {
    pub fn run(&self) -> Result<()> {
        execute!(
            stdout(),
            SavePosition,
            EnableBlinking,
        )?;
        
        while let Ok(_run) = enable_raw_mode() {
            execute!(stdout(), RestorePosition)?;
            if poll(Duration::from_millis(500))? {
                match read()? {
                    Event::Key(event) => {
                        writeln!(stdout(), "{:?}\n", event)?;
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
        
        disable_raw_mode()
    }
}
