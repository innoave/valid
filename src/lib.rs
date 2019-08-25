//! # `valid` - composable validation for custom types
//!
//! The `valid` crate let us write validation functions for our custom types
//! through composition of available validation functions. Any custom written
//! validation function again can be used to build validations for even more
//! complex types.
//!
//! The features and design of `valid` follow these goals:
//!
//! 1. Build more complex validations by composition of simplier validation
//!    functions.
//! 2. One common API for validating all kind of business rules.
//! 3. One common error type for all errors resulting from validating any kind
//!    business rule.
//! 4. Focus on the validation process. The presentation of validation results
//!    is not scope of this crate.
//! 5. No dependencies to 3rd party crates in the core API. Optional
//!    dependencies to support implementation of advanced constraints.
//!
//!
//! # Validation function, constraints and context
//!
//! The purpose of the validation is to confirm that a given value of type `T` is
//! compliant to a set of one or several constraints. To find out whether a
//! value is compliant we implement a function that checks some constraints on
//! the given value. The signature of such a validation function might look like
//! this:
//!
//! ```rust,ignore
//! fn validate<T, S, C>(value: T, context: S, constraint: C) -> Result<Validated<C, T>, ValidationError>;
//! ```
//!
//! This function takes a value `T`, a context `S`, and a constraint definition
//! `C` as input and returns a result that is either a `Validated<C, T>` or a
//! `ValidationError`. So we might define the validation function as a function
//! that converts a type `T` into some `Validated<C, T>` or returns an error if
//! one of the defined constraints is violated.
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
//! This crate provides some constraints. Those built in constraints are found
//! in the [`constraint`](constraint/index.html) module.
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
//! of an application.
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
//! [`ConstraintViolation`](enum.ConstraintViolation.html)s. A constraint
//! violation is an enum with 3 variants, one for each of the 3 categories we
//! talked about:
//!
//! * `ConstraintViolation::Field(InvalidaValue)`
//! * `ConstraintViolation::Relation(InvalidRelation)`
//! * `ConstraintViolation::State(InvalidState)`
//!
//!
//! # Generic implementation of constraints and properties
//!
//! The validation function of the [`Validate`](trait.Validate.html) trait is
//! applied to the combination of a constraint and a value. To validate some
//! constraint `C` for a value of type `T` the [`Validate`](trait.Validate.html)
//! trait must be implemented for the combination of these two types.
//!
//! Most primitive constraints evaluate one property of a value, such as the
//! length of a string or the number of fraction digits of a decimal number.
//! If we use traits to determine the relevant property of a value (lets call
//! them property traits) we can implement the [`Validate`](trait.Validate.html)
//! trait for all types `T` that implement the according property trait.
//!
//! This crate implements the [`Validate`](trait.Validate.html) trait for all
//! provided constraints for all generic types `T` that implement a certain
//! property trait. If there is a trait suitable as a property trait defined by
//! the std-lib we use that otherwise we define our own trait.
//!
//! The property traits defined by this crate are found in the
//! [`property`](property/index.html) module.
//!
//!
//! # Validating values using the built in constraints
//!
//! Successful validation of a simple variable:
//!
//! ```
//! use valid::Validate;
//! use valid::constraint::CharCount;
//!
//! let text = String::from("the answer is 42");
//!
//! let result = text.validate("text", &CharCount::MinMax(2, 16)).result();
//!
//! let validated = result.expect("successful validation");
//!
//! assert_eq!(validated.unwrap(), String::from("the answer is 42"));
//! ```
//!
//! Validating a pair of related values:
//!
//! ```
//! use valid::Validate;
//! use valid::constraint::MustMatch;
//!
//! let password = "s3cr3t".to_string();
//! let repeated = "s3cr3t".to_string();
//!
//! let result = (password, repeated).validate(("password", "repeated"), &MustMatch).result();
//!
//! let validated = result.expect("successful validation");
//!
//! assert_eq!(validated.unwrap(), ("s3cr3t".to_string(), "s3cr3t".to_string()));
//! ```
//!
//!
//! # Validation errors
//!
//! A failing validation returs a `ValidationError`. It contains a list of
//! constraint violations and and an optional message. The message is meant to
//! describe the context in which the validation has been performed. It is
//! helpful when validating a struct that represents an input form or a REST
//! command. In such cases the message would be something like "validating
//! registration form" or "invalid post entry command".
//!
//! The optional message is added by using the `with_message` function to finish
//! validation (instead of the `result` function).
//!
//! Here is an example for a validation that is failing with a message:
//!
//! ```
//! use valid::{Validate, ValidationError, InvalidValue, Field, Value};
//! use valid::constraint::CharCount;
//!
//! let text = String::from("the answer is 42");
//!
//! let result = text.validate("text", &CharCount::MinMax(2, 15)).with_message("validating `text`");
//!
//! assert_eq!(result, Err(ValidationError {
//!     message: Some("validating `text`".into()),
//!     violations: vec![InvalidValue {
//!         code: "invalid.char.count.max".into(),
//!         field: Field {
//!             name: "text".into(),
//!             actual: Some(Value::Integer(16)),
//!             expected: Some(Value::Integer(15)),
//!         }
//!     }.into()],
//! }));
//!
//! let error = result.unwrap_err();
//!
//! // ValidationError implements the Display trait
//! assert_eq!(error.to_string(), "validating `text`: invalid.char.count.max of text which is 16, expected to be 15");
//!
//! // ValidationError can be converted into `failure::Error`
//! let error: failure::Error = error.into();
//! ```
//!
//! `ValidationError` implements the `Display` and `std::error::Error` trait
//! from std-lib. It also can be converted into a `failure::Error` from the
//! `failure` crate.
//!
//! With the optional crate feature "serde1" enabled the `ValidationError`
//! implements `Serialize` and `Deserialize` from the `serde` crate. This
//! enables us to send errors to the client of an application via the network
//! or store them in a database.
//!
//!
//! # Composite validation functions
//!
//! Validating a struct with severals fields typically means to validate each
//! field and list all the violations if any are found. With `valid` we can
//! combine validations using the combinator methods, e.g. `Validation::and` and
//! `Validation::and_then`.
//!
//! Lets say we have a struct that represents a command to register a new user.
//!
//! ```
//! #[derive(Debug, Clone, PartialEq)]
//! struct RegisterUser {
//!     username: String,
//!     password: String,
//!     password2: String,
//!     age: i32,
//! }
//! ```
//!
//! Now we write a function that validates our struct and validate some instance
//! of the command:
//!
//! ```
//! # #[derive(Debug, Clone, PartialEq)]
//! # struct RegisterUser {
//! #     username: String,
//! #     password: String,
//! #     password2: String,
//! #     age: i32,
//! # }
//! use valid::{State, Validate, Validation, ValidationResult};
//! use valid::constraint::{Bound, CharCount, MustMatch};
//!
//! fn validate_register_user_cmd(command: RegisterUser) -> ValidationResult<(), RegisterUser> {
//!     let RegisterUser {
//!         username,
//!         password,
//!         password2,
//!         age,
//!     } = command;
//!
//!     username
//!         .validate("name", &CharCount::MinMax(4, 20))
//!         .and(password.validate("password", &CharCount::MinMax(6, 20)))
//!         .and_then(|(username, password)| {
//!             (password, password2)
//!                 .validate(("password", "password2"), &MustMatch)
//!                 .combine(username)
//!         })
//!         .and(age.validate("age", &Bound::ClosedRange(13, 199)))
//!         .map(|((username, (password, password2)), age)| RegisterUser {
//!             username,
//!             password,
//!             password2,
//!             age,
//!         })
//!         .with_message("validating register user command")
//! }
//!
//! let register_user = RegisterUser {
//!     username: "jane.doe".into(),
//!     password: "s3cr3t".into(),
//!     password2: "s3cr3t".into(),
//!     age: 42,
//! };
//! let original = register_user.clone();
//!
//! let result = validate_register_user_cmd(register_user);
//! let validated = result.unwrap();
//!
//! assert_eq!(validated.unwrap(), original);
//! ```
//!
//! Alternatively we can implement the `Validate` trait and do the same
//! validation:
//!
//! ```
//! # #[derive(Debug, Clone, PartialEq)]
//! # struct RegisterUser {
//! #     username: String,
//! #     password: String,
//! #     password2: String,
//! #     age: i32,
//! # }
//! use valid::{State, Validate, Validation};
//! use valid::constraint::{Bound, CharCount, MustMatch};
//!
//! struct RegistrationForm;
//!
//! impl Validate<RegistrationForm, State<()>> for RegisterUser {
//!     fn validate(self, context: impl Into<State<()>>, constraint: &RegistrationForm) -> Validation<RegistrationForm, Self> {
//!         let RegisterUser {
//!             username,
//!             password,
//!             password2,
//!             age,
//!         } = self;
//!
//!         username
//!             .validate("name", &CharCount::MinMax(4, 20))
//!             .and(password.validate("password", &CharCount::MinMax(6, 20)))
//!             .and_then(|(username, password)| {
//!                 (password, password2)
//!                     .validate(("password", "password2"), &MustMatch)
//!                     .combine(username)
//!             })
//!             .and(age.validate("age", &Bound::ClosedRange(13, 199)))
//!             .map(|((username, (password, password2)), age)| RegisterUser {
//!                 username,
//!                 password,
//!                 password2,
//!                 age,
//!             })
//!     }
//! }
//!
//! let register_user = RegisterUser {
//!     username: "jane.doe".into(),
//!     password: "s3cr3t".into(),
//!     password2: "s3cr3t".into(),
//!     age: 42,
//! };
//! let original = register_user.clone();
//!
//! let result = register_user
//!     .validate((), &RegistrationForm)
//!     .with_message("validating register user command");
//!
//! let validated = result.unwrap();
//!
//! assert_eq!(validated.unwrap(), original);
//! ```
//!
//! In terms of boilerplate code there is not much difference to the plain
//! function in the previous example. The code that actually does the validation
//! is exactly the same.
//!
//!
//! # Custom constraints
//!
//! To implement a custom constraint we first define a struct that represents
//! the constraint. The constraint usually holds parameters of the constraint
//! such as allowed limits. Then we implement the `Validate` trait for the
//! combination of our new constraint and any type that should be validated for
//! this constraint.
//!
//! Lets say we have an enum that represents the days of a week.
//!
//! ```
//! #[derive(Debug, PartialEq)]
//! enum Weekday {
//!     Monday,
//!     Tuesday,
//!     Wednesday,
//!     Thursday,
//!     Friday,
//!     Saturday,
//!     Sunday,
//! }
//! ```
//!
//! For some usage in our application only workdays are allowed. But it depends
//! on some configuration parameter whether saturday is considered a workday or
//! not. So we define the enum `Workday` with two variants to represent our
//! constraint.
//!
//! ```
//! enum Workday {
//!     InclSaturday,
//!     ExclSaturday,
//! }
//! ```
//!
//! To be able to validate whether of value of type `Weekday` is compliant to
//! our `Workday` constraint we implement the `Validate` trait for the
//! `Weekday` trait.
//!
//! ```
//! # #[derive(Debug, PartialEq)]
//! # enum Weekday {
//! #     Monday,
//! #     Tuesday,
//! #     Wednesday,
//! #     Thursday,
//! #     Friday,
//! #     Saturday,
//! #     Sunday,
//! # }
//! # enum Workday {
//! #     InclSaturday,
//! #     ExclSaturday,
//! # }
//! use valid::{Validate, FieldName, Validation, invalid_value};
//!
//! impl Validate<Workday, FieldName> for Weekday {
//!     fn validate(self, name: impl Into<FieldName>, constraint: &Workday) -> Validation<Workday, Self> {
//!         match (&self, constraint) {
//!             (Weekday::Sunday, _) => Validation::failure(vec![
//!                 invalid_value("invalid.workday.incl.saturday", name, "sunday".to_string(), "monday - friday".to_string())
//!             ]),
//!             (Weekday::Saturday, Workday::ExclSaturday) => Validation::failure(vec![
//!                 invalid_value("invalid.workday.excl.saturday", name, "saturday".to_string(), "monday - friday".to_string())
//!             ]),
//!             (_, _) => Validation::success(self),
//!         }
//!     }
//! }
//! ```
//!
//! Now we can validate some values for being workdays.
//!
//! ```
//! # #[derive(Debug, PartialEq)]
//! # enum Weekday {
//! #     Monday,
//! #     Tuesday,
//! #     Wednesday,
//! #     Thursday,
//! #     Friday,
//! #     Saturday,
//! #     Sunday,
//! # }
//! # enum Workday {
//! #     InclSaturday,
//! #     ExclSaturday,
//! # }
//! # use valid::{Validate, FieldName, Validation, invalid_value};
//! #
//! # impl Validate<Workday, FieldName> for Weekday {
//! #     fn validate(self, name: impl Into<FieldName>, constraint: &Workday) -> Validation<Workday, Self> {
//! #         match (&self, constraint) {
//! #             (Weekday::Sunday, _) => Validation::failure(vec![
//! #                 invalid_value("invalid.workday.incl.saturday", name, "sunday".to_string(), "monday - friday".to_string())
//! #             ]),
//! #             (Weekday::Saturday, Workday::ExclSaturday) => Validation::failure(vec![
//! #                 invalid_value("invalid.workday.excl.saturday", name, "saturday".to_string(), "monday - friday".to_string())
//! #             ]),
//! #             (_, _) => Validation::success(self),
//! #         }
//! #     }
//! # }
//! let validated = Weekday::Monday.validate("day of release", &Workday::ExclSaturday).result()
//!     .expect("a valid workday");
//!
//! assert_eq!(validated.unwrap(), Weekday::Monday);
//!
//! let result = Weekday::Saturday.validate("day of release", &Workday::ExclSaturday).result();
//!
//! assert!(result.is_err());
//!
//! let result = Weekday::Saturday.validate("day of release", &Workday::InclSaturday).result();
//!
//! assert!(result.is_ok());
//! ```
//!
//!
//! # Validation depending on application state
//!
//! A business rule may require that a certain field must be unique within the
//! application, such as the username in the registration command. Another
//! business rule may require that an operation may be performed only once, such
//! as reverting a financial transaction. These are examples where some state
//! information is needed to validate the business rule. Following the goal of
//! `valid` to provide one common API and one error type for all kind of
//! validations, it must be possible to validate those kind of business rules
//! as well. Lets have a look at an example.
//!
//! Lets say we have a command for reverting the booking of a reservation. The
//! command struct may look like this.
//!
//! ```
//! struct RevertReservation {
//!     reservation_id: String,
//! }
//! ```
//!
//! The constraint for our business rule is that a reservation must not be
//! reverted already.
//!
//! ```
//! struct IsNotReverted;
//! ```
//!
//! To determine whether a reservation has been reverted already we need a
//! repository that keep track of the reservations and its state.
//!
//! ```
//! mod repo {
//!     use std::collections::HashMap;
//!
//!     pub struct ReservationList {
//!         reverted_reservations: HashMap<String, bool>,
//!     }
//!
//!     impl ReservationList {
//!         pub fn is_reservation_reverted(&self, reservation_code: &str) -> bool {
//!             self.reverted_reservations.get(reservation_code).copied().unwrap_or(false)
//!         }
//!     }
//! }
//! ```
//!
//! Now the interesting part. The implementation of the `Validate` trait. This
//! may look like:
//!
//! ```
//! # #[derive(Debug, Clone, PartialEq)]
//! # struct RevertReservation {
//! #     reservation_code: String,
//! # }
//! # struct IsNotReverted;
//! # mod repo {
//! #     use std::collections::HashMap;
//! #
//! #     pub struct ReservationList {
//! #         reverted_reservations: HashMap<String, bool>,
//! #     }
//! #
//! #     impl ReservationList {
//! #         pub fn new() -> Self {
//! #             Self { reverted_reservations: HashMap::new() }
//! #         }
//! #         pub fn is_reservation_reverted(&self, reservation_code: &str) -> bool {
//! #             self.reverted_reservations.get(reservation_code).copied().unwrap_or(false)
//! #         }
//! #     }
//! # }
//! use valid::{State, Validate, Validation, invalid_state};
//! use repo::ReservationList;
//!
//! impl<'a> Validate<IsNotReverted, State<&'a ReservationList>> for RevertReservation {
//!     fn validate(self, context: impl Into<State<&'a ReservationList>>, constraint: &IsNotReverted)
//!         -> Validation<IsNotReverted, Self>
//!     {
//!         let context = context.into();
//!         if context.is_reservation_reverted(&self.reservation_code) {
//!             Validation::failure(vec![invalid_state("constraint_violation_reservation_already_reverted", vec![])])
//!         } else {
//!             Validation::success(self)
//!         }
//!     }
//! }
//!
//! let reservation_list = ReservationList::new();
//!
//! let revert_reservation = RevertReservation {
//!     reservation_code: "HRS1900123456".into(),
//! };
//! let original_cmd = revert_reservation.clone();
//!
//! let result = revert_reservation.validate(&reservation_list, &IsNotReverted).result();
//! let validated = result.expect("validating revert reservation command");
//!
//! assert_eq!(validated.unwrap(), original_cmd);
//! ```

#![doc(html_root_url = "https://docs.rs/valid/0.1.0")]
#![warn(
    bare_trait_objects,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
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

// re-export the core API
pub use crate::core::{
    invalid_optional_value, invalid_relation, invalid_state, invalid_value, ConstraintViolation,
    Field, FieldName, InvalidRelation, InvalidState, InvalidValue, RelatedFields, State, Validate,
    Validated, Validation, ValidationError, ValidationResult, Value,
};
