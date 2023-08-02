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
        position,
        SavePosition,
        RestorePosition,
        MoveTo,
        EnableBlinking,
        DisableBlinking,
    }
}; 

pub struct Terminal{
    pub col: u16,
    pub row: u16,
}

impl Terminal {
    pub fn run(&mut self) -> Result<()> {
        let current_pos = position()?;
        self.col = current_pos.0;
        self.row = current_pos.1;
        
        while let Ok(_run) = enable_raw_mode() {
            execute!(
                stdout(), 
                DisableBlinking,
                EnableBlinking,
            )?;
            
            if poll(Duration::from_millis(500))? {
                match read()? {
                    Event::Key(event) => {
                        match (event.code, event.modifiers) {
                            (KeyCode::Char('c'), KeyModifiers::CONTROL) => break,
                            (KeyCode::Backspace, _) => {
                                write!(stdout(), "\u{8}")?;
                                self.col -= 1;
                                execute!(
                                    stdout(),
                                    MoveTo(self.col, self.row),
                                )?;
                            },
                            (KeyCode::Char(c), _) => {
                                write!(stdout(), "{}", c)?;
                                self.col += 1;
                                execute!(
                                    stdout(),
                                    MoveTo(self.col, self.row),
                                )?;
                            },
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
