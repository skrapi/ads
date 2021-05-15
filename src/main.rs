mod answers;
use core::time;
use std::io::{stdin, stdout, Write};
use std::{fmt::Display, thread};

use termion::event::{Event, Key};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;

use answers::{Answer, TimeComplexity};

/// Defines that type of test
#[derive(Debug)]
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
            "{}{}Question: {}\n\r\
             {} 1. {}\n\r\
             {} 2. {}\n\r\
             {} 3. {}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            question,
            selected_answer[0],
            possible_answers[0],
            selected_answer[1],
            possible_answers[1],
            selected_answer[2],
            possible_answers[2]
        )
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum State {
    Entry,
    Runtime,
    Exit,
}

#[derive(Debug, PartialEq)]
enum Area {
    Greeting(State),
    Testing(State),
    Completed(State),
    Exit(State),
}

impl Area {
    fn generate_output_string(&mut self, key: Key) -> String {
        let mut output_string = String::new();
        match self {
            Area::Greeting(state) => match state {
                State::Entry => {
                    output_string = format!(
                        "{}{}{}Hello, you have selected: {}\
                                {}Use <q> to exit \
                                {}Use arrow keys to move around\
                                {}Press ENTER to begin",
                        termion::clear::All,
                        termion::cursor::Hide,
                        termion::cursor::Goto(1, 1),
                        TestType::All,
                        termion::cursor::Goto(1, 2),
                        termion::cursor::Goto(1, 3),
                        termion::cursor::Goto(1, 4),
                    );

                    *self = Area::Greeting(State::Exit);
                }
                State::Runtime => match key {
                    Key::Char('\n') => {
                        output_string = Question::generate(Answer::One);
                        *self = Area::Testing(State::Entry);
                    }
                    Key::Char('q') => *self = Area::Exit(State::Exit),
                    _ => {}
                },
                State::Exit => *self = Area::Testing(State::Entry),
            },
            Area::Testing(_) => {
                let mut current_answer = Answer::One;

                match key {
                    Key::Char('\n') => output_string = Question::generate(current_answer),
                    Key::Down => {
                        current_answer.increment();
                        output_string = Question::generate(current_answer)
                    }
                    Key::Up => {
                        current_answer.decrement();
                        output_string = Question::generate(current_answer)
                    }
                    Key::Char('q') => *self = Area::Exit(State::Exit),
                    _ => {}
                }
            }
            _ => {}
        }

        return output_string;
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdin_keys = termion::async_stdin().keys();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut area = Area::Greeting(State::Entry);

    while area != Area::Exit(State::Exit) {
        // Get input
        let key = match stdin_keys.next() {
            Some(Ok(key)) => key,
            _ => Key::Null,
        };

        let output_string = area.generate_output_string(key);

        // print to output
        write!(stdout, "{}", output_string)?;
        stdout.flush()?;

        thread::sleep(time::Duration::from_millis(50));
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
