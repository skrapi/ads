use std::fmt::Display;

/// Types of time complexity
pub enum TimeComplexity {
    ConstantTime,
    LinearTime,
    LogarithmicTime,
}

impl Display for TimeComplexity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimeComplexity::ConstantTime => write!(f, "Constant Time O(1)"),
            TimeComplexity::LinearTime => write!(f, "Linear Time O(n)"),
            TimeComplexity::LogarithmicTime => write!(f, "Logarithmic Time O(log n)"),
        }
    }
}
#[derive(Clone, Copy)]
pub enum Answer {
    One = 1,
    Two = 2,
    Three = 3,
}

impl Answer {
    pub fn increment(&mut self) {
        *self = match self {
            Answer::One => Answer::Two,
            Answer::Two => Answer::Three,
            Answer::Three => Answer::Three,
        }
    }

    pub fn decrement(&mut self) {
        *self = match self {
            Answer::One => Answer::One,
            Answer::Two => Answer::One,
            Answer::Three => Answer::Two,
        }
    }
}
