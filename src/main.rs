mod answers;
use std::fmt::Display;
use std::io::{stdin, stdout, Write};

use termion::event::{Event, Key};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;

use structopt::StructOpt;

use answers::{Answer, TimeComplexity};

/// Defines that type of test
#[derive(Debug, StructOpt)]
enum TestType {
    /// Selects all tests
    All,
    LinkedLists,
}

impl Display for TestType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TestType::All => write!(f, "Test all subject matter"),
            TestType::LinkedLists => write!(f, "Testing linked list knowledge"),
        }
    }
}

struct Question;

impl Question {
    fn generate(selection: Answer) -> String {
        let possible_answers = vec![
            TimeComplexity::ConstantTime,
            TimeComplexity::LogarithmicTime,
            TimeComplexity::LinearTime,
        ];

        let selected_answer = match selection {
            Answer::One => [">", " ", " "],
            Answer::Two => [" ", ">", " "],
            Answer::Three => [" ", " ", ">"],
        };

        let question =
            "What is the time complexity for inserting an element at the head of a linked list?";

        format!(
            "{}{}{}Question: {}\
            {} {} 1. {}\
            {} {} 2. {}\
            {} {} 3. {}",
            termion::clear::All,
            termion::cursor::Hide,
            termion::cursor::Goto(1, 1),
            question,
            termion::cursor::Goto(1, 2),
            selected_answer[0],
            possible_answers[0],
            termion::cursor::Goto(1, 3),
            selected_answer[1],
            possible_answers[1],
            termion::cursor::Goto(1, 4),
            selected_answer[2],
            possible_answers[2]
        )
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = TestType::from_args();
    let stdin = stdin();
    let mut stdout = MouseTerminal::from(stdout().into_raw_mode()?);

    let hello_message = format!(
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
    );
    write!(stdout, "{}", hello_message)?;

    stdout.flush()?;

    let mut current_answer = Answer::One;

    for character in stdin.events() {
        let event = character?;
        match event {
            Event::Key(Key::Char('\n')) => {
                write!(stdout, "{}", Question::generate(current_answer))?
            }
            Event::Key(Key::Down) => {
                current_answer.increment();
                write!(stdout, "{}", Question::generate(current_answer))?
            }
            Event::Key(Key::Up) => {
                current_answer.decrement();
                write!(stdout, "{}", Question::generate(current_answer))?
            }
            Event::Key(Key::Char('q')) => break,
            _ => {}
        }
        stdout.flush()?;
    }

    // Clean up

    write!(
        stdout,
        "{}{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Show
    )?;

    stdout.flush()?;

    Ok(())
}
