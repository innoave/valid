//! The core API of the `valid` crate

#[cfg(feature = "bigdecimal")]
use bigdecimal::BigDecimal;
#[cfg(feature = "chrono")]
use chrono::{DateTime, NaiveDate, Utc};
#[cfg(feature = "serde1")]
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display, Write};
use std::iter::FromIterator;
use std::marker::PhantomData;
use std::ops::Deref;

/// A wrapper type to express that the value of type `T` has been validated by
/// the constraint `C`.
///
/// The idea is that an instance of `Validated<C, T>` can only be obtained by
/// validating a value of type `T` using the constraint `C`. There is no way to
/// construct an instance of `Validated` directly.[^1]
///
/// It follows the new type pattern and can be de-referenced to a immutable
/// reference to its inner value or unwrapped to get the owned inner value.
///
/// In an application we can make use of the type system to assure that only
/// valid values of some type can be input to some function performing some
/// domain related things.
///
/// For example, lets assume we have a function that expects a valid email
/// address as input. We could write the function like:
///
/// ```
/// fn send_email(to: String, message: String) {
///     unimplemented!()
/// }
/// ```
///
/// The problem with this approach is, that we can never be sure that the string
/// input for the `to` argument is a valid email address.
///
/// Lets rewrite the same function using `Validated<Email, String>`.
///
/// ```ignore //TODO remove ignore when Email constraint is implemented
/// use valid::Validated;
///
/// fn send_email(to: Validated<Email, String>, message: String) {
///     unimplemented!()
/// }
/// ```
///
/// Due to we can not instantiate `Validated` directly using some constructor
/// function like `Validated(email)` or `Validated::new(email)` we need to use
/// a validation function like:
///
/// ```ignore //TODO remove ignore when Email constraint is implemented
/// # fn send_email(to: Validated<Email, String>, message: String) {
/// #     unimplemented!()
/// # }
/// use valid::{Validated, Validate};
///
/// let to_addr = "jane.doe@email.net".to_string().validate("email", Email).result(None)
///         .expect("valid email address");
///
/// send_email(to_addr, "some message".into());
/// ```
///
/// Now we can be sure that the variable `to_addr` contains a valid email
/// address.
///
/// To further make use of meaningful new types we might define a custom new
/// type for email addresses, that can only be constructed from a validated
/// value like so:
///
/// ```ignore //TODO remove ignore when Email constraint is implemented
/// # fn send_email(to: EmailAddress, message: String) {
/// #     unimplemented!()
/// # }
/// use valid::{Validate, Validated};
///
/// mod domain_model {
///     use valid::Validated;
///     pub struct EmailAddress(String);
///
///     impl From<Validated<Email, String>> for EmailAddress {
///         fn from(value: Validated<String>) -> Self {
///             EmailAddress(value.unwrap())
///         }
///     }
/// }
///
/// let validated = "jane.doe@email.net".to_string().validate("email", Email).result(None)
///         .expect("valid email address");
///
/// let to_addr = EmailAddress::from(validated);
///
/// send_email(to_addr, "some message".into());
/// ```
///
/// Due to the type `EmailAddress` is defined in another module it can only be
/// constructed from a `Validated<Email, String>`.
///
/// [^1]: Actually there is a way to construct an instance of `Validated`
///       without actually doing any validation: we can use the
///       `Validation::success` method (see unit tests on how it can be done)
///       We need this method for custom implementations of the `Validate`
///       trait. Unfortunately I have no idea how to prevent this.
///       Fortunately such code can be found by (automated) code review.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Validated<C, T>(PhantomData<C>, T);

impl<C, T> Validated<C, T> {
    /// Unwraps the original value that has been validated
    pub fn unwrap(self) -> T {
        self.1
    }
}

impl<C, T> Deref for Validated<C, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.1
    }
}

