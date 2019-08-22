use crate::{invalid_optional_value, invalid_relation, invalid_value, Validate, Validation, Value};
use std::borrow::Cow;

pub const INVALID_ASSERT_TRUE: &str = "invalid.assert.true";
pub const INVALID_ASSERT_FALSE: &str = "invalid.assert.false";

pub const INVALID_NOT_EMPTY: &str = "invalid.not.empty";

pub const INVALID_LENGTH_EXACT: &str = "invalid.length.exact";
pub const INVALID_LENGTH_MAX: &str = "invalid.length.max";
pub const INVALID_LENGTH_MIN: &str = "invalid.length.min";

pub const INVALID_CHAR_COUNT_EXACT: &str = "invalid.char.count.exact";
pub const INVALID_CHAR_COUNT_MAX: &str = "invalid.char.count.max";
pub const INVALID_CHAR_COUNT_MIN: &str = "invalid.char.count.min";

pub const INVALID_BOUND_EXACT: &str = "invalid.bound.exact";
pub const INVALID_BOUND_MAX: &str = "invalid.bound.max";
pub const INVALID_BOUND_MIN: &str = "invalid.bound.min";

pub const INVALID_CONTAINS_ELEMENT: &str = "invalid.contains.element";

pub const INVALID_MUST_MATCH: &str = "invalid.must.match";

pub const INVALID_FROM_TO_INCLUSIVE: &str = "invalid.from.to.inclusive";
pub const INVALID_FROM_TO_EXCLUSIVE: &str = "invalid.from.to.exclusive";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AssertTrue;

impl AssertTrue {
    pub fn validate(&self, value: bool) -> Option<(&'static str, ())> {
        if value {
            None
        } else {
            Some((INVALID_ASSERT_TRUE, ()))
        }
    }
}

impl Validate<AssertTrue> for bool {
    fn validate(
        self,
        name: impl Into<Cow<'static, str>>,
        constraint: &AssertTrue,
    ) -> Validation<Self> {
        if let Some((code, ())) = constraint.validate(self) {
            Validation::Failure(vec![invalid_value(code, name, self, true)])
        } else {
            Validation::Success(self)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AssertFalse;

impl AssertFalse {
    pub fn validate(&self, value: bool) -> Option<(&'static str, ())> {
        if value {
            Some((INVALID_ASSERT_FALSE, ()))
        } else {
            None
        }
    }
}

impl Validate<AssertFalse> for bool {
    fn validate(
        self,
        name: impl Into<Cow<'static, str>>,
        constraint: &AssertFalse,
    ) -> Validation<Self> {
        if let Some((code, ())) = constraint.validate(self) {
            Validation::Failure(vec![invalid_value(code, name, self, false)])
        } else {
            Validation::Success(self)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NotEmpty;

impl NotEmpty {
    pub fn validate<T>(&self, value: &T) -> Option<(&'static str, ())>
    where
        T: IsEmptyValue,
    {
        if value.is_empty_value() {
            Some((INVALID_NOT_EMPTY, ()))
        } else {
            None
        }
    }
}

pub trait IsEmptyValue {
    fn is_empty_value(&self) -> bool;
}

impl<T> Validate<NotEmpty> for T
where
    T: IsEmptyValue,
{
    fn validate(
        self,
        name: impl Into<Cow<'static, str>>,
        constraint: &NotEmpty,
    ) -> Validation<Self> {
        if let Some((code, ())) = constraint.validate(&self) {
            Validation::Failure(vec![invalid_optional_value(code, name, None, None)])
        } else {
            Validation::Success(self)
        }
    }
}

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
    Exact(usize),
    Max(usize),
    Min(usize),
    MinMax(usize, usize),
}

impl CharCount {
    pub fn validate(&self, char_count: usize) -> Option<(&'static str, usize)> {
        match *self {
            CharCount::Exact(exact_val) => {
                if char_count != exact_val {
                    Some((INVALID_CHAR_COUNT_EXACT, exact_val))
                } else {
                    None
                }
            }
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
    Range(T, T),
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
            Bound::Range(min, max) => {
                if value < min {
                    Some((INVALID_BOUND_MIN, min.clone()))
                } else if value > max {
                    Some((INVALID_BOUND_MAX, max.clone()))
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
pub struct Contains<'a, A>(pub &'a A);

impl<'a, A> Contains<'a, A>
where
    A: Clone,
{
    pub fn validate<T>(&self, value: &T) -> Option<(&'static str, A)>
    where
        T: HasElement<A>,
    {
        if value.has_element(self.0) {
            None
        } else {
            Some((INVALID_CONTAINS_ELEMENT, self.0.clone()))
        }
    }
}

pub trait HasElement<A> {
    fn has_element(&self, element: &A) -> bool;
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
//TODO find better name for `FromTo`
pub struct FromTo(pub &'static str, pub &'static str);

impl FromTo {
    pub fn validate<T>(&self, value1: &T, value2: &T) -> Option<(&'static str, ())>
    where
        T: Eq + Ord,
    {
        if value1 <= value2 {
            None
        } else {
            Some((INVALID_FROM_TO_INCLUSIVE, ()))
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
