use crate::terminal::editor::{Editor, KeyHandle};

use std::io::stdout;
use std::time::Duration;
use crossterm::{
    Result,
    execute,
    terminal::{
        enable_raw_mode,
        disable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    event::{
        poll,
        read,
        Event,
    },
    cursor::{
        EnableBlinking,
        DisableBlinking,
    }
};

pub struct Terminal;

impl Terminal {
    pub fn run(&mut self) -> Result<()> {
        execute!(
            stdout(),
            EnterAlternateScreen,
        )?;

        enable_raw_mode()?;
        
        loop {
            execute!(
                stdout(), 
                DisableBlinking,
                EnableBlinking,
            )?;
            
            if poll(Duration::from_millis(500))? {
                if let Event::Key(event) = read()? {
                    match Editor::handle_key(event) {
                        KeyHandle::EXIT => break,
                        KeyHandle::CONTINUE | KeyHandle::WRITE => continue,
                    }
                } else {
                    continue;
                }
            } else {
                // nothing
            }
        }

        execute!(
            stdout(),
            LeaveAlternateScreen,
        )?;

        disable_raw_mode()
    }
}
