//! Constraints defined by this crate
//!
//! For each constraint the possible error codes are defined as a set of
//! constants. The name of the constants follow the naming convention:
//!
//! ```text,ignore
//! INVALID_<constraint-name>[_<variant>]
//! ```
//!
//! The <variant> part is optional and only present if the constraint has some
//! variants. The name of a constraint and its variants is converted to
//! screaming snake case. The string values of the error codes follow a similar
//! naming convention.

use crate::property::{DecimalDigits, HasCharCount, HasLength, HasMember, IsEmptyValue};
use crate::{
    invalid_optional_value, invalid_relation, invalid_value, FieldName, RelatedFields, Validate,
    Validation, Value,
};
use std::marker::PhantomData;

/// Error code: the value does not assert to true (`AssertTrue` constraint)
pub const INVALID_ASSERT_TRUE: &str = "invalid.assert.true";

/// Error code: the value does not assert to false (`AssertFalse` constraint)
pub const INVALID_ASSERT_FALSE: &str = "invalid.assert.false";

/// Error code: the value is empty (`NotEmpty` constraint)
pub const INVALID_NOT_EMPTY: &str = "invalid.not.empty";

/// Error code: the length is not the exactly the specified value
/// (`Length::Exact` constraint)
pub const INVALID_LENGTH_EXACT: &str = "invalid.length.exact";

/// Error code: the length is not less or equal the specified maximum
/// (`Length::Max` constraint)
pub const INVALID_LENGTH_MAX: &str = "invalid.length.max";

/// Error code: the length is not greater or equal the specified minimum
/// (`Length::Min` constraint)
pub const INVALID_LENGTH_MIN: &str = "invalid.length.min";

/// Error code: the number of characters is not exactly the specified value
/// (`CharCount::Exact` constraint)
pub const INVALID_CHAR_COUNT_EXACT: &str = "invalid.char.count.exact";

/// Error code: the number of characters is not less or equal the specified
/// maximum (`CharCount::Max` constraint)
pub const INVALID_CHAR_COUNT_MAX: &str = "invalid.char.count.max";

/// Error code: the number of characters is not greater or equal the specified
/// minimum (`CharCount::Min` constraint)
pub const INVALID_CHAR_COUNT_MIN: &str = "invalid.char.count.min";

/// Error code: the value is not exactly the specified value
/// (`Bound::Exact` constraint)
pub const INVALID_BOUND_EXACT: &str = "invalid.bound.exact";

/// Error code: the value is not less than or equal to the specified maximum
/// (`Bound::ClosedRange` or `Bound::OpenClosedRange` constraint)
pub const INVALID_BOUND_CLOSED_MAX: &str = "invalid.bound.closed.max";

/// Error code: the value is not greater than or equal to the specified minimum
/// (`Bound::ClosedRange` or `Bound::ClosedOpenRange` constraint)
pub const INVALID_BOUND_CLOSED_MIN: &str = "invalid.bound.closed.min";

/// Error code: the value is not less than the specified maximum
/// (`Bound::OpenRange` or `Bound::ClosedOpenRange` constraint)
pub const INVALID_BOUND_OPEN_MAX: &str = "invalid.bound.open.max";

/// Error code: the value is not greater than the specified minimum
/// (`Bound::OpenRange` or `Bound::OpenClosedRange` constraint)
pub const INVALID_BOUND_OPEN_MIN: &str = "invalid.bound.open.min";

/// Error code: the number of integer digits is not less than or equal to the
/// specified maximum (`Digits::integer` constraint)
pub const INVALID_DIGITS_INTEGER: &str = "invalid.digits.integer";

/// Error code: the number of fraction digits is not less than or equal to the
/// specified maximum (`Digits::fraction` constraint)
pub const INVALID_DIGITS_FRACTION: &str = "invalid.digits.fraction";

/// Error code: the value does not contain the specified member element
/// (`Contains` constraint)
pub const INVALID_CONTAINS_ELEMENT: &str = "invalid.contains.element";

/// Error code: the two values do not match (`MustMatch` constraint)
pub const INVALID_MUST_MATCH: &str = "invalid.must.match";

/// Error code: the first value is not less than or equal to the second value
/// (`FromTo::Inclusive` constraint)
pub const INVALID_FROM_TO_INCLUSIVE: &str = "invalid.from.to.inclusive";

/// Error code: the first value is not less than the second value
/// (`FromTo::Exclusive` constraint)
pub const INVALID_FROM_TO_EXCLUSIVE: &str = "invalid.from.to.exclusive";

/// The value must be true
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AssertTrue;

impl Validate<AssertTrue, FieldName> for bool {
    fn validate(
        self,
        name: impl Into<FieldName>,
        _constraint: &AssertTrue,
    ) -> Validation<AssertTrue, Self> {
        if self {
            Validation::Success(PhantomData, self)
        } else {
            Validation::Failure(vec![invalid_value(INVALID_ASSERT_TRUE, name, self, true)])
        }
    }
}

/// The value must be false
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AssertFalse;

impl Validate<AssertFalse, FieldName> for bool {
    fn validate(
        self,
        name: impl Into<FieldName>,
        _constraint: &AssertFalse,
    ) -> Validation<AssertFalse, Self> {
        if self {
            Validation::Failure(vec![invalid_value(INVALID_ASSERT_FALSE, name, self, false)])
        } else {
            Validation::Success(PhantomData, self)
        }
    }
}