/// The validation function validates whether the given value complies to the
/// specified constraint.
///
/// It returns a `Validation` value that may be used to perform further
/// validations using its combinator methods `and` or `and_then` or get the
/// final result by calling the `result` method.
///
/// The context provides additional information to perform the validation,
/// for example a lookup table or some state information. It may also hold
/// parameters needed to provide additional parameters to the error in case
/// of a constraint violation. (see the crate level documentation for more
/// details on how to use the context)
///
/// see the crate level documentation for details about how to implement a the
/// `Validate` trait for custom constraints and custom types.
pub trait Validate<C, S>
where
    S: Context,
    Self: Sized,
{
    /// Validates this value for being compliant to the specified constraint
    /// `C` in the given context `S`.
    fn validate(self, context: impl Into<S>, constraint: &C) -> Validation<C, Self>;
}

mod private {
    pub trait Sealed {}

    impl<T> Sealed for T where T: super::Context {}
}

/// Trait to mark structs as context for validation functions.
///
/// This trait is sealed an can not be implemented for types outside this crate.
pub trait Context: private::Sealed {}

/// Represents the field level context for validation functions. Its value is
/// the name of the field to be validated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldName(pub Cow<'static, str>);

impl Context for FieldName {}

impl<A> From<A> for FieldName
where
    A: Into<Cow<'static, str>>,
{
    fn from(value: A) -> Self {
        FieldName(value.into())
    }
}

impl Deref for FieldName {
    type Target = Cow<'static, str>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FieldName {
    /// Unwraps this field name context and returns the field name itself
    pub fn unwrap(self) -> Cow<'static, str> {
        self.0
    }
}

/// Represents a pair of related fields as context for validation functions.
/// It holds the names of the two related fields that are validated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelatedFields(pub Cow<'static, str>, pub Cow<'static, str>);

impl Context for RelatedFields {}

impl<A, B> From<(A, B)> for RelatedFields
where
    A: Into<Cow<'static, str>>,
    B: Into<Cow<'static, str>>,
{
    fn from((value1, value2): (A, B)) -> Self {
        RelatedFields(value1.into(), value2.into())
    }
}

impl RelatedFields {
    /// Unwraps this related fields context and returns the 2 field names
    pub fn unwrap(self) -> (Cow<'static, str>, Cow<'static, str>) {
        (self.0, self.1)
    }

    /// Returns a reference to the name of the first field
    pub fn first(&self) -> &str {
        &self.0
    }

    /// Returns a reference to the name of the second field
    pub fn second(&self) -> &str {
        &self.1
    }
}

/// Represents the state context for validation functions. Its value is the
/// state information needed to execute the validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct State<S>(pub S);

impl<S> Context for State<S> {}

impl<S> From<S> for State<S> {
    fn from(value: S) -> Self {
        State(value)
    }
}

impl<S> Deref for State<S> {
    type Target = S;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<S> State<S> {
    /// Unwraps this state context and returns the state information itself
    pub fn unwrap(self) -> S {
        self.0
    }
}

enum InnerValidation<C, T> {
    Success(PhantomData<C>, T),
    Failure(Vec<ConstraintViolation>),
}

/// State of an ongoing validation.
///
/// It provides combinator methods like [`and`] and [`and_then`] to combine
/// validation steps to complex validations and accumulates all constraint
/// violations found by the executed validations.
///
/// The result of a validation can be obtained by calling the [`result`] method.
///
/// see the crate level documentation for details and examples on how to use
/// the methods provided by this struct.
///
/// [`and`]: #method.and
/// [`and_then`]: #method.and_then
/// [`result`]: #method.result
pub struct Validation<C, T>(InnerValidation<C, T>);

impl<C, T> Debug for Validation<C, T>
where
    C: Debug,
    T: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            InnerValidation::Success(constraint, value) => {
                write!(f, "Validation(Success({:?}, {:?}))", constraint, value)
            }
            InnerValidation::Failure(violations) => {
                write!(f, "Validation(Failure({:?}))", violations)
            }
        }
    }
}

impl<C, T> Validation<C, T> {
    /// Constructs a `Validation` for a successful validation step.
    ///
    /// This method is provided to enable users of this crate to implement
    /// custom validation function.
    pub fn success(valid: T) -> Self {
        Validation(InnerValidation::Success(PhantomData, valid))
    }

