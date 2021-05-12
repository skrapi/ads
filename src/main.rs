use std::fmt::Display;
use std::io::{stdin, stdout, Write};

use termion::event::{Event, Key};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;

use structopt::StructOpt;

/// Defines that type of test
#[derive(Debug, StructOpt)]
enum TestType {
    /// Selects all tests
    All,
}

impl Display for TestType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TestType::All => write!(f, "Test all subject matter"),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = TestType::from_args();
    let stdin = stdin();
    let mut stdout = MouseTerminal::from(stdout().into_raw_mode()?);

    write!(
        stdout,
        "{}{}{}Hello, you have selected: {}\
        {}Use <q> to exit \
        {}Use arrow keys to move around\
        {}Press ENTER to begin",
        termion::clear::All,
        termion::cursor::Hide,
        termion::cursor::Goto(1, 1),
        args,
        termion::cursor::Goto(1, 2),
        termion::cursor::Goto(1, 3),
        termion::cursor::Goto(1, 4),
    )?;

    stdout.flush()?;

    // while let Some(begin_character) = stdin.events().next()? {}

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
        stdout.flush()?;
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
