#[cfg(feature = "bigdecimal")]
use bigdecimal::BigDecimal;
#[cfg(feature = "chrono")]
use chrono::{DateTime, NaiveDate, Utc};
#[cfg(feature = "serde1")]
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

pub fn fail(message: Option<Cow<'static, str>>, violation: ConstraintViolation) -> ValidationError {
    ValidationError {
        message,
        violations: vec![violation],
    }
}

pub fn invalid_value(
    code: Cow<'static, str>,
    field_name: Cow<'static, str>,
    value: impl Into<Value>,
) -> ConstraintViolation {
    ConstraintViolation::Field(InvalidValue {
        code,
        field: Field {
            name: field_name,
            value: value.into(),
        },
    })
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Validated<T>(T);

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
    pub value: Value,
}

#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct OptionalField {
    pub name: Cow<'static, str>,
    pub value: Option<Value>,
}

#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct InvalidValue {
    pub code: Cow<'static, str>,
    pub field: Field,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InvalidRelation {
    pub code: Cow<'static, str>,
    pub field1: OptionalField,
    pub field2: OptionalField,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InvalidState {
    pub code: Cow<'static, str>,
    pub params: Vec<Field>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConstraintViolation {
    Field(InvalidValue),
    Relation(InvalidRelation),
    State(InvalidState),
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

impl ValidationError {
    fn merge(mut self, other: ValidationError) -> Self {
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

impl<T> Validated<T> {
    pub fn unwrap(self) -> T {
        self.0
    }
}

pub trait Validate<C>: Sized {
    fn validate(self, field_name: impl Into<Cow<'static, str>>, constraint: &C)
        -> Validation<Self>;
}

#[cfg(test)]
mod tests;