    /// Constructs a `Validation` for a failed validation step.
    ///
    /// This method is provided to enable users of this crate to implement
    /// custom validation function.
    pub fn failure(constraint_violations: impl IntoIterator<Item = ConstraintViolation>) -> Self {
        Validation(InnerValidation::Failure(Vec::from_iter(
            constraint_violations.into_iter(),
        )))
    }

    /// Finishes a validation and returns the result of the validation.
    ///
    /// A validation may comprise multiple validation steps that are combined
    /// using the combinator methods of this struct. After all steps are
    /// executed this method can be called to get the [`ValidationResult`]
    ///
    /// [`ValidationResult`]: type.ValidationResult.html
    pub fn result(self) -> ValidationResult<C, T> {
        match self.0 {
            InnerValidation::Success(_c, entity) => Ok(Validated(_c, entity)),
            InnerValidation::Failure(violations) => Err(ValidationError {
                message: None,
                violations,
            }),
        }
    }

    /// Finishes a validation providing a message and returns the result.
    ///
    /// A validation may comprise multiple validation steps that are combined
    /// using the combinator methods of this struct. After all steps are
    /// executed this method can be called to get the [`ValidationResult`]
    ///
    /// In case of an error the [`ValidationError`] will contain the given
    /// message. It is meant to describe the context in which the validation has
    /// been executed. E.g when validating a struct that represents an input
    /// form the message would be something like "validating registration form"
    /// or when validating a struct that represents a REST command the message
    /// would be something like "invalid post entry command".
    ///
    /// [`ValidationResult`]: type.ValidationResult.html
    /// [`ValidationError`]: struct.ValidationError.html
    pub fn with_message(self, message: impl Into<Cow<'static, str>>) -> ValidationResult<C, T> {
        match self.0 {
            InnerValidation::Success(_c, entity) => Ok(Validated(_c, entity)),
            InnerValidation::Failure(violations) => Err(ValidationError {
                message: Some(message.into()),
                violations,
            }),
        }
    }

    /// Combines a value that needs no further validation with the validation
    /// result.
    ///
    /// This method may be especially useful in combination with the
    /// [`and_then`] combinator method. See the crate level documentation for
    /// an example.
    ///
    /// [`and_then`]: #method.and_then
    pub fn combine<U>(self, value: U) -> Validation<C, (U, T)> {
        match self.0 {
            InnerValidation::Success(_, entity) => Validation::success((value, entity)),
            InnerValidation::Failure(violations) => Validation::failure(violations),
        }
    }

    /// Maps the validated values into another type.
    ///
    /// This method is used for complex validations that validate multiple
    /// fields of a struct and the result should be mapped back into this
    /// struct. See the crate level documentation for an example.
    pub fn map<D, U>(self, convert: impl Fn(T) -> U) -> Validation<D, U> {
        match self.0 {
            InnerValidation::Success(_, entity) => Validation::success(convert(entity)),
            InnerValidation::Failure(violations) => Validation::failure(violations),
        }
    }

    /// Combines this validation with another validation unconditionally.
    ///
    /// The other validation is executed regardless whether this validation has
    /// been successful or not.
    ///
    /// The resulting validation is only successful if itself was successful
    /// and the other validation is also successful. Any constraint violations
    /// found either by this validation or the other validation are accumulated.
    ///
    /// See the crate level documentation for an example.
    pub fn and<D, U>(self, other: Validation<D, U>) -> Validation<D, (T, U)> {
        match (self.0, other.0) {
            (InnerValidation::Success(_, value1), InnerValidation::Success(_, value2)) => {
                Validation::success((value1, value2))
            }
            (InnerValidation::Failure(violations), InnerValidation::Success(_, _)) => {
                Validation::failure(violations)
            }
            (InnerValidation::Success(_, _), InnerValidation::Failure(violations)) => {
                Validation::failure(violations)
            }
            (InnerValidation::Failure(mut violations), InnerValidation::Failure(violations2)) => {
                violations.extend(violations2);
                Validation::failure(violations)
            }
        }
    }

