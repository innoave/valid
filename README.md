# valid
 
[![Latest Release]][crates.io]
[![Documentation]][docs.rs]
[![License]](LICENSE)
[![Build Status]][actions]
[![Test Coverage]][codecov]
[![Rustc Version 1.39+]][Rust 1.39]

[Latest Release]: https://img.shields.io/crates/v/valid.svg
[Documentation]: https://docs.rs/valid/badge.svg
[License]: https://img.shields.io/badge/license-MIT%2FApache_2.0-blue.svg
[Build Status]: https://img.shields.io/github/workflow/status/innoave/valid/CI/master
[Test Coverage]: https://codecov.io/gh/innoave/valid/branch/master/graph/badge.svg
[Rustc Version 1.39+]: https://img.shields.io/badge/rustc-1.39+-lightgray.svg

[crates.io]: https://crates.io/crates/valid/
[docs.rs]: https://docs.rs/valid
[MIT]: https://opensource.org/licenses/MIT
[Apache-2.0]: https://www.apache.org/licenses/LICENSE-2.0
[actions]: https://github.com/innoave/valid/actions?query=branch%3Amaster
[codecov]: https://codecov.io/github/innoave/valid?branch=master
[Rust 1.39]: https://blog.rust-lang.org/2019/11/07/Rust-1.39.0.html

**Let the business logic only accept valid values!**

* Validate custom types by composing primitive validation functions.

* Use one common API for validating all kind of business rules including aspects of the application 
  state.

* One common error type for all kind of constraint validations. It is designed to help with error
  messages that are meaningful to the user of an application. 

`valid` is a validation library for the [Rust] language. It let us write validation functions for
our custom types through composition of available validators. Any custom written validation function
again can be used to build validations for even more complex types.

The `valid` crate defines the types and traits to implement validation functions and use them to
validate our values. Additionally, it defines primitive constraints.
 
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
  "serde1")
* Support for widely used types of 3rd party crates through optional crate features
* Support for `BigDecimal` of the [`bigdecimal`] crate (optional crate feature "bigdecimal")
* Support for `BigInt` of the [`num-bigint`] crate (optional crate feature "num-bigint")
* Support for `DateTime` and `NaiveDate` of the [`chrono`] crate (optional crate feature "chrono")

## Usage
 
For detailed information on how to use [`valid`] including lots of examples see the
[API documentation at docs.rs](https://docs.rs/valid).

To use `valid` we add it as a dependency to our `Cargo.toml` file:

```toml
[dependencies]
valid = "0.3"
```

`valid` provides some of its functionality as optional crate features. Some features enable support
for validating a type that is provided by this crate. Other features enable the implementation of
additional constraints. To use any optional functionality we must enable the relevant crate feature
in our `Cargo.toml` file. All crate features can be enabled in any combination.

Here is an overview of all crate features:

| crate feature | supported types         | enabled constraints |
|---------------|-------------------------|---------------------|
| `bigint`      | `BigInt`                |                     |
| `bigdecimal`  | `BigDecimal`            |                     |
| `chrono`      | `DateTime`, `NaiveDate` |                     |
| `regex`       |                         | `Pattern`           | 

Additionally the "serde1" feature enables serialization and deserialization of `ValdiationError` 
using the [`serde`] crate:

```toml
[dependencies]
valid = { version = "0.3", features = ["serde1"] }
```


[rust]: https://rust-lang.org
[`bigdecimal`]: https://crates.io/crates/bigdecimal
[`chrono`]: https://crates.io/crates/chrono
[`failure`]: https://crates.io/crates/failure
[_fluent_]: https://projectfluent.org/
[`num-bigint`]: https://crates.io/crates/num-bigint
[`serde`]: https://crates.io/crates/serde
[`valid`]: https://crates.io/crates/valid
