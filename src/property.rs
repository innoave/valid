//! Property traits
//!
//! This module defines property traits that enable the generic implementation
//! of the `Validate` trait of the provided constraints.
//!
//! Property traits are one way to derive an implementation of `Validate` trait
//! for a custom type. For example if we have a custom type that represents
//! some kind of decimal number for which we implement the `Decimal` trait we
//! can use the existing implementation of the `Validate` trait for the `Digits`
//! constraint and our custom type.

/// The checked property of a type.
///
/// This can be property of enums with 2 variants that have a similar meaning to
/// the boolean type, e.g. yes/no, agreed/rejected, open/closed,...
pub trait HasCheckedValue {
    /// Returns whether this value represents "checked"
    fn is_checked_value(&self) -> bool;
}

/// The empty property of a type.
///
/// This is usually a property of some kind of container like `String`, `Vec`,
/// `HashSet` or `HashMap`.
pub trait HasEmptyValue {
    /// Returns whether the value is empty
    fn is_empty_value(&self) -> bool;
}

/// The length property of a type.
///
/// This is usually a property of some kind of container like `String`, `Vec`,
/// `HashSet`, `HashMap` or `&[T]`.
pub trait HasLength {
    /// Returns the length of a value
    fn length(&self) -> usize;
}

/// The number of characters property of a type.
///
/// Counts the number of contained characters. The character count may be
/// different from the length if any character occupies more than one byte in
/// memory.
///
/// This is usually a property of a container of `char`s like `String`,
/// `Vec<char>` or `&[char]`
pub trait HasCharCount {
    /// Returns the number of characters.
    fn char_count(&self) -> usize;
}

/// Properties of a decimal number.
pub trait HasDecimalDigits {
    /// Returns the number of integer digits
    ///
    /// These are the digits to the left of the decimal point
    fn integer_digits(&self) -> u64;

    /// Returns the number of fractional digits
    ///
    /// These are the digits to the right of the decimal point
    fn fraction_digits(&self) -> u64;
}

/// Determines whether the given element is part of a value or member of
/// a collection.
///
/// This is usually a property of some kind of container like `String`, `Vec`,
/// `HashSet` or `&[T]`.
pub trait HasMember<A> {
    /// Returns whether the given element is part of this value or a member of
    /// it
    fn has_member(&self, element: &A) -> bool;
}
