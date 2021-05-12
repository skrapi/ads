use std::{
    fmt::Display,
    io::{BufRead, StdoutLock},
    thread::sleep,
};

use std::io::{stdin, stdout, Write};

use std::time::Duration;

use termion::event::{Event, Key, MouseEvent};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;

/// Defines that type of test
enum TestType {
    /// Selects all tests
    All,
}

impl Display for TestType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TestType::All => write!(f, "All tests"),
        }
    }
}

fn clear_terminal(stdout_handle: &mut StdoutLock) -> Result<(), Box<dyn std::error::Error>> {
    let mut clear_character = Vec::new();
    write!(clear_character, "{esc}c", esc = 27 as char)?;
    stdout_handle.write_all(&clear_character)?;
    Ok(())
}

fn write_to_terminal(
    stdout_handle: &mut StdoutLock,
    buf: Vec<u8>,
) -> Result<(), Box<dyn std::error::Error>> {
    clear_terminal(stdout_handle)?;
    stdout_handle.write_all(&buf)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = stdin();
    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());

    write!(
        stdout,
        "{}{}{}Hello, q to exit, arrow keys to move around",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Hide
    )
    .unwrap();
    stdout.flush().unwrap();

    for character in stdin.events() {
        let event = character?;
        match event {
            Event::Key(Key::Char('q')) => break,
            Event::Key(Key::Down) => write!(
                stdout,
                "{}{}Down",
                termion::clear::All,
                termion::cursor::Goto(1, 1)
            )?,
            Event::Key(Key::Up) => write!(
                stdout,
                "{}{}Up",
                termion::clear::All,
                termion::cursor::Goto(1, 1)
            )?,
            Event::Key(Key::Left) => write!(
                stdout,
                "{}{}Left",
                termion::clear::All,
                termion::cursor::Goto(1, 1)
            )?,
            Event::Key(Key::Right) => write!(
                stdout,
                "{}{}Right",
                termion::clear::All,
                termion::cursor::Goto(1, 1)
            )?,
            _ => {}
        }
        stdout.flush().unwrap();
    }

    write!(
        stdout,
        "{}{}{}Bye\n",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Show
    )?;

    Ok(())
}
