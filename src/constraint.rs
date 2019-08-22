use crate::{invalid_relation, invalid_value, Validate, Validation, Value};
use std::borrow::Cow;

pub const INVALID_LENGTH_EXACT: &str = "invalid.length.exact";
pub const INVALID_LENGTH_MAX: &str = "invalid.length.max";
pub const INVALID_LENGTH_MIN: &str = "invalid.length.min";

pub const INVALID_CHAR_COUNT_MAX: &str = "invalid.char.count.max";
pub const INVALID_CHAR_COUNT_MIN: &str = "invalid.char.count.min";

pub const INVALID_BOUND_EXACT: &str = "invalid.bound.exact";
pub const INVALID_BOUND_CLOSED_MAX: &str = "invalid.bound.closed.max";
pub const INVALID_BOUND_CLOSED_MIN: &str = "invalid.bound.closed.min";
pub const INVALID_BOUND_OPEN_MAX: &str = "invalid.bound.open.max";
pub const INVALID_BOUND_OPEN_MIN: &str = "invalid.bound.open.min";

pub const INVALID_CONTAINS_ELEMENT: &str = "invalid.contains.element";

pub const INVALID_MUST_MATCH: &str = "invalid.must.match";

pub const INVALID_FROM_TO_INCLUSIVE: &str = "invalid.from.to.inclusive";
pub const INVALID_FROM_TO_EXCLUSIVE: &str = "invalid.from.to.exclusive";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Length {
    Exact(usize),
    Max(usize),
    Min(usize),
    MinMax(usize, usize),
}

