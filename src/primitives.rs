use crate::{invalid_value, Validate, Validation};
use std::borrow::Cow;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Length {
    Max(usize),
    Min(usize),
    MinMax(usize, usize),
}

impl Validate<Length> for String {
    fn validate(
        self,
        field_name: impl Into<Cow<'static, str>>,
        constraint: &Length,
    ) -> Validation<Self> {
        let length = self.len();
        let maybe_violation = match *constraint {
            Length::Max(max) => {
                if length <= max {
                    Some("invalid.length.min")
                } else {
                    None
                }
            }
            Length::Min(min) => {
                if length >= min {
                    Some("invalid.length.min")
                } else {
                    None
                }
            }
            Length::MinMax(min, max) => {
                if length < min {
                    Some("invalid.length.min")
                } else if length > max {
                    Some("invalid.length.max")
                } else {
                    None
                }
            }
        };
        if let Some(code) = maybe_violation {
            Validation::Failure(vec![invalid_value(code.into(), field_name.into(), self)])
        } else {
            Validation::Success(self)
        }
    }
}
