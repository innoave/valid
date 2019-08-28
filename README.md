# valid

[![Crates.io][crb]][crl]
[![Docs.rs][dcb]][dcl]
[![Linux Build Status][tcb]][tcl]
[![Windows Build Status][avb]][avl]
[![codevoc.io][cvb]][cvl]
[![MIT/Apache-2.0][lib]][lil]

[crb]: https://img.shields.io/crates/v/valid.svg
[dcb]: https://docs.rs/valid/badge.svg
[tcb]: https://travis-ci.org/innoave/valid.svg?branch=master
[avb]: https://ci.appveyor.com/api/projects/status/github/innoave/valid?branch=master&svg=true
[cvb]: https://codecov.io/gh/innoave/valid/branch/master/graph/badge.svg
[lib]: https://img.shields.io/badge/license-MIT%2FApache-blue.svg
[l1b]: https://img.shields.io/badge/license-MIT-blue.svg
[l2b]: https://img.shields.io/badge/license-Apache-blue.svg

[crl]: https://crates.io/crates/valid/
[dcl]: https://docs.rs/valid
[tcl]: https://travis-ci.org/innoave/valid/
[avl]: https://ci.appveyor.com/project/innoave/valid
[cvl]: https://codecov.io/github/innoave/valid?branch=master
[lil]: LICENSE
[l1l]: https://opensource.org/licenses/MIT
[l2l]: https://www.apache.org/licenses/LICENSE-2.0

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
* Support for `DateTime<Utc>` and `NaiveDate` of the [`chrono`] crate (optional crate feature
  [chrono])

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
 
Support for validating `NaiveDate` and `DateTime<Utc>` of the [`chrono`] crate:

```toml
[dependencies]
valid = { version = "0.1", features = ["chrono"] }
```
 
Theses crate features can be enabled in any combination. For detailed information on how to use this
crate see the [API documentation at docs.rs](https://docs.rs/valid).

[rust]: https://rust-lang.org
[`bigdecimal`]: https://crates.io/crates/bigdecimal
[`chrono`]: https://crates.io/crates/chrono
[`failure`]: https://crates.io/crates/failure
[_fluent_]: https://projectfluent.org/
[`serde`]: https://crates.io/crates/serde
[`valid`]: https://crates.io/crates/valid
