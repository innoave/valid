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
use std::fmt::{Display, Write};
use std::iter::FromIterator;
use std::marker::PhantomData;
use std::ops::Deref;

/// A wrapper type to express that the value of type `T` has been validated
///
/// The idea is that an instance of `Validated<C, T>` can only be obtained by
/// validating a value of type `T` using the constraint `C`. There is no way to
/// construct an instance of `Validated` directly.
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
/// ```rust
/// fn send_email(to: String, message: String) {
///     unimplemented!()
/// }
/// ```
///
/// The problem with this approach is, that we can never be sure that the input
/// string for the 'to' argument is a valid email address.
///
/// Lets rewrite the same function using `Validated<Email, String>`.
///
/// ```rust,ignore //TODO remove ignore when Email constraint is implemented
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
/// ```rust,ignore //TODO remove ignore when Email constraint is implemented
/// use valid::{Validated, Validate};
///
/// let to_addr = "jane.doe@email.net".to_string().validate("email", Email).result(None)
///         .expect("valid email address");
///
/// send_email(to_addr, "some message".into());
///
/// fn send_email(to: Validated<String>, message: String) {
///     unimplemented!()
/// }
/// ```
///
/// Now we can be sure that the variable `to_addr` contains a valid email
/// address.
///
/// To further make use of meaningful new types we might define a custom new
/// type for email addresses, that can only be constructed from a validated
/// value like so:
///
/// ```rust,ignore //TODO remove ignore when Email constraint is implemented
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
///
/// fn send_email(to: EmailAddress, message: String) {
///     unimplemented!()
/// }
/// ```
///
/// Due to the type `EmailAddress` is defined in another module it can only be
/// constructed from a `Validated<Email, String>`.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Validated<C, T>(PhantomData<C>, T);

impl<C, T> Validated<C, T> {
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

pub trait Validate<C, S>
where
    S: Context,
    Self: Sized,
{
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

#[derive(Debug, Clone, PartialEq)]
pub enum Validation<C, T> {
    Success(PhantomData<C>, T),
    Failure(Vec<ConstraintViolation>),
}

impl<C, T> Validation<C, T> {
    pub fn success(valid: T) -> Self {
        Validation::Success(PhantomData, valid)
    }

    pub fn failure(constraint_violations: impl IntoIterator<Item = ConstraintViolation>) -> Self {
        Validation::Failure(Vec::from_iter(constraint_violations.into_iter()))
    }

    pub fn result(
        self,
        message: impl Into<Option<Cow<'static, str>>>,
    ) -> Result<Validated<C, T>, ValidationError> {
        match self {
            Validation::Success(_c, entity) => Ok(Validated(_c, entity)),
            Validation::Failure(violations) => Err(ValidationError {
                message: message.into(),
                violations,
            }),
        }
    }

    pub fn and<D, S, U>(self, context: impl Into<S>, constraint: &D, entity: U) -> Validation<D, ()>
    where
        S: Context,
        U: Validate<D, S>,
    {
        let other = entity.validate(context, constraint);
        match (self, other) {
            (Validation::Success(_, _), Validation::Success(_, _)) => Validation::success(()),
            (Validation::Failure(violations), Validation::Success(_, _)) => {
                Validation::failure(violations)
            }
            (Validation::Success(_, _), Validation::Failure(violations)) => {
                Validation::failure(violations)
            }
            (Validation::Failure(mut violations), Validation::Failure(violations2)) => {
                violations.extend(violations2);
                Validation::failure(violations)
            }
        }
    }

    pub fn and_then<D, S, U>(
        self,
        context: impl Into<S>,
        constraint: &D,
        entity: U,
    ) -> Validation<D, U>
    where
        S: Context,
        U: Validate<D, S>,
    {
        match self {
            Validation::Success(_, _) => entity.validate(context, constraint),
            Validation::Failure(error) => Validation::failure(error),
        }
    }
}

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
