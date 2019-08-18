//! # `valid` - composable validation for custom types
//!
//! `valid` is simple to use with only 2 basic elements: the `validate` function of the
//! `Validate` trait and the `ValidationError`. The result of the `validate`
//! function is the `Validation` type, which allows to compose implementations
//! of `Validate`.
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
mod primitives;

// re-export all the core types
pub use crate::core::*;
