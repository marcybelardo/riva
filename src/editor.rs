use std::{io::stdout, time::Duration};
use crossterm::{
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
        KeyEvent,
        KeyCode,
        KeyModifiers,
    },
};

pub struct Editor;

impl Drop for Editor {
    fn drop(&mut self) {
        disable_raw_mode().expect("Could not disable raw mode");
        execute!(stdout(), LeaveAlternateScreen).expect("Error");
    }
}

impl Editor {
    pub fn new() -> Self {
        enable_raw_mode().expect("Could not enable raw mode");
        execute!(stdout(), EnterAlternateScreen).expect("Err");

        Self
    }

    pub fn run(&mut self) -> crossterm::Result<bool> {
        self.process_key()
    }

    fn process_key(&self) -> crossterm::Result<bool> {
        match self.read_key()? {
            KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => return Ok(false),
            _ => {},
        }

        Ok(true)
    }

    fn read_key(&self) -> crossterm::Result<KeyEvent> {
        loop {
            if poll(Duration::from_millis(500))? {
                if let Event::Key(event) = read()? {
                    return Ok(event);
                }
            }
        }
    }
}

