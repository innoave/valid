use crate::{invalid_value, Validate, Validation};
use std::borrow::Cow;

pub const INVALID_LENGTH_MAX: &str = "invalid.length.max";
pub const INVALID_LENGTH_MIN: &str = "invalid.length.min";

pub trait HasLength {
    fn length(&self) -> usize;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Length {
    Max(usize),
    Min(usize),
    MinMax(usize, usize),
}

impl Length {
    pub fn validate(&self, length: usize) -> Option<(&'static str, usize)> {
        match *self {
            Length::Max(max) => {
                if length > max {
                    Some((INVALID_LENGTH_MAX, max))
                } else {
                    None
                }
            }
            Length::Min(min) => {
                if length < min {
                    Some((INVALID_LENGTH_MIN, min))
                } else {
                    None
                }
            }
            Length::MinMax(min, max) => {
                if length < min {
                    Some((INVALID_LENGTH_MIN, min))
                } else if length > max {
                    Some((INVALID_LENGTH_MAX, max))
                } else {
                    None
                }
            }
        }
    }
}

impl<T> Validate<Length> for T
where
    T: HasLength,
{
    fn validate(self, name: impl Into<Cow<'static, str>>, constraint: &Length) -> Validation<Self> {
        if let Some((code, expected)) = constraint.validate(self.length()) {
            Validation::Failure(vec![invalid_value(code, name, self.length(), expected)])
        } else {
            Validation::Success(self)
        }
    }
}

impl HasLength for String {
    fn length(&self) -> usize {
        self.len()
    }
}

impl HasLength for &str {
    fn length(&self) -> usize {
        self.len()
    }
}

impl<T> HasLength for Vec<T> {
    fn length(&self) -> usize {
        self.len()
    }
}

impl<T> HasLength for &[T] {
    fn length(&self) -> usize {
        self.len()
    }
}