    /// Combines this validation with another validation conditionally.
    ///
    /// The other validation is only executed if this validation has been
    /// successful.
    ///
    /// See the crate level documentation for an example.
    pub fn and_then<D, U>(self, next: impl FnOnce(T) -> Validation<D, U>) -> Validation<D, U> {
        match self.0 {
            InnerValidation::Success(_, value1) => next(value1),
            InnerValidation::Failure(error) => Validation::failure(error),
        }
    }
}

/// A `Value` represents a value of certain type.
///
/// The purpose of a `Value` is to include field values or parameters in
/// [`ConstraintViolation`]s in a type that allows to display the value in a
/// localized format as part of a user facing error message.
///
/// It has variants for the basic types that are used in most applications.
///
/// Important types of 3rd party crates are supported through optional crate
/// features:
///
/// | 3rd party crate | supported type  | crate feature |
/// |-----------------|-----------------|---------------|
/// | [`bigdecimal`]  | `BigDecimal`    | `bigdecimal`  |
/// | [`chrono`]      | `NaiveDate`     | `chrono`      |
/// | [`chrono`]      | `DateTime<Utc>` | `chrono`      |
///
/// The `From` trait is implemented for the underlying types. Additionally
/// there are implementations of the `From` trait for the primitive types `i8`,
/// `i16`, `i64`, `u8`, `u16`, `u32`, `u64`.
///
/// `u32` values greater than `i32::max_value()` are converted to `Long(i64)`.
///
/// # Panics
///
/// Converting `u64` values greater than `i64::max_value()` has an unreliable
/// behavior and might panic.
///
/// # Notes
///
/// The list of supported types is very opinionated and may not fit all kind of
/// applications. Please file and issue if you feel that support for another
/// type may be useful!
///
/// [`ConstraintViolation`]: enum.ConstraintViolation.html
/// [`bigdecimal`]: https://crates.io/crates/bigdecimal
/// [`chrono`]: https://crates.io/crates/chrono
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// a string value
    String(String),
    /// a 32bit signed integer value
    Integer(i32),
    /// a 64bit signed integer value
    Long(i64),
    /// a 32bit float value
    Float(f32),
    /// a 64bit float value
    Double(f64),
    /// a boolean value
    Boolean(bool),
    /// a decimal value
    #[cfg(feature = "bigdecimal")]
    Decimal(BigDecimal),
    /// a date value
    #[cfg(feature = "chrono")]
    Date(NaiveDate),
    /// a value with date, time and timezone
    #[cfg(feature = "chrono")]
    DateTime(DateTime<Utc>),
}

impl Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::String(value) => write!(f, "{}", value),
            Value::Integer(value) => write!(f, "{}", value),
            Value::Long(value) => write!(f, "{}", value),
            Value::Float(value) => write!(f, "{}", value),
            Value::Double(value) => write!(f, "{}", value),
            Value::Boolean(value) => write!(f, "{}", value),
            #[cfg(feature = "bigdecimal")]
            Value::Decimal(value) => write!(f, "{}", value),
            #[cfg(feature = "chrono")]
            Value::Date(value) => write!(f, "{}", value),
            #[cfg(feature = "chrono")]
            Value::DateTime(value) => write!(f, "{}", value),
        }
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(value)
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Value::Integer(value)
    }
}

impl From<i16> for Value {
    fn from(value: i16) -> Self {
        Value::Integer(i32::from(value))
    }
}

impl From<u16> for Value {
    fn from(value: u16) -> Self {
        Value::Integer(i32::from(value))
    }
}

impl From<i8> for Value {
    fn from(value: i8) -> Self {
        Value::Integer(i32::from(value))
    }
}

impl From<u8> for Value {
    fn from(value: u8) -> Self {
        Value::Integer(i32::from(value))
    }
}

impl From<u32> for Value {
    fn from(value: u32) -> Self {
        if value > i32::max_value() as u32 {
            Value::Long(i64::from(value))
        } else {
            Value::Integer(value as i32)
        }
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::Long(value)
    }
}

//TODO unreliable conversion - should be removed!
impl From<u64> for Value {
    fn from(value: u64) -> Self {
        assert!(
            value <= i64::max_value() as u64,
            "u64 value to big to be converted to i64"
        );
        Value::Long(value as i64)
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Value::Float(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Double(value)
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Boolean(value)
    }
}

#[cfg(feature = "bigdecimal")]
impl From<BigDecimal> for Value {
    fn from(value: BigDecimal) -> Self {
        Value::Decimal(value)
    }
}

#[cfg(feature = "chrono")]
impl From<NaiveDate> for Value {
    fn from(value: NaiveDate) -> Self {
        Value::Date(value)
    }
}

#[cfg(feature = "chrono")]
impl From<DateTime<Utc>> for Value {
    fn from(value: DateTime<Utc>) -> Self {
        Value::DateTime(value)
    }
}

fn option_to_string<T: Display>(optional_value: Option<&T>) -> String {
    match optional_value {
        Some(value) => value.to_string(),
        None => "(n.a.)".to_string(),
    }
}

fn array_to_string<T: Display>(array: &[T]) -> String {
    let separator = ", ";
    let len = array.len();
    let mut iter = array.iter();
    match iter.next() {
        None => String::new(),
        Some(first_elem) => {
            let mut result = String::with_capacity(len * separator.len());
            write!(&mut result, "{}", first_elem).unwrap();
            for elem in iter {
                result.push_str(separator);
                write!(&mut result, "{}", elem).unwrap();
            }
            result
        }
    }
}

/// Details about a field.
///
/// This struct is used to provide more details in [`ConstraintViolation`]s.
///
/// [`ConstraintViolation`]: enum.ConstraintViolation.html
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    /// The name of the field
    pub name: Cow<'static, str>,

    /// The actual value of the field
    pub actual: Option<Value>,

    /// An example for an expected value
    pub expected: Option<Value>,
}

impl Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "field: {}, actual: {}, expected: {}",
            self.name,
            option_to_string(self.actual.as_ref()),
            option_to_string(self.expected.as_ref())
        )
    }
}

/// Holds details about a constraint violation found by validating a constraint
/// in the [`FieldName`] context.
///
/// [`FieldName`]: struct.FieldName.html
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct InvalidValue {
    /// Error code that identifies the exact error.
    ///
    /// A client that receives the constraint violation should be able to
    /// interpret this error code.
    pub code: Cow<'static, str>,

    /// Details about the field having a value that violates a constraint.
    pub field: Field,
}

impl Display for InvalidValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} of {} which is {}, expected to be {}",
            self.code,
            self.field.name,
            option_to_string(self.field.actual.as_ref()),
            option_to_string(self.field.expected.as_ref())
        )
    }
}

/// Holds details about a constraint violation found by validating a constraint
/// in the [`RelatedFields`] context.
///
/// [`RelatedFields`]: struct.RelatedFields.html
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct InvalidRelation {
    /// Error code that identifies the exact error.
    ///
    /// A client that receives the constraint violation should be able to
    /// interpret this error code.
    pub code: Cow<'static, str>,

    /// Details about the first of the pair of related fields
    pub field1: Field,

    /// Details about the second of the pair of related fields
    pub field2: Field,
}

impl Display for InvalidRelation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} of {} which is {} and {} which is {}",
            self.code,
            self.field1.name,
            option_to_string(self.field1.actual.as_ref()),
            self.field2.name,
            option_to_string(self.field2.actual.as_ref())
        )
    }
}

/// Holds details about a constraint violation found by validating a constraint
/// in the [`State`] context.
///
/// [`State`]: struct.State.html
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct InvalidState {
    /// Error code that identifies the exact error.
    ///
    /// A client that receives the constraint violation should be able to
    /// interpret this error code.
    pub code: Cow<'static, str>,

    /// A list of parameters that may be used to provide more meaningful error
    /// messages to the user of an application
    pub params: Vec<Field>,
}

