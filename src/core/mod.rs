//! The core API of the `valid` crate.

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
use std::ops::Deref;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Validated<T>(T);

impl<T> Validated<T> {
    pub fn unwrap(self) -> T {
        self.0
    }
}

impl<T> Deref for Validated<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub trait Validate<C>: Sized {
    fn validate(self, name: impl Into<Cow<'static, str>>, constraint: &C) -> Validation<Self>;
}

#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
    Integer(i32),
    Long(i64),
    Float(f32),
    Double(f64),
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

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::Long(value)
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
pub enum Validation<T> {
    Success(T),
    Failure(Vec<ConstraintViolation>),
}

impl<T> From<ConstraintViolation> for Validation<T> {
    fn from(constraint_violation: ConstraintViolation) -> Self {
        Validation::Failure(vec![constraint_violation])
    }
}

impl<T> From<Vec<ConstraintViolation>> for Validation<T> {
    fn from(constraint_violations: Vec<ConstraintViolation>) -> Self {
        Validation::Failure(constraint_violations)
    }
}

impl<T> Validation<T> {
    pub fn result(
        self,
        message: impl Into<Option<Cow<'static, str>>>,
    ) -> Result<Validated<T>, ValidationError> {
        match self {
            Validation::Success(entity) => Ok(Validated(entity)),
            Validation::Failure(violations) => Err(ValidationError {
                message: message.into(),
                violations,
            }),
        }
    }

    pub fn and<C, U>(
        self,
        field_name: impl Into<Cow<'static, str>>,
        constraint: &C,
        entity: U,
    ) -> Validation<()>
    where
        U: Validate<C>,
    {
        let other = entity.validate(field_name, constraint);
        match (self, other) {
            (Validation::Success(_), Validation::Success(_)) => Validation::Success(()),
            (Validation::Failure(violations), Validation::Success(_)) => {
                Validation::Failure(violations)
            }
            (Validation::Success(_), Validation::Failure(violations)) => {
                Validation::Failure(violations)
            }
            (Validation::Failure(mut violations), Validation::Failure(violations2)) => {
                violations.extend(violations2);
                Validation::Failure(violations)
            }
        }
    }

    pub fn and_then<C, U>(
        self,
        field_name: impl Into<Cow<'static, str>>,
        constraint: &C,
        entity: U,
    ) -> Validation<U>
    where
        U: Validate<C>,
    {
        match self {
            Validation::Success(_) => entity.validate(field_name, constraint),
            Validation::Failure(error) => Validation::Failure(error),
        }
    }

    pub fn is_success(&self) -> bool {
        match self {
            Validation::Success(_) => true,
            Validation::Failure(_) => false,
        }
    }

    pub fn is_failure(&self) -> bool {
        match self {
            Validation::Success(_) => false,
            Validation::Failure(_) => true,
        }
    }
}

pub fn invalid_value(
    code: impl Into<Cow<'static, str>>,
    field_name: impl Into<Cow<'static, str>>,
    actual_value: impl Into<Value>,
    expected_value: impl Into<Value>,
) -> ConstraintViolation {
    ConstraintViolation::Field(InvalidValue {
        code: code.into(),
        field: Field {
            name: field_name.into(),
            actual: Some(actual_value.into()),
            expected: Some(expected_value.into()),
        },
    })
}

pub fn invalid_optional_value(
    code: impl Into<Cow<'static, str>>,
    field_name: impl Into<Cow<'static, str>>,
    actual: Option<Value>,
    expected: Option<Value>,
) -> ConstraintViolation {
    ConstraintViolation::Field(InvalidValue {
        code: code.into(),
        field: Field {
            name: field_name.into(),
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

pub fn fail(message: Option<Cow<'static, str>>, violation: ConstraintViolation) -> ValidationError {
    ValidationError {
        message,
        violations: vec![violation],
    }
}

#[cfg(test)]
mod tests;
