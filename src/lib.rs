//! # `valid` - composable validation for custom types
//!
//! The `valid` crate let us write validation functions for our custom types
//! through composition of available validation functions. Any custom written
//! validation function again can be used to build validations for even more
//! complex types.
//!
//! ## Validation function, constraints and context
//!
//! The goal of the validation is to confirm that a given value of type `T` is
//! compliant to a set of one or several constraints. To find out whether a
//! value is compliant we implement a function that checks some constraints on
//! the given value. The signature of such a validation function might look like
//! this:
//!
//! ```rust,ignore
//! fn validate<T, S, C>(value: T, context: S, constraint: C) -> Result<Validated<T>, ValidationError>;
//! ```
//!
//! This function takes a value `T`, a context S, and a constraint definition
//! `C` as input and returns a result that is either a `Validated<T>` or a
//! `ValidationError`. So we might define the validation function as a function
//! that converts a type `T` into some `Validated<T>` or returns an error if one
//! of the defined constraints is violated.
//!
//! The actual validation function of this crate is defined by the
//! [`Validate`](trait.Validate.html) trait. The only difference to the function
//! above is that the value is the `self` parameter. Lets have a look what the
//! other two input parameters 'constraint' and 'context' are about.
//!
//! A constraint defines how to determine whether a value is valid or not. For
//! example a value is valid if it is between a lower and an upper limit or
//! the number of characters in a string must be between a lower and an upper
//! limit. As another example a constraint may define that two fields must
//! match. Additionally business rules may be defined for an application such as
//! that an identifier must be unique within an application.
//!
//! In this library constraints are assigned to one of 3 categories:
//!
//! * Field level constraint
//! * Constraints on the relation between two fields
//! * Constraints for the state of an application
//!
//! This categorization of constraints helps with two aspects of the design of
//! the API. First the kind of information that is needed for the validation
//! function to perform the validation and second to provide a common error
//! type that can be turned into error messages that are meaningful to the user
//! of the application.
//!
//! The actual validation function is defined by the
//! [`Validate`](trait.Validate.html) trait. It takes some context as input.
//! The context provides additional information to the validation function that
//! enables us to implement more complex validations and add additional
//! parameters to the returned error.
//!
//! The context can be one 3 types, where each type corresponds to one of the 3
//! categories mentioned above:
//!
//! * [`FieldName`](struct.FieldName.html) - provides a name of the field that
//!   is validated
//! * [`RelatedFields`](struct.RelatedFields.html) - provides the names of two
//!   related fields
//! * [`State<S>`](struct.State.html) - provides some generic state information
//!
//! For the second aspect the [`ValidationError`](struct.ValidationError.html)
//! struct as defined by this crate contains a list of
//! [`ConstraintViolation`](enum.ConstraintViolation.thml)s. A constraint
//! violation is an enum with 3 variants, one for each of the 3 categories we
//! talked about:
//!
//! * `ConstraintViolation::Field(InvalidaValue)`
//! * `ConstraintViolation::Relation(InvalidRelation)`
//! * `ConstraintViolation::State(InvalidState)`
//!
//!
//! ## Generic implementation of constraints and properties
//!
//! The validation function of the [`Validate`](trait.Validate.html) trait is
//! applied to the combination of a constraint and a value. To validate some
//! constraint `C` for a value of type `T` the `Validate` trait must be
//! implemented for the combination of these two types.
//!
//! Most primitive constraints evaluate one property of a value, such as the
//! length of a string or the number of fraction digits of a decimal number.
//! If we use traits to determine the relevant property of a value (lets call
//! them property traits) we can implement the `Validate` trait for all types
//! `T` that implement the according property trait.
//!
//! This crate implements the `Validate` trait for all provided constraints
//! for all generic types `T` that implement a certain property trait. If there
//! is a trait suitable as a property trait defined by the std-lib we use that
//! otherwise we define our own trait.
//!
//! The property traits defined by this crate are found in the
//! [`property`](mod.property.html) module.
//!
//!
//! ## Examples
//!
//! Successful validation of a simple variable:
//!
//! ```
//! use valid::{Validate, Validated};
//! use valid::constraint::Length;
//!
//! let text = String::from("the answer is 42");
//!
//! let result = text.validate("text", &Length::MinMax(2, 16)).result(Some("validating `text`".into()));
//!
//! assert_eq!(result.unwrap().unwrap(), String::from("the answer is 42"));
//! ```
//!
//!
//! Validating a simple variable with an invalid value:
//!
//! ```
//! use valid::{Validate, ValidationError, InvalidValue, Field, Value};
//! use valid::constraint::Length;
//!
//! let text = String::from("the answer is 42");
//!
//! let result = text.validate("text", &Length::MinMax(2, 15)).result(Some("validating `text`".into()));
//!
//! assert_eq!(result, Err(ValidationError {
//!     message: Some("validating `text`".into()),
//!     violations: vec![InvalidValue {
//!         code: "invalid.length.max".into(),
//!         field: Field {
//!             name: "text".into(),
//!             actual: Some(Value::Integer(16)),
//!             expected: Some(Value::Integer(15)),
//!         }
//!     }.into()],
//! }));
//!
//! // ValidationError implements the Display trait
//! assert_eq!(result.unwrap_err().to_string(), "validating `text`: invalid.length.max of text which is 16, expected to be 15");
//! ```
//!

#![doc(html_root_url = "https://docs.rs/valid/0.1.0")]
#![warn(
    bare_trait_objects,
    missing_copy_implementations,
    missing_debug_implementations,
//    missing_docs,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications
)]

#[cfg(feature = "bigdecimal")]
mod bigdecimal;
pub mod constraint;
mod core;
pub mod property;
mod std_types;

// re-export all the core types
pub use crate::core::*;
