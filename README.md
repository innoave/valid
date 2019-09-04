# valid
 
[![Latest Release]][crates.io]
[![Documentation]][docs.rs]
[![License]](LICENSE)
[![Linux Build Status]][travis-ci]
[![Windows Build Status]][appveyor-ci]
[![Test Coverage]][codecov]
[![Rustc Version 1.35+]][rustc-notes]

[Latest Release]: https://img.shields.io/crates/v/valid.svg
[Documentation]: https://docs.rs/valid/badge.svg
[License]: https://img.shields.io/badge/license-MIT%2FApache_2.0-blue.svg
[Linux Build Status]: https://travis-ci.org/innoave/valid.svg?branch=master
[Windows Build Status]: https://ci.appveyor.com/api/projects/status/github/innoave/valid?branch=master&svg=true
[Test Coverage]: https://codecov.io/gh/innoave/valid/branch/master/graph/badge.svg
[Rustc Version 1.35+]: https://img.shields.io/badge/rustc-1.35+-lightgray.svg

[crates.io]: https://crates.io/crates/valid/
[docs.rs]: https://docs.rs/valid
[MIT]: https://opensource.org/licenses/MIT
[Apache-2.0]: https://www.apache.org/licenses/LICENSE-2.0
[travis-ci]: https://travis-ci.org/innoave/valid/
[appveyor-ci]: https://ci.appveyor.com/project/innoave/valid
[codecov]: https://codecov.io/github/innoave/valid?branch=master
[rustc-notes]: https://blog.rust-lang.org/2019/05/23/Rust-1.35.0.html

**Let your business logic only accept valid values**

`valid` is a validation library for the [Rust] language. It let us write validation functions for
our custom types through composition of available validators. Any custom written validation function
again can be used to build validations for even more complex types.

The `valid` crate defines the types and traits to implement validation functions and use them to
validate our values. Additionally it defines primitive constraints.
 
Most primitive constraints validate one property of the validated type. E.g. the `Length` constraint
validates the length property of strings, container types or slices. If the constraint property is
not covered by a trait of the std-lib, a related trait is defined, which we call a _property trait_. 

The builtin constraints are implemented for generic types `T` that implement the related property
trait.

One goal of `valid` is to provide one common API that can be used to validate all kind of business 
rules. Constraints are grouped into one of 3 categories:
 
1. field level constraint, e.g. only a range of values is allowed
2. the relation of 2 fields, e.g. 2 password fields must match or 2 fields must define a range
3. business rules that verify some aspect of the application state, e.g. a username must be unique

Any violation of constraints are returned in one common error type, regardless of the category of 
the business rule that defines the constraint.

One principle for the core functionality of this crate is to have no dependencies other than
the std-lib. Support for types of 3rd party crates such as [`bigdecimal`] and [`chrono`] are 
implemented as optional crate features. So you can pick and choose which types you need in your 
application and which dependencies you will have in your project.

## Features

* Common validation API for validating constraints on field values, constraints on related fields 
  and constraints on application state 
* Definition of primitive constraints, such as `Length`, `CharCount`, `Bound` and `MustMatch` 
* Generic implementations of the validation function for the provided constraints
* Composition of validation functions to implement validation for complex types
* One common error type for validation errors of all kind of constraints
* Accumulation of multiple constraint violations
* Separation of the validation process itself and presentation of validation errors
* The `ValidationError` is designed to help with composing detailed and helpful error messages  
  targeted to the users of an application. Localization or internationalization of error messages is
  not scope of this crate.
* The core functionality has no dependencies to 3rd party crates
* Error codes are compatible with the naming convention in the [_fluent_] project
* The error type `ValidationError` implements `std::error::Error` and can be used with the
  [`failure`] crate
* Serialization and deserialization of `ValidationError` through [`serde`] (optional crate feature
  [serde1])
* Support for widely used types of 3rd party crates through optional crate features
* Support for `BigDecimal` of the [`bigdecimal`] crate (optional crate feature [bigdecimal])
* Support for `DateTime` and `NaiveDate` of the [`chrono`] crate (optional crate feature [chrono])

## Usage

`valid` provides some of its functionality as optional crate features. To use it we must enable the 
relevant crate feature in our `Cargo.toml` file.

Serialization and deserialization of `ValdiationError` through the [`serde`] crate:

```toml
[dependencies]
valid = { version = "0.1", features = ["serde1"] }
```

Support for validating `BigDecimal` of the [`bigdecimal`] crate:

```toml
[dependencies]
valid = { version = "0.1", features = ["bigdecimal"] }
```
 
Support for validating `NaiveDate` and `DateTime` of the [`chrono`] crate:

```toml
[dependencies]
valid = { version = "0.1", features = ["chrono"] }
```
 
Theses crate features can be enabled in any combination. For detailed information on how to use
[`valid`] see the [API documentation at docs.rs](https://docs.rs/valid).

[rust]: https://rust-lang.org
[`bigdecimal`]: https://crates.io/crates/bigdecimal
[`chrono`]: https://crates.io/crates/chrono
[`failure`]: https://crates.io/crates/failure
[_fluent_]: https://projectfluent.org/
[`serde`]: https://crates.io/crates/serde
[`valid`]: https://crates.io/crates/valid