impl Display for InvalidState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} for parameters: {}",
            self.code,
            array_to_string(&self.params)
        )
    }
}

/// Represents a constraint violation found by some validation function.
///
/// The variants provide different details about a constraint violation. As
/// described in the crate level documentation this crate considers 3 categories
/// of business rules or constraints. Violations of constraints of the different
/// categories might provide different details about the validation.
///
/// For example a field validation might provide the field name, the actual value
/// and an example for the expected value. A constraint on the relation of a
/// pair of fields might provide the names of the 2 fields. Stateful constraints
/// may provide a list of parameters that might be useful to describe the
/// reason of the constraint violation.
///
/// An implementation of a constraint should choose the most appropriate
/// context for the kind of business rule it is implementing. Here is a table
/// that shows the relation of the implemented context and the variant of the
/// constraint violation type.
///
/// | Context            | Constraint Violation | Construction Method      |
/// |--------------------|----------------------|--------------------------|
/// | [`FieldName`]      | `Field`              | [`invalid_value`]<br/>[`invalid_optional_value`] |
/// | [`RelatedFields`]  | `Relation`           | [`invalid_relation`]     |
/// | [`State<S>`]       | `State`              | [`invalid_state`]        |
///
/// The construction methods are a convenient way to construct
/// `ConstraintViolation`s.
///
/// `ConstraintViolation` can be serialized and deserialized using the [`serde`]
/// crate. To use the `serde` support the optional crate feature `serde1` must
/// be enabled.
///
/// [`FieldName`]: struct.FieldName.html
/// [`RelatedFields`]: struct.RelatedFields.html
/// [`State<S>`]: struct.State.html
/// [`invalid_value`]: fn.invalid_value.html
/// [`invalid_optional_value`]: fn.invalid_optional_value.html
/// [`invalid_relation`]: fn.invalid_relation.html
/// [`invalid_state`]: fn.invalid_state.html
/// [`serde`]: https://crates.io/crates/serde
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum ConstraintViolation {
    /// Violation of a constraint validated in the `FieldName` context
    Field(InvalidValue),
    /// Violation of a constraint validated in the `RelatedField` context
    Relation(InvalidRelation),
    /// Violation of a constraint validated in the `State` context
    State(InvalidState),
}

impl Display for ConstraintViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConstraintViolation::Field(value) => write!(f, "{}", value),
            ConstraintViolation::Relation(value) => write!(f, "{}", value),
            ConstraintViolation::State(value) => write!(f, "{}", value),
        }
    }
}

impl From<InvalidValue> for ConstraintViolation {
    fn from(invalid_value: InvalidValue) -> Self {
        ConstraintViolation::Field(invalid_value)
    }
}

impl From<InvalidRelation> for ConstraintViolation {
    fn from(invalid_relation: InvalidRelation) -> Self {
        ConstraintViolation::Relation(invalid_relation)
    }
}

impl From<InvalidState> for ConstraintViolation {
    fn from(invalid_state: InvalidState) -> Self {
        ConstraintViolation::State(invalid_state)
    }
}

/// The error type returned if the validation finds any constraint violation.
///
/// It holds a list of constraint violations and an optional message. The
/// message is meant to describe the context in which the validation has been
/// performed. It is helpful when validating a struct that represents an input
/// form or a REST command. In such cases the message would be something like
/// "validating registration form" or "invalid post entry command".
///
/// The `Display` and `Error` traits are implemented to be compatible with most
/// error management concepts. It can be converted into `failure::Error` using
/// `From` or `Into` conversion traits.
///
/// It can be serialized and deserialized using the [`serde`] crate. To enable
/// `serde` support the optional crate feature `serde1` must be enabled.
///
/// [`serde`]: https://crates.io/crates/serde
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct ValidationError {
    /// Message that describes the context in which the validation has been
    /// executed
    pub message: Option<Cow<'static, str>>,

    /// A list of constraint violations found during validation
    pub violations: Vec<ConstraintViolation>,
}

