use super::Row;

use std::io::{Write, stdout};
use crossterm::{
    ExecutableCommand,
    event::{
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
    },
};

#[derive(Debug, PartialEq)]
pub enum KeyHandle {
    WRITE,
    EXIT,
    CONTINUE,
}

pub struct Editor {
    row: [Row; 10],
}

impl Editor {
    pub fn handle_key(event: KeyEvent) -> KeyHandle {
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
}


#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{
        KeyEventKind,
        KeyEventState,
    };

    #[test]
    fn handle_key_returns_rendered_char() {
        let press = KeyEvent {
            code: KeyCode::Char('a'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        };

        assert_eq!(Editor::handle_key(press), KeyHandle::WRITE);
    }
}