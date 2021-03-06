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
//! naming convention but use a dash (`-`) instead of the underscore to separate
//! terms. Thus the codes are compatible with the convention used in the
//! [_fluent_] project.
//!
//! [_fluent_]: https://projectfluent.org/

use crate::property::{
    HasCharCount, HasCheckedValue, HasDecimalDigits, HasEmptyValue, HasLength, HasMember,
    HasZeroValue,
};
use crate::{
    invalid_optional_value, invalid_relation, invalid_value, FieldName, RelatedFields, Validate,
    Validation, Value,
};
use std::convert::TryFrom;

/// Error code: the value does not assert to true (`AssertTrue` constraint)
pub const INVALID_ASSERT_TRUE: &str = "invalid-assert-true";

/// Error code: the value does not assert to false (`AssertFalse` constraint)
pub const INVALID_ASSERT_FALSE: &str = "invalid-assert-false";

/// Error code: the value is empty (`NotEmpty` constraint)
pub const INVALID_NOT_EMPTY: &str = "invalid-not-empty";

/// Error code: the length is not the exactly the specified value
/// (`Length::Exact` constraint)
pub const INVALID_LENGTH_EXACT: &str = "invalid-length-exact";

/// Error code: the length is not less or equal the specified maximum
/// (`Length::Max` constraint)
pub const INVALID_LENGTH_MAX: &str = "invalid-length-max";

/// Error code: the length is not greater or equal the specified minimum
/// (`Length::Min` constraint)
pub const INVALID_LENGTH_MIN: &str = "invalid-length-min";

/// Error code: the number of characters is not exactly the specified value
/// (`CharCount::Exact` constraint)
pub const INVALID_CHAR_COUNT_EXACT: &str = "invalid-char-count-exact";

/// Error code: the number of characters is not less or equal the specified
/// maximum (`CharCount::Max` constraint)
pub const INVALID_CHAR_COUNT_MAX: &str = "invalid-char-count-max";

/// Error code: the number of characters is not greater or equal the specified
/// minimum (`CharCount::Min` constraint)
pub const INVALID_CHAR_COUNT_MIN: &str = "invalid-char-count-min";

/// Error code: the value is not exactly the specified value
/// (`Bound::Exact` constraint)
pub const INVALID_BOUND_EXACT: &str = "invalid-bound-exact";

/// Error code: the value is not less than or equal to the specified maximum
/// (`Bound::ClosedRange` or `Bound::OpenClosedRange` constraint)
pub const INVALID_BOUND_CLOSED_MAX: &str = "invalid-bound-closed-max";

/// Error code: the value is not greater than or equal to the specified minimum
/// (`Bound::ClosedRange` or `Bound::ClosedOpenRange` constraint)
pub const INVALID_BOUND_CLOSED_MIN: &str = "invalid-bound-closed-min";

/// Error code: the value is not less than the specified maximum
/// (`Bound::OpenRange` or `Bound::ClosedOpenRange` constraint)
pub const INVALID_BOUND_OPEN_MAX: &str = "invalid-bound-open-max";

/// Error code: the value is not greater than the specified minimum
/// (`Bound::OpenRange` or `Bound::OpenClosedRange` constraint)
pub const INVALID_BOUND_OPEN_MIN: &str = "invalid-bound-open-min";

/// Error code: the value is zero (`NonZero` constraint)
pub const INVALID_NON_ZERO: &str = "invalid-non-zero";

/// Error code: the number of integer digits is not less than or equal to the
/// specified maximum (`Digits::integer` constraint)
pub const INVALID_DIGITS_INTEGER: &str = "invalid-digits-integer";

/// Error code: the number of fraction digits is not less than or equal to the
/// specified maximum (`Digits::fraction` constraint)
pub const INVALID_DIGITS_FRACTION: &str = "invalid-digits-fraction";

/// Error code: the value does not contain the specified member element
/// (`Contains` constraint)
pub const INVALID_CONTAINS_ELEMENT: &str = "invalid-contains-element";

/// Error code: the two values do not match (`MustMatch` constraint)
pub const INVALID_MUST_MATCH: &str = "invalid-must-match";

/// Error code: the first value is not less than or equal to the second value
/// (`MustDefineRange::Inclusive` constraint)
pub const INVALID_MUST_DEFINE_RANGE_INCLUSIVE: &str = "invalid-must-define-range-inclusive";

/// Error code: the first value is not less than the second value
/// (`MustDefineRange::Exclusive` constraint)
pub const INVALID_MUST_DEFINE_RANGE_EXCLUSIVE: &str = "invalid-must-define-range-exclusive";

/// The value must be true.
///
/// The validation function can be applied in the [`FieldName`] context.
/// It is implemented for all types `T` that implement the [`HasCheckedValue`]
/// property trait.
///
/// [`FieldName`]: ../core/struct.FieldName.html
/// [`HasCheckedValue`]: ../property/trait.HasCheckedValue.html
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AssertTrue;

impl<T> Validate<AssertTrue, FieldName> for T
where
    T: HasCheckedValue,
{
    fn validate(
        self,
        name: impl Into<FieldName>,
        _constraint: &AssertTrue,
    ) -> Validation<AssertTrue, Self> {
        if self.is_checked_value() {
            Validation::success(self)
        } else {
            Validation::failure(vec![invalid_value(INVALID_ASSERT_TRUE, name, false, true)])
        }
    }
}

/// The value must be false.
///
/// The validation function can be applied in the [`FieldName`] context.
/// It is implemented for all types `T` that implement the [`HasCheckedValue`]
/// property trait.
///
/// [`FieldName`]: ../core/struct.FieldName.html
/// [`HasCheckedValue`]: ../property/trait.HasCheckedValue.html
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AssertFalse;

impl<T> Validate<AssertFalse, FieldName> for T
where
    T: HasCheckedValue,
{
    fn validate(
        self,
        name: impl Into<FieldName>,
        _constraint: &AssertFalse,
    ) -> Validation<AssertFalse, Self> {
        if self.is_checked_value() {
            Validation::failure(vec![invalid_value(INVALID_ASSERT_FALSE, name, true, false)])
        } else {
            Validation::success(self)
        }
    }
}

/// The value must not be empty.
///
/// The validation function can be applied in the [`FieldName`] context.
/// It is implemented for all types `T` that implement the [`HasEmptyValue`]
/// property trait.
///
/// [`FieldName`]: ../core/struct.FieldName.html
/// [`HasEmptyValue`]: ../property/trait.HasEmptyValue.html
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NotEmpty;

impl<T> Validate<NotEmpty, FieldName> for T
where
    T: HasEmptyValue,
{
    fn validate(
        self,
        name: impl Into<FieldName>,
        _constraint: &NotEmpty,
    ) -> Validation<NotEmpty, Self> {
        if self.is_empty_value() {
            Validation::failure(vec![invalid_optional_value(
                INVALID_NOT_EMPTY,
                name,
                None,
                None,
            )])
        } else {
            Validation::success(self)
        }
    }
}

/// The length of a value must be within some bounds.
///
/// The validation function can be applied in the [`FieldName`] context.
/// It is implemented for all types `T` that implement the [`HasLength`]
/// property trait.
///
/// [`FieldName`]: ../core/struct.FieldName.html
/// [`HasLength`]: ../property/trait.HasLength.html
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Length {
    /// The length of the value must be less than or equal to the specified
    /// maximum
    Max(usize),
    /// The length of the value must be greater than or equal to the specified
    /// minimum
    Min(usize),
    /// The length of the value must be between the specified minimum and
    /// maximum (inclusive)
    MinMax(usize, usize),
    /// The value must be of an exact length
    Exact(usize),
}

impl<T> Validate<Length, FieldName> for T
where
    T: HasLength,
{
    fn validate(self, name: impl Into<FieldName>, constraint: &Length) -> Validation<Length, Self> {
        let length = self.length();
        if let Some((code, expected)) = match *constraint {
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
            Length::Exact(exact_len) => {
                if length != exact_len {
                    Some((INVALID_LENGTH_EXACT, exact_len))
                } else {
                    None
                }
            }
        } {
            let actual = Value::try_from(length).ok();
            let expected = Value::try_from(expected).ok();
            Validation::failure(vec![invalid_optional_value(code, name, actual, expected)])
        } else {
            Validation::success(self)
        }
    }
}

