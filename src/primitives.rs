use crate::{invalid_value, Validate, Validation};
use std::borrow::Cow;

pub const INVALID_LENGTH_MAX: &str = "invalid.length.max";
pub const INVALID_LENGTH_MIN: &str = "invalid.length.min";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Length {
    Max(usize),
    Min(usize),
    MinMax(usize, usize),
}

impl Length {
    pub fn validate(&self, length: usize) -> Option<&'static str> {
        match *self {
            Length::Max(max) => {
                if length > max {
                    Some(INVALID_LENGTH_MAX)
                } else {
                    None
                }
            }
            Length::Min(min) => {
                if length < min {
                    Some(INVALID_LENGTH_MIN)
                } else {
                    None
                }
            }
            Length::MinMax(min, max) => {
                if length < min {
                    Some(INVALID_LENGTH_MIN)
                } else if length > max {
                    Some(INVALID_LENGTH_MAX)
                } else {
                    None
                }
            }
        }
    }
}

impl Validate<Length> for String {
    fn validate(self, name: impl Into<Cow<'static, str>>, constraint: &Length) -> Validation<Self> {
        if let Some(code) = constraint.validate(self.len()) {
            Validation::Failure(vec![invalid_value(code, name, self.len())])
        } else {
            Validation::Success(self)
        }
    }
}

impl<T> Validate<Length> for Vec<T> {
    fn validate(self, name: impl Into<Cow<'static, str>>, constraint: &Length) -> Validation<Self> {
        if let Some(code) = constraint.validate(self.len()) {
            Validation::Failure(vec![invalid_value(code, name, self.len())])
        } else {
            Validation::Success(self)
        }
    }
}