/// The value must not be empty
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NotEmpty;

impl<T> Validate<NotEmpty, FieldName> for T
where
    T: IsEmptyValue,
{
    fn validate(
        self,
        name: impl Into<FieldName>,
        _constraint: &NotEmpty,
    ) -> Validation<NotEmpty, Self> {
        if self.is_empty_value() {
            Validation::Failure(vec![invalid_optional_value(
                INVALID_NOT_EMPTY,
                name,
                None,
                None,
            )])
        } else {
            Validation::Success(PhantomData, self)
        }
    }
}

/// The length of a value must be within some bounds
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Length {
    /// The value must be of an exact length
    Exact(usize),
    /// The length of the value must be less than or equal to the specified
    /// maximum
    Max(usize),
    /// The length of the value must be greater than or equal to the specified
    /// minimum
    Min(usize),
    /// The length of the value must be between the specified minimum and
    /// maximum (inclusive)
    MinMax(usize, usize),
}

impl<T> Validate<Length, FieldName> for T
where
    T: HasLength,
{
    fn validate(self, name: impl Into<FieldName>, constraint: &Length) -> Validation<Length, Self> {
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
            Validation::Success(PhantomData, self)
        }
    }
}

/// The number of characters must be within some bounds
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharCount {
    /// The number of characters must be equal to the specified amount
    Exact(usize),
    /// The number of characters must be less than or equal to the specified
    /// maximum
    Max(usize),
    /// The number of characters must be greater than or equal to the specified
    /// minimum
    Min(usize),
    /// The number of characters must be between the specified minimum and
    /// maximum (inclusive)
    MinMax(usize, usize),
}

impl<T> Validate<CharCount, FieldName> for T
where
    T: HasCharCount,
{
    fn validate(
        self,
        name: impl Into<FieldName>,
        constraint: &CharCount,
    ) -> Validation<CharCount, Self> {
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
            Validation::Success(PhantomData, self)
        }
    }
}

/// The value must be within some bounds
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bound<T> {
    /// The value must have the specified value
    Exact(T),
    /// The value must be between the specified minimum (inclusive) and
    /// maximum (inclusive)
    ClosedRange(T, T),
    /// The value must be between the specified minimum (inclusive) and
    /// maximum (exclusive)
    ClosedOpenRange(T, T),
    /// The value must be between the specified minimum (exclusive) and
    /// maximum (inclusive)
    OpenClosedRange(T, T),
    /// The value must be between the specified minimum (exclusive) and
    /// maximum (exclusive)
    OpenRange(T, T),
}

impl<T> Validate<Bound<T>, FieldName> for T
where
    T: PartialEq + PartialOrd + Clone,
    Value: From<T>,
{
    fn validate(
        self,
        name: impl Into<FieldName>,
        constraint: &Bound<T>,
    ) -> Validation<Bound<T>, Self> {
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
            Validation::Success(PhantomData, self)
        }
    }
}

/// Maximum number of allowed integer digits and fraction digits
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Digits {
    /// Maximum number of allowed integer digits (digits to the left of the
    /// decimal point)
    pub integer: u64,
    /// Maximum number of allowed fraction digits (digits to the right of the
    /// decimal point)
    pub fraction: u64,
}

impl<T> Validate<Digits, FieldName> for T
where
    T: DecimalDigits,
{
    fn validate(self, name: impl Into<FieldName>, constraint: &Digits) -> Validation<Digits, Self> {
        let integer = self.integer_digits();
        let fraction = self.fraction_digits();
        if integer <= constraint.integer {
            if fraction <= constraint.fraction {
                Validation::Success(PhantomData, self)
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

/// The value must contain the specified member or the specified member must be
/// part of the value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Contains<'a, A>(pub &'a A);

impl<'a, T, A> Validate<Contains<'a, A>, FieldName> for T
where
    T: HasMember<A>,
    A: Clone,
    Value: From<A> + From<T>,
{
    fn validate(
        self,
        name: impl Into<FieldName>,
        constraint: &Contains<'a, A>,
    ) -> Validation<Contains<'a, A>, Self> {
        if self.has_member(&constraint.0) {
            Validation::Success(PhantomData, self)
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

/// Two related fields must be equal
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
    ) -> Validation<MustMatch, Self> {
        let RelatedFields(name1, name2) = fields.into();
        if self.0 == self.1 {
            Validation::Success(PhantomData, self)
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

/// Two related fields must define a range
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
//TODO find better name for `FromTo`
pub enum FromTo {
    /// The first value must be less than or equal to the second value
    Inclusive,
    /// The first value must be less than the second value
    Exclusive,
}

impl<T> Validate<FromTo, RelatedFields> for (T, T)
where
    T: PartialEq + PartialOrd,
    Value: From<T>,
{
    fn validate(
        self,
        fields: impl Into<RelatedFields>,
        constraint: &FromTo,
    ) -> Validation<FromTo, Self> {
        let RelatedFields(name1, name2) = fields.into();
        match *constraint {
            FromTo::Inclusive => {
                if self.0 <= self.1 {
                    Validation::Success(PhantomData, self)
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
                    Validation::Success(PhantomData, self)
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
