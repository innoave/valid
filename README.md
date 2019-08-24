# valid - composable validation functions for custom types

`valid` is a validation library for the [Rust] language. It let us write validation functions for
our custom types through composition of available validators. Any custom written validation function
again can be used to build validations for even more complex types.

The `valid` crate defines the types and traits to implement validation functions and use them to
validate our values. Additionally it provides definitions of basic constraints with implementations
of the validation function for all primitive types and for some types of the std-lib. 

The goal for the core functionality of this crate is to have no dependencies other than the std-lib.
Support for types of other crates such as [`bigdecimal`] and [`chrono`] are implemented as optional
crate features. So you can pick and choose which types you need in your application and which
dependencies you will have in your project.

## Features

* Definition of a simple validation API
* Definition of primitive constraints, such as `Length`, `CharCount`, `Bound` and `MustMatch` 
* Composition of validation functions to implement validation for complex types
* Separation of validation and presentation of validation errors
* The `ValidationError` is designed to help with composing detailed and helpful error messages for 
  targeted to users of an application. Localization or internationalization of error messages is not
  scope of this crate.
* Generic implementations of the validation function for the provided constraints
* The core functionality has no dependencies to 3rd party crates
* The error type `ValidationError` implements `std::error::Error` and can be used with the
  [`failure`] crate
* Serialization and deserialization of `ValidationError` through [`serde`] (optional crate feature
  [serde1])
* Support for widely used types of 3rd party crates through optional crate features
* Support for `BigDecimal` of the [`bigdecimal`] crate (optional crate feature [bigdecimal])
* Support for `DateTime<Utc>` and `NaiveDate` of the [`chrono`] crate (optional crate feature
  [chrono])



[rust]: https://rust-lang.org
[`bigdecimal`]: https://crates.io/crate/bigdecimal
[`chrono`]: https:://crates.io/crate/chrono
[`failure`]: https:://crates.io/crate/failure
[`serde`]: https:://crates.io/crate/serde
[`valid`]: https://crates.io/crates/valid
