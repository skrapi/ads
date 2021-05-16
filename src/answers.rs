use std::fmt::Display;

/// Types of time complexity
#[derive(Debug, Clone, Copy, PartialEq)]
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

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Selection {
    One = 1,
    Two = 2,
    Three = 3,
}

impl Display for Selection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as usize)
    }
}

impl Selection {
    pub fn increment(&mut self) {
        *self = match self {
            Selection::One => Selection::Two,
            Selection::Two => Selection::Three,
            Selection::Three => Selection::Three,
        }
    }

    pub fn decrement(&mut self) {
        *self = match self {
            Selection::One => Selection::One,
            Selection::Two => Selection::One,
            Selection::Three => Selection::Two,
        }
    }
}
