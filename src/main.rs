mod answers;
use std::{
    fmt,
    io::{stdout, Write},
    time,
};
use std::{fmt::Display, thread};

use termion::event::Key;
use termion::{input::TermRead, raw::IntoRawMode};

use answers::{Selection, TimeComplexity};

/// Defines that type of test
#[derive(Debug, PartialEq, Clone, Copy, PartialOrd)]
enum TestType {
    /// Selects all tests
    All,
    LinkedLists,
    HashTables,
}

impl TestType {
    fn list_all() -> Vec<TestType> {
        vec![TestType::All, TestType::LinkedLists, TestType::HashTables]
    }
    fn generate(selection: TestType) -> String {
        let possible_answers = TestType::list_all();

        let selected_answer = match selection {
            TestType::All => [">", " ", " "],
            TestType::LinkedLists => [" ", ">", " "],
            TestType::HashTables => [" ", " ", ">"],
        };

        format!(
            "{}{}Which test would you like to take? \n\r\
             {} 1. {}\n\r\
             {} 2. {}\n\r\
             {} 3. {}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            selected_answer[0],
            possible_answers[0],
            selected_answer[1],
            possible_answers[1],
            selected_answer[2],
            possible_answers[2]
        )
    }
    fn increment(&mut self) {
        *self = match self {
            TestType::All => TestType::LinkedLists,
            TestType::LinkedLists => TestType::HashTables,
            TestType::HashTables => TestType::HashTables,
        }
    }

    fn decrement(&mut self) {
        *self = match self {
            TestType::All => TestType::All,
            TestType::LinkedLists => TestType::All,
            TestType::HashTables => TestType::LinkedLists,
        }
    }
}

impl Display for TestType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TestType::All => write!(f, "Test all subject matter"),
            TestType::LinkedLists => write!(f, "Testing linked list knowledge"),
            TestType::HashTables => write!(f, "Testing hash table knowledge"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Question {
    question: String,
    correct_answer: TimeComplexity,
    selected_answer: Selection,
    options: [TimeComplexity; 3],
}

impl Question {
    fn new(question: String, correct_answer: TimeComplexity, options: [TimeComplexity; 3]) -> Self {
        Question {
            question,
            correct_answer,
            selected_answer: Selection::One,
            options,
        }
    }
    fn generate(&self) -> String {
        let selected_answer = match self.selected_answer {
            Selection::One => [">", " ", " "],
            Selection::Two => [" ", ">", " "],
            Selection::Three => [" ", " ", ">"],
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
            self.options[0],
            selected_answer[1],
            self.options[1],
            selected_answer[2],
            self.options[2]
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
    TestSelection(State, TestType),
    Testing(State, usize, Vec<Question>),
    Completed(State),
    Exit(State),
}

impl Area {
    fn generate_output_string(&mut self, key: Key) -> String {
        let mut output_string = String::new();

        if key == Key::Char('q') {
            *self = Area::Exit(State::Exit);
        } else {
            match self {
                Area::Greeting(state) => match state {
                    State::Entry => {
                        output_string = format!(
                            "{}{}{}Hello, welcome to testing\n\r\
                            Use <q> to exit \n\r\
                            Use arrow keys to move around\n\r\
                            Press ENTER to begin",
                            termion::clear::All,
                            termion::cursor::Hide,
                            termion::cursor::Goto(1, 1),
                        );

                        *self = Area::Greeting(State::Runtime);
                    }
                    State::Runtime => match key {
                        Key::Char('\n') => {
                            *self = Area::Greeting(State::Exit);
                        }
                        _ => {}
                    },
                    State::Exit => *self = Area::TestSelection(State::Entry, TestType::All),
                },
                Area::TestSelection(state, selection) => match state {
                    State::Entry => {
                        output_string = TestType::generate(*selection);
                        *state = State::Runtime
                    }
                    State::Runtime => match key {
                        Key::Char('\n') => *state = State::Exit,
                        Key::Down => {
                            selection.increment();
                            output_string = TestType::generate(*selection);
                        }
                        Key::Up => {
                            selection.decrement();
                            output_string = TestType::generate(*selection);
                        }
                        _ => {}
                    },
                    State::Exit => {
                        output_string = format!(
                            "{}{}Hello, you have selected: {}\n\r\
                            Press ENTER to begin",
                            termion::clear::All,
                            termion::cursor::Goto(1, 1),
                            selection,
                        );
                        if key == Key::Char('\n') {
                            *self = Area::Testing(State::Entry, 0, Vec::new())
                        }
                    }
                },
                Area::Testing(state, current_question_index, questions) => match state {
                    State::Entry => {
                        output_string = questions[*current_question_index].generate();
                        *state = State::Runtime
                    }
                    State::Runtime => match key {
                        Key::Char('\n') => {
                            *current_question_index += 1;
                            if *current_question_index < questions.len() {
                                output_string = questions[*current_question_index].generate();
                            } else {
                                *state = State::Exit;
                            }
                        }
                        Key::Down => {
                            questions[*current_question_index]
                                .selected_answer
                                .increment();
                            output_string = questions[*current_question_index].generate();
                        }
                        Key::Up => {
                            questions[*current_question_index]
                                .selected_answer
                                .decrement();
                            output_string = questions[*current_question_index].generate();
                        }
                        _ => {}
                    },
                    State::Exit => {
                        output_string = format!(
                            "{}{}{}",
                            termion::clear::All,
                            termion::cursor::Goto(1, 1),
                            questions
                                .iter()
                                .map(|x| x.selected_answer)
                                .collect::<Selection>()
                        );
                        if key == Key::Char('\n') {
                            *self = Area::Exit(State::Exit);
                        }
                    }
                },
                _ => {}
            }
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
