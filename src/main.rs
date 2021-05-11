use std::fmt::Display;

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
            TestType::All => write!(f, "All tests"),
        }
    }
}

fn main() {
    let args = TestType::from_args();
    println!("{}", args);
}
