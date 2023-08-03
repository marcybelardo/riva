use std::io::{
    Write, 
    stdout,
};
use std::time::Duration;
use crossterm::{
    Result,
    ExecutableCommand,
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
    cursor::{
        MoveUp,
        MoveDown,
        MoveLeft,
        MoveRight,
        MoveToNextLine,
        EnableBlinking,
        DisableBlinking,
    }
};

enum KeyHandle {
    WRITE,
    EXIT,
    CONTINUE,
}

pub struct Terminal {
    pub current_col: u16,
    pub current_row: u16,
}

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
                    match handle_key(event) {
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

fn handle_key(event: KeyEvent) -> KeyHandle {
    match (event.code, event.modifiers) {
        (KeyCode::Char('c'), KeyModifiers::CONTROL) => return KeyHandle::EXIT,
        (KeyCode::Enter, _) => {
            stdout().execute(MoveToNextLine(1)).unwrap();
            return KeyHandle::WRITE;
        },
        (KeyCode::Backspace, _) => {
            stdout().execute(MoveLeft(1)).unwrap();
            stdout().write_all(format!(" ").as_bytes()).unwrap();
            stdout().execute(MoveLeft(1)).unwrap();
            return KeyHandle::WRITE;
        },
        (KeyCode::Char(c), _) => {
            stdout().write_all(format!("{c}").as_bytes()).unwrap();
            return KeyHandle::WRITE;
        },
        (KeyCode::Up, _) => {
            stdout().execute(MoveUp(1)).unwrap();
            return KeyHandle::WRITE;
        },
        (KeyCode::Down, _) => {
            stdout().execute(MoveDown(1)).unwrap();
            return KeyHandle::WRITE;
        },
        (KeyCode::Left, _) => {
            stdout().execute(MoveLeft(1)).unwrap();
            return KeyHandle::WRITE;
        },
        (KeyCode::Right, _) => {
            stdout().execute(MoveRight(1)).unwrap();
            return KeyHandle::WRITE;
        }
        _ => KeyHandle::CONTINUE,
    }
}