/// The number of characters must be within some bounds.
///
/// The validation function can be applied in the [`FieldName`] context.
/// It is implemented for all types `T` that implement the [`HasCharCount`]
/// property trait.
///
/// [`FieldName`]: ../core/struct.FieldName.html
/// [`HasCharCount`]: ../property/trait.HasCharCount.html
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharCount {
    /// The number of characters must be less than or equal to the specified
    /// maximum
    Max(usize),
    /// The number of characters must be greater than or equal to the specified
    /// minimum
    Min(usize),
    /// The number of characters must be between the specified minimum and
    /// maximum (inclusive)
    MinMax(usize, usize),
    /// The number of characters must be equal to the specified amount
    Exact(usize),
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
                    Some((INVALID_CHAR_COUNT_MIN, min))
                } else if char_count > max {
                    Some((INVALID_CHAR_COUNT_MAX, max))
                } else {
                    None
                }
            }
            CharCount::Exact(exact_val) => {
                if char_count != exact_val {
                    Some((INVALID_CHAR_COUNT_EXACT, exact_val))
                } else {
                    None
                }
            }
        } {
            let actual = Value::try_from(char_count).ok();
            let expected = Value::try_from(expected).ok();
            Validation::failure(vec![invalid_optional_value(code, name, actual, expected)])
        } else {
            Validation::success(self)
        }
    }
}

/// The value must be within some bounds.
///
/// The validation function can be applied in the [`FieldName`] context.
/// It is implemented for all types `T` that implement the `PartialOrd` trait
/// and `Into<Value>`.
///
/// [`FieldName`]: ../core/struct.FieldName.html
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bound<T> {
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
    /// The value must have the specified value
    Exact(T),
}

impl<T> Validate<Bound<T>, FieldName> for T
where
    T: PartialOrd + Clone + Into<Value>,
{
    fn validate(
        self,
        name: impl Into<FieldName>,
        constraint: &Bound<T>,
    ) -> Validation<Bound<T>, Self> {
        if let Some((code, expected)) = match constraint {
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
            Bound::Exact(bound) => {
                if *bound != self {
                    Some((INVALID_BOUND_EXACT, bound.clone()))
                } else {
                    None
                }
            }
        } {
            Validation::failure(vec![invalid_value(code, name, self, expected)])
        } else {
            Validation::success(self)
        }
    }
}

/// Values of zero are not allowed.
///
/// The validation function can be applied in the [`FieldName`] context.
/// It is implemented for all types `T` that implement the [`HasZeroValue`]
/// property trait and `Into<Value>`.
///
/// [`FieldName`]: ../core/struct.FieldName.html
/// [`HasZeroValue`]: ../property/trait.HasZeroValue.html
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NonZero;

impl<T> Validate<NonZero, FieldName> for T
where
    T: HasZeroValue + Into<Value>,
{
    fn validate(
        self,
        name: impl Into<FieldName>,
        _constraint: &NonZero,
    ) -> Validation<NonZero, Self> {
        if self.is_zero_value() {
            Validation::failure(vec![invalid_optional_value(
                INVALID_NON_ZERO,
                name,
                Some(self.into()),
                None,
            )])
        } else {
            Validation::success(self)
        }
    }
}

