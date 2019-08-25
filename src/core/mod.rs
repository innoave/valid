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
/// The problem with this approach is, that we can never be sure that the input
/// string for the `to` argument is a valid email address.
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
/// use valid::{Validated, Validate};
///
/// fn send_email(to: Validated<Email, String>, message: String) {
///     unimplemented!()
/// }
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
/// use valid::{Validate, Validated};
///
/// fn send_email(to: EmailAddress, message: String) {
///     unimplemented!()
/// }
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

pub trait Context: private::Sealed {}

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
    pub fn unwrap(self) -> Cow<'static, str> {
        self.0
    }
}

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
    pub fn unwrap(self) -> (Cow<'static, str>, Cow<'static, str>) {
        (self.0, self.1)
    }

    pub fn first(&self) -> &str {
        &self.0
    }

    pub fn second(&self) -> &str {
        &self.1
    }
}

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
    pub fn unwrap(self) -> S {
        self.0
    }
}

enum InnerValidation<C, T> {
    Success(PhantomData<C>, T),
    Failure(Vec<ConstraintViolation>),
}

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
    pub fn success(valid: T) -> Self {
        Validation(InnerValidation::Success(PhantomData, valid))
    }

    pub fn failure(constraint_violations: impl IntoIterator<Item = ConstraintViolation>) -> Self {
        Validation(InnerValidation::Failure(Vec::from_iter(
            constraint_violations.into_iter(),
        )))
    }

    pub fn result(self) -> ValidationResult<C, T> {
        match self.0 {
            InnerValidation::Success(_c, entity) => Ok(Validated(_c, entity)),
            InnerValidation::Failure(violations) => Err(ValidationError {
                message: None,
                violations,
            }),
        }
    }

    pub fn with_message(self, message: impl Into<Cow<'static, str>>) -> ValidationResult<C, T> {
        match self.0 {
            InnerValidation::Success(_c, entity) => Ok(Validated(_c, entity)),
            InnerValidation::Failure(violations) => Err(ValidationError {
                message: Some(message.into()),
                violations,
            }),
        }
    }

    pub fn combine<U>(self, value: U) -> Validation<C, (U, T)> {
        match self.0 {
            InnerValidation::Success(_, entity) => Validation::success((value, entity)),
            InnerValidation::Failure(violations) => Validation::failure(violations),
        }
    }

    pub fn map<D, U>(self, convert: impl Fn(T) -> U) -> Validation<D, U> {
        match self.0 {
            InnerValidation::Success(_, entity) => Validation::success(convert(entity)),
            InnerValidation::Failure(violations) => Validation::failure(violations),
        }
    }

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

    pub fn and_then<D, U>(self, next: impl FnOnce(T) -> Validation<D, U>) -> Validation<D, U> {
        match self.0 {
            InnerValidation::Success(_, value1) => next(value1),
            InnerValidation::Failure(error) => Validation::failure(error),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Combined;

impl Context for Combined {}

#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
    Integer(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    Boolean(bool),
    #[cfg(feature = "bigdecimal")]
    Decimal(BigDecimal),
    #[cfg(feature = "chrono")]
    Date(NaiveDate),
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

impl From<u64> for Value {
    fn from(value: u64) -> Self {
        Value::Long(value as i64)
    }
}

impl From<isize> for Value {
    fn from(value: isize) -> Self {
        if value > i32::max_value() as isize {
            Value::Long(value as i64)
        } else {
            Value::Integer(value as i32)
        }
    }
}

impl From<usize> for Value {
    fn from(value: usize) -> Self {
        if value > i32::max_value() as usize {
            Value::Long(value as i64)
        } else {
            Value::Integer(value as i32)
        }
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

#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    pub name: Cow<'static, str>,
    pub actual: Option<Value>,
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

#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct InvalidValue {
    pub code: Cow<'static, str>,
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

#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct InvalidRelation {
    pub code: Cow<'static, str>,
    pub field1: Field,
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

#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct InvalidState {
    pub code: Cow<'static, str>,
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

#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum ConstraintViolation {
    Field(InvalidValue),
    Relation(InvalidRelation),
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

#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct ValidationError {
    pub message: Option<Cow<'static, str>>,
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

pub type ValidationResult<C, T> = Result<Validated<C, T>, ValidationError>;

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
