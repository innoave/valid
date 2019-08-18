#![doc(html_root_url = "https://docs.rs/valid/0.1.0")]
#![warn(
    bare_trait_objects,
    missing_copy_implementations,
    missing_debug_implementations,
//    missing_docs,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications
)]
#![allow(dead_code)] //TODO remove eventually

#[cfg(feature = "bigdecimal")]
use bigdecimal::BigDecimal;
#[cfg(feature = "chrono")]
use chrono::{DateTime, NaiveDate, Utc};
#[cfg(feature = "serde1")]
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

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
    pub field1: Field,
    pub field2: OptionalField,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InvalidState {
    pub code: Cow<'static, str>,
    pub params: Vec<Field>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConstraintViolation {
    Field(Field),
    Relation(Field, Field),
    State(InvalidState),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ValidationError {
    pub message: Option<Cow<'static, str>>,
    pub violations: Vec<ConstraintViolation>,
}

impl ValidationError {
    pub fn merge(mut self, other: ValidationError) -> Self {
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
    Successful(T),
    Failed(ValidationError),
}

impl<T> Validation<T> {
    pub fn result(self) -> Result<Validated<T>, ValidationError> {
        match self {
            Validation::Successful(entity) => Ok(Validated(entity)),
            Validation::Failed(error) => Err(error),
        }
    }

    pub fn and<U>(self, validate: impl Validate<Entity = U>, entity: &U) -> Validation<()> {
        let validation2 = validate.validate(entity);
        match self {
            Validation::Successful(_) => match validation2 {
                Validation::Successful(_) => Validation::Successful(()),
                Validation::Failed(error2) => Validation::Failed(error2),
            },
            Validation::Failed(error) => match validation2 {
                Validation::Successful(_) => Validation::Successful(()),
                Validation::Failed(error2) => Validation::Failed(error.merge(error2)),
            },
        }
    }

    pub fn and_then<U>(self, validate: impl Validate<Entity = U>, entity: &U) -> Validation<U> {
        match self {
            Validation::Successful(_) => validate.validate(entity),
            Validation::Failed(error) => Validation::Failed(error),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Validated<T>(T);

impl<T> Validated<T> {
    pub fn unwrap(self) -> T {
        self.0
    }
}

pub trait Validate {
    type Entity;

    fn validate(&self, entity: &Self::Entity) -> Validation<Self::Entity>;
}

#[cfg(test)]
mod tests;