/// Maximum number of allowed integer digits and fraction digits.
///
/// The validation function can be applied in the [`FieldName`] context.
/// It is implemented for all types `T` that implement the [`HasDecimalDigits`]
/// property trait.
///
/// [`FieldName`]: ../core/struct.FieldName.html
/// [`HasDecimalDigits`]: ../property/trait.HasDecimalDigits.html
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
    T: HasDecimalDigits,
{
    fn validate(self, name: impl Into<FieldName>, constraint: &Digits) -> Validation<Digits, Self> {
        let integer = self.integer_digits();
        let fraction = self.fraction_digits();
        if integer <= constraint.integer {
            if fraction <= constraint.fraction {
                Validation::success(self)
            } else {
                Validation::failure(vec![invalid_value(
                    INVALID_DIGITS_FRACTION,
                    name,
                    fraction,
                    constraint.fraction,
                )])
            }
        } else if fraction <= constraint.fraction {
            Validation::failure(vec![invalid_value(
                INVALID_DIGITS_INTEGER,
                name,
                integer,
                constraint.integer,
            )])
        } else {
            let name = name.into();
            Validation::failure(vec![
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
///
/// The validation function can be applied in the [`FieldName`] context.
/// It is implemented for all types `T` that implement the [`HasMember`]
/// property trait and `Into<Value>`.
///
/// [`FieldName`]: ../core/struct.FieldName.html
/// [`HasMember`]: ../property/trait.HasMember.html
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Contains<'a, A>(pub &'a A);

impl<'a, T, A> Validate<Contains<'a, A>, FieldName> for T
where
    T: HasMember<A> + Into<Value>,
    A: Clone + Into<Value>,
{
    fn validate(
        self,
        name: impl Into<FieldName>,
        constraint: &Contains<'a, A>,
    ) -> Validation<Contains<'a, A>, Self> {
        if self.has_member(&constraint.0) {
            Validation::success(self)
        } else {
            Validation::failure(vec![invalid_value(
                INVALID_CONTAINS_ELEMENT,
                name,
                self,
                constraint.0.clone(),
            )])
        }
    }
}

/// Two related fields must be equal.
///
/// The validation function can be applied in the [`RelatedFields`] context.
/// It is implemented for all types `T` that implement the `PartialEq` trait.
///
/// [`RelatedFields`]: ../core/struct.RelatedFields.html
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MustMatch;

impl<T> Validate<MustMatch, RelatedFields> for (T, T)
where
    T: PartialEq + Into<Value>,
{
    fn validate(
        self,
        fields: impl Into<RelatedFields>,
        _constraint: &MustMatch,
    ) -> Validation<MustMatch, Self> {
        let RelatedFields(name1, name2) = fields.into();
        if self.0 == self.1 {
            Validation::success(self)
        } else {
            Validation::failure(vec![invalid_relation(
                INVALID_MUST_MATCH,
                name1,
                self.0,
                name2,
                self.1,
            )])
        }
    }
}

/// Two related fields must define a range.
///
/// This constraint is useful for structs with pairs of fields that define a
/// range such as `valid_from` and `valid_until` or `min_salary` and
/// `max_salary`.
///
/// The validation function can be applied in the [`RelatedFields`] context.
/// It is implemented for all types `T` that implement the `PartialOrd` trait
/// and `Into<Value`.
///
/// [`RelatedFields`]: ../core/struct.RelatedFields.html
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MustDefineRange {
    /// The first value must be less than or equal to the second value
    Inclusive,
    /// The first value must be less than the second value
    Exclusive,
}

impl<T> Validate<MustDefineRange, RelatedFields> for (T, T)
where
    T: PartialOrd + Into<Value>,
{
    fn validate(
        self,
        fields: impl Into<RelatedFields>,
        constraint: &MustDefineRange,
    ) -> Validation<MustDefineRange, Self> {
        let RelatedFields(name1, name2) = fields.into();
        match *constraint {
            MustDefineRange::Inclusive => {
                if self.0 <= self.1 {
                    Validation::success(self)
                } else {
                    Validation::failure(vec![invalid_relation(
                        INVALID_MUST_DEFINE_RANGE_INCLUSIVE,
                        name1,
                        self.0,
                        name2,
                        self.1,
                    )])
                }
            }
            MustDefineRange::Exclusive => {
                if self.0 < self.1 {
                    Validation::success(self)
                } else {
                    Validation::failure(vec![invalid_relation(
                        INVALID_MUST_DEFINE_RANGE_EXCLUSIVE,
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

#[cfg(feature = "regex")]
pub use with_regex::*;

#[cfg(feature = "regex")]
mod with_regex {
    use crate::{invalid_value, FieldName, Validate, Validation};
    use regex::Regex;

    /// Error code: the value does not match the specified pattern
    /// (`Pattern` constraint)
    pub const INVALID_PATTERN: &str = "invalid-pattern";

    /// The value must match some regular expression.
    ///
    /// The validation function can be applied in the [`FieldName`] context.
    /// It is implemented for `String`.
    ///
    /// [`FieldName`]: ../core/struct.FieldName.html
    #[derive(Debug, Clone)]
    pub struct Pattern(pub Regex);

    impl Validate<Pattern, FieldName> for String {
        fn validate(
            self,
            name: impl Into<FieldName>,
            constraint: &Pattern,
        ) -> Validation<Pattern, Self> {
            if constraint.0.is_match(&self) {
                Validation::success(self)
            } else {
                Validation::failure(vec![invalid_value(
                    INVALID_PATTERN,
                    name,
                    self,
                    constraint.0.to_string(),
                )])
            }
        }
    }
}

#[cfg(test)]
mod tests;