impl Length {
    pub fn validate(&self, length: usize) -> Option<(&'static str, usize)> {
        match *self {
            Length::Exact(exact_len) => {
                if length != exact_len {
                    Some((INVALID_LENGTH_EXACT, exact_len))
                } else {
                    None
                }
            }
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

pub trait HasLength {
    fn length(&self) -> usize;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharCount {
    Max(usize),
    Min(usize),
    MinMax(usize, usize),
}

impl CharCount {
    pub fn validate(&self, char_count: usize) -> Option<(&'static str, usize)> {
        match *self {
            CharCount::Max(max) => {
                if char_count > max {
                    Some((INVALID_CHAR_COUNT_MAX, max))
                } else {
                    None
                }
            }
            CharCount::Min(min) => {
                if char_count < min {
                    Some((INVALID_CHAR_COUNT_MIN, min))
                } else {
                    None
                }
            }
            CharCount::MinMax(min, max) => {
                if char_count < min {
                    Some((INVALID_LENGTH_MIN, min))
                } else if char_count > max {
                    Some((INVALID_CHAR_COUNT_MAX, max))
                } else {
                    None
                }
            }
        }
    }
}

pub trait HasCharCount {
    fn char_count(&self) -> usize;
}

impl<T> Validate<CharCount> for T
where
    T: HasCharCount,
{
    fn validate(
        self,
        name: impl Into<Cow<'static, str>>,
        constraint: &CharCount,
    ) -> Validation<Self> {
        if let Some((code, expected)) = constraint.validate(self.char_count()) {
            Validation::Failure(vec![invalid_value(code, name, self.char_count(), expected)])
        } else {
            Validation::Success(self)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bound<T> {
    Exact(T),
    ClosedRange(T, T),
    ClosedOpenRange(T, T),
    OpenClosedRange(T, T),
    OpenRange(T, T),
}

impl<T> Bound<T>
where
    T: Eq + Ord + Clone,
{
    pub fn validate(&self, value: &T) -> Option<(&'static str, T)> {
        match self {
            Bound::Exact(bound) => {
                if bound != value {
                    Some((INVALID_BOUND_EXACT, bound.clone()))
                } else {
                    None
                }
            }
            Bound::ClosedRange(min, max) => {
                if value < min {
                    Some((INVALID_BOUND_CLOSED_MIN, min.clone()))
                } else if value > max {
                    Some((INVALID_BOUND_CLOSED_MAX, max.clone()))
                } else {
                    None
                }
            }
            Bound::ClosedOpenRange(min, max) => {
                if value < min {
                    Some((INVALID_BOUND_CLOSED_MIN, min.clone()))
                } else if value >= max {
                    Some((INVALID_BOUND_OPEN_MAX, max.clone()))
                } else {
                    None
                }
            }
            Bound::OpenClosedRange(min, max) => {
                if value <= min {
                    Some((INVALID_BOUND_OPEN_MIN, min.clone()))
                } else if value > max {
                    Some((INVALID_BOUND_CLOSED_MAX, max.clone()))
                } else {
                    None
                }
            }
            Bound::OpenRange(min, max) => {
                if value <= min {
                    Some((INVALID_BOUND_OPEN_MIN, min.clone()))
                } else if value >= max {
                    Some((INVALID_BOUND_OPEN_MAX, max.clone()))
                } else {
                    None
                }
            }
        }
    }
}

impl<T> Validate<Bound<T>> for T
where
    T: Eq + Ord + Clone,
    Value: From<T>,
{
    fn validate(
        self,
        name: impl Into<Cow<'static, str>>,
        constraint: &Bound<T>,
    ) -> Validation<Self> {
        if let Some((code, expected)) = constraint.validate(&self) {
            Validation::Failure(vec![invalid_value(code, name, self, expected)])
        } else {
            Validation::Success(self)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Contains<'a, E> {
    Element(&'a E),
}

pub trait HasElement<T> {
    fn has_element(&self, element: &T) -> bool;
}

impl<'a, E> Contains<'a, E>
where
    E: Clone,
{
    pub fn validate<T>(&self, value: &T) -> Option<(&'static str, E)>
    where
        T: HasElement<E>,
    {
        match *self {
            Contains::Element(element) => {
                if value.has_element(element) {
                    None
                } else {
                    Some((INVALID_CONTAINS_ELEMENT, element.clone()))
                }
            }
        }
    }
}

impl<'a, T, A> Validate<Contains<'a, A>> for T
where
    T: HasElement<A>,
    A: Clone,
    Value: From<A> + From<T>,
{
    fn validate(
        self,
        name: impl Into<Cow<'static, str>>,
        constraint: &Contains<'a, A>,
    ) -> Validation<Self> {
        if let Some((code, expected)) = constraint.validate(&self) {
            Validation::Failure(vec![invalid_value(code, name, self, expected)])
        } else {
            Validation::Success(self)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MustMatch(pub &'static str, pub &'static str);

impl MustMatch {
    pub fn validate<T>(&self, value1: &T, value2: &T) -> Option<(&'static str, ())>
    where
        T: Eq,
    {
        if value1 == value2 {
            None
        } else {
            Some((INVALID_MUST_MATCH, ()))
        }
    }
}

impl<T> Validate<MustMatch> for (T, T)
where
    T: Eq,
    Value: From<T>,
{
    fn validate(
        self,
        _name: impl Into<Cow<'static, str>>,
        constraint: &MustMatch,
    ) -> Validation<Self> {
        if let Some((code, _)) = constraint.validate(&self.0, &self.1) {
            Validation::Failure(vec![invalid_relation(
                code,
                constraint.0,
                self.0,
                constraint.1,
                self.1,
            )])
        } else {
            Validation::Success(self)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FromTo {
    Inclusive(&'static str, &'static str),
    Exclusive(&'static str, &'static str),
}

impl FromTo {
    pub fn validate<T>(&self, value1: &T, value2: &T) -> Option<(&'static str, ())>
    where
        T: Eq + Ord,
    {
        match *self {
            FromTo::Inclusive(_, _) => {
                if value1 <= value2 {
                    None
                } else {
                    Some((INVALID_FROM_TO_INCLUSIVE, ()))
                }
            }
            FromTo::Exclusive(_, _) => {
                if value1 < value2 {
                    None
                } else {
                    Some((INVALID_FROM_TO_EXCLUSIVE, ()))
                }
            }
        }
    }
}

impl<T> Validate<FromTo> for (T, T)
where
    T: Eq + Ord,
    Value: From<T>,
{
    fn validate(
        self,
        _name: impl Into<Cow<'static, str>>,
        constraint: &FromTo,
    ) -> Validation<Self> {
        let (name1, name2) = match *constraint {
            FromTo::Inclusive(name1, name2) => (name1, name2),
            FromTo::Exclusive(name1, name2) => (name1, name2),
        };

        if let Some((code, _)) = constraint.validate(&self.0, &self.1) {
            Validation::Failure(vec![invalid_relation(code, name1, self.0, name2, self.1)])
        } else {
            Validation::Success(self)
        }
    }
}
