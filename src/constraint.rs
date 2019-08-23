use crate::{
    invalid_optional_value, invalid_relation, invalid_value, FieldName, RelatedFields, Validate,
    Validation, Value,
};

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
pub const INVALID_BOUND_CLOSED_MAX: &str = "invalid.bound.closed.max";
pub const INVALID_BOUND_CLOSED_MIN: &str = "invalid.bound.closed.min";
pub const INVALID_BOUND_OPEN_MAX: &str = "invalid.bound.open.max";
pub const INVALID_BOUND_OPEN_MIN: &str = "invalid.bound.open.min";

pub const INVALID_DIGITS_INTEGER: &str = "invalid.digits.integer";
pub const INVALID_DIGITS_FRACTION: &str = "invalid.digits.fraction";

pub const INVALID_CONTAINS_ELEMENT: &str = "invalid.contains.element";

pub const INVALID_MUST_MATCH: &str = "invalid.must.match";

pub const INVALID_FROM_TO_INCLUSIVE: &str = "invalid.from.to.inclusive";
pub const INVALID_FROM_TO_EXCLUSIVE: &str = "invalid.from.to.exclusive";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AssertTrue;

impl Validate<AssertTrue, FieldName> for bool {
    fn validate(self, name: impl Into<FieldName>, _constraint: &AssertTrue) -> Validation<Self> {
        if self {
            Validation::Success(self)
        } else {
            Validation::Failure(vec![invalid_value(INVALID_ASSERT_TRUE, name, self, true)])
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AssertFalse;

impl Validate<AssertFalse, FieldName> for bool {
    fn validate(self, name: impl Into<FieldName>, _constraint: &AssertFalse) -> Validation<Self> {
        if self {
            Validation::Failure(vec![invalid_value(INVALID_ASSERT_FALSE, name, self, false)])
        } else {
            Validation::Success(self)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NotEmpty;

pub trait IsEmptyValue {
    fn is_empty_value(&self) -> bool;
}

impl<T> Validate<NotEmpty, FieldName> for T
where
    T: IsEmptyValue,
{
    fn validate(self, name: impl Into<FieldName>, _constraint: &NotEmpty) -> Validation<Self> {
        if self.is_empty_value() {
            Validation::Failure(vec![invalid_optional_value(
                INVALID_NOT_EMPTY,
                name,
                None,
                None,
            )])
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

pub trait HasLength {
    fn length(&self) -> usize;
}

impl<T> Validate<Length, FieldName> for T
where
    T: HasLength,
{
    fn validate(self, name: impl Into<FieldName>, constraint: &Length) -> Validation<Self> {
        let length = self.length();
        if let Some((code, expected)) = match *constraint {
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
        } {
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

pub trait HasCharCount {
    fn char_count(&self) -> usize;
}

impl<T> Validate<CharCount, FieldName> for T
where
    T: HasCharCount,
{
    fn validate(self, name: impl Into<FieldName>, constraint: &CharCount) -> Validation<Self> {
        let char_count = self.char_count();
        if let Some((code, expected)) = match *constraint {
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
        } {
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

impl<T> Validate<Bound<T>, FieldName> for T
where
    T: PartialEq + PartialOrd + Clone,
    Value: From<T>,
{
    fn validate(self, name: impl Into<FieldName>, constraint: &Bound<T>) -> Validation<Self> {
        if let Some((code, expected)) = match constraint {
            Bound::Exact(bound) => {
                if *bound != self {
                    Some((INVALID_BOUND_EXACT, bound.clone()))
                } else {
                    None
                }
            }
            Bound::ClosedRange(min, max) => {
                if self < *min {
                    Some((INVALID_BOUND_CLOSED_MIN, min.clone()))
                } else if self > *max {
                    Some((INVALID_BOUND_CLOSED_MAX, max.clone()))
                } else {
                    None
                }
            }
            Bound::ClosedOpenRange(min, max) => {
                if self < *min {
                    Some((INVALID_BOUND_CLOSED_MIN, min.clone()))
                } else if self >= *max {
                    Some((INVALID_BOUND_OPEN_MAX, max.clone()))
                } else {
                    None
                }
            }
            Bound::OpenClosedRange(min, max) => {
                if self <= *min {
                    Some((INVALID_BOUND_OPEN_MIN, min.clone()))
                } else if self > *max {
                    Some((INVALID_BOUND_CLOSED_MAX, max.clone()))
                } else {
                    None
                }
            }
            Bound::OpenRange(min, max) => {
                if self <= *min {
                    Some((INVALID_BOUND_OPEN_MIN, min.clone()))
                } else if self >= *max {
                    Some((INVALID_BOUND_OPEN_MAX, max.clone()))
                } else {
                    None
                }
            }
        } {
            Validation::Failure(vec![invalid_value(code, name, self, expected)])
        } else {
            Validation::Success(self)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Digits {
    pub integer: u64,
    pub fraction: u64,
}

pub trait Decimal {
    fn integer_digits(&self) -> u64;
    fn fraction_digits(&self) -> u64;
}

impl<T> Validate<Digits, FieldName> for T
where
    T: Decimal,
{
    fn validate(self, name: impl Into<FieldName>, constraint: &Digits) -> Validation<Self> {
        let integer = self.integer_digits();
        let fraction = self.fraction_digits();
        if integer <= constraint.integer {
            if fraction <= constraint.fraction {
                Validation::Success(self)
            } else {
                Validation::Failure(vec![invalid_value(
                    INVALID_DIGITS_FRACTION,
                    name,
                    fraction,
                    constraint.fraction,
                )])
            }
        } else if fraction <= constraint.fraction {
            Validation::Failure(vec![invalid_value(
                INVALID_DIGITS_INTEGER,
                name,
                integer,
                constraint.integer,
            )])
        } else {
            let name = name.into();
            Validation::Failure(vec![
                invalid_value(
                    INVALID_DIGITS_INTEGER,
                    name.clone(),
                    integer,
                    constraint.integer,
                ),
                invalid_value(INVALID_DIGITS_FRACTION, name, fraction, constraint.fraction),
            ])
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Contains<'a, A>(pub &'a A);

pub trait HasElement<A> {
    fn has_element(&self, element: &A) -> bool;
}

impl<'a, T, A> Validate<Contains<'a, A>, FieldName> for T
where
    T: HasElement<A>,
    A: Clone,
    Value: From<A> + From<T>,
{
    fn validate(
        self,
        name: impl Into<FieldName>,
        constraint: &Contains<'a, A>,
    ) -> Validation<Self> {
        if self.has_element(&constraint.0) {
            Validation::Success(self)
        } else {
            Validation::Failure(vec![invalid_value(
                INVALID_CONTAINS_ELEMENT,
                name,
                self,
                constraint.0.clone(),
            )])
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MustMatch;

impl<T> Validate<MustMatch, RelatedFields> for (T, T)
where
    T: Eq,
    Value: From<T>,
{
    fn validate(
        self,
        fields: impl Into<RelatedFields>,
        _constraint: &MustMatch,
    ) -> Validation<Self> {
        let RelatedFields(name1, name2) = fields.into();
        if self.0 == self.1 {
            Validation::Success(self)
        } else {
            Validation::Failure(vec![invalid_relation(
                INVALID_MUST_MATCH,
                name1,
                self.0,
                name2,
                self.1,
            )])
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
//TODO find better name for `FromTo`
pub enum FromTo {
    Inclusive,
    Exclusive,
}

impl<T> Validate<FromTo, RelatedFields> for (T, T)
where
    T: PartialEq + PartialOrd,
    Value: From<T>,
{
    fn validate(self, fields: impl Into<RelatedFields>, constraint: &FromTo) -> Validation<Self> {
        let RelatedFields(name1, name2) = fields.into();
        match *constraint {
            FromTo::Inclusive => {
                if self.0 <= self.1 {
                    Validation::Success(self)
                } else {
                    Validation::Failure(vec![invalid_relation(
                        INVALID_FROM_TO_INCLUSIVE,
                        name1,
                        self.0,
                        name2,
                        self.1,
                    )])
                }
            }
            FromTo::Exclusive => {
                if self.0 < self.1 {
                    Validation::Success(self)
                } else {
                    Validation::Failure(vec![invalid_relation(
                        INVALID_FROM_TO_EXCLUSIVE,
                        name1,
                        self.0,
                        name2,
                        self.1,
                    )])
                }
            }
        }
    }
}