impl Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.message {
            Some(message) => write!(f, "{}: {}", message, array_to_string(&self.violations)),
            None => write!(f, "{}", array_to_string(&self.violations)),
        }
    }
}

impl Error for ValidationError {}

impl ValidationError {
    /// Merges this validation error with another validation error and returns
    /// a new validation error that contains all constraint violations from
    /// both errors merged into one list.
    pub fn merge(mut self, other: ValidationError) -> Self {
        //TODO find a more reasonable solution for merging messages
        self.message = match (self.message, other.message) {
            (_, Some(msg2)) => Some(msg2),
            (Some(msg1), None) => Some(msg1),
            (None, None) => None,
        };
        self.violations.extend(other.violations);
        self
    }
}

/// Type alias for the validation result for shorter type annotations.
pub type ValidationResult<C, T> = Result<Validated<C, T>, ValidationError>;

/// Convenience method to construct a [`ConstraintViolation`] for a validation
/// performed in the [`FieldName`] context.
///
/// Use this method if the field value is mandatory. If the field is of type
/// `Option<T>` consider using the [`invalid_optional_value`] method instead.
///
/// [`ConstraintViolation`]: enum.ConstraintViolation.html
/// [`FieldName`]: struct.FieldName.html
/// [`invalid_optional_value`]: fn.invalid_optional_value.html
pub fn invalid_value(
    code: impl Into<Cow<'static, str>>,
    field_name: impl Into<FieldName>,
    actual_value: impl Into<Value>,
    expected_value: impl Into<Value>,
) -> ConstraintViolation {
    ConstraintViolation::Field(InvalidValue {
        code: code.into(),
        field: Field {
            name: field_name.into().unwrap(),
            actual: Some(actual_value.into()),
            expected: Some(expected_value.into()),
        },
    })
}

/// Convenience method to construct a [`ConstraintViolation`] for a validation
/// performed in the [`FieldName`] context.
///
/// Use this method if the field value is optional. If the field is not of type
/// `Option<T>` consider using the [`invalid_value`] method instead.
///
/// [`ConstraintViolation`]: enum.ConstraintViolation.html
/// [`FieldName`]: struct.FieldName.html
/// [`invalid_value`]: fn.invalid_value.html
pub fn invalid_optional_value(
    code: impl Into<Cow<'static, str>>,
    field_name: impl Into<FieldName>,
    actual: Option<Value>,
    expected: Option<Value>,
) -> ConstraintViolation {
    ConstraintViolation::Field(InvalidValue {
        code: code.into(),
        field: Field {
            name: field_name.into().unwrap(),
            actual,
            expected,
        },
    })
}

/// Convenience method to construct a [`ConstraintViolation`] for a validation
/// performed in the [`RelatedFields`] context.
///
/// [`ConstraintViolation`]: enum.ConstraintViolation.html
/// [`RelatedFields`]: struct.RelatedFields.html
pub fn invalid_relation(
    code: impl Into<Cow<'static, str>>,
    field_name1: impl Into<Cow<'static, str>>,
    field_value1: impl Into<Value>,
    field_name2: impl Into<Cow<'static, str>>,
    field_value2: impl Into<Value>,
) -> ConstraintViolation {
    ConstraintViolation::Relation(InvalidRelation {
        code: code.into(),
        field1: Field {
            name: field_name1.into(),
            actual: Some(field_value1.into()),
            expected: None,
        },
        field2: Field {
            name: field_name2.into(),
            actual: Some(field_value2.into()),
            expected: None,
        },
    })
}

/// Convenience method to construct a [`ConstraintViolation`] for a validation
/// performed in the [`State`] context.
///
/// [`ConstraintViolation`]: enum.ConstraintViolation.html
/// [`State`]: struct.State.html
pub fn invalid_state(
    code: impl Into<Cow<'static, str>>,
    params: impl IntoIterator<Item = Field>,
) -> ConstraintViolation {
    ConstraintViolation::State(InvalidState {
        code: code.into(),
        params: Vec::from_iter(params.into_iter()),
    })
}

#[cfg(test)]
mod tests;
