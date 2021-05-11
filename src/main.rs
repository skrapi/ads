use std::{
    fmt::Display,
    io::{BufRead, StdoutLock},
    thread::sleep,
};

use std::io::{stdin, stdout, Write};

use std::time::Duration;

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
    let stdout = stdout();
    let mut handle_stdout = stdout.lock();
    let mut handle_stdin = stdin.lock();

    handle_stdout.write_all(b"Hello\n").unwrap();
    handle_stdout.flush()?;

    for n in 0..10 {
        let mut buf = Vec::new();
        let mut read = String::new();
        write!(buf, "{}\n", n)?;
        write_to_terminal(&mut handle_stdout, buf)?;
        handle_stdin.read_line(&mut read)?;
        println!("{}", read);
    }

    clear_terminal(&mut handle_stdout)?;

    Ok(())
}
