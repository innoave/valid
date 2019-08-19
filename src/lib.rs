//! # `valid` - composable validation for custom types
//!
//! `valid` is simple to use with only 2 basic elements: the `validate` function of the
//! `Validate` trait and the `ValidationError`. The result of the `validate`
//! function is the `Validation` type, which allows to compose implementations
//! of `Validate`.
//!
//! ## Examples
//!
//! Successful validation of a simple variable:
//!
//! ```
//! use valid::{Validate, Validated};
//! use valid::primitives::Length;
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
//! use valid::primitives::Length;
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
//! ```
//!
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
#![allow(dead_code)] //TODO remove eventually

mod core;
pub mod primitives;

// re-export all the core types
pub use crate::core::*;
