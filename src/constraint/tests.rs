use super::*;
use crate::{ConstraintViolation, Field, InvalidValue, ValidationError};
use proptest::prelude::*;

mod assert_true {
    use super::*;

    #[test]
    fn validate_assert_true_on_value_true() {
        let result = true.validate("agreed", &AssertTrue).result();

        assert_eq!(result.unwrap().unwrap(), true);
    }

    #[test]
    fn validate_assert_true_on_value_false() {
        let result = false.validate("agreed", &AssertTrue).result();

        assert_eq!(
            result,
            Err(ValidationError {
                message: None,
                violations: vec![ConstraintViolation::Field(InvalidValue {
                    code: "invalid.assert.true".into(),
                    field: Field {
                        name: "agreed".into(),
                        actual: Some(Value::Boolean(false)),
                        expected: Some(Value::Boolean(true)),
                    }
                })]
            })
        );
    }
}

mod assert_false {
    use super::*;

    #[test]
    fn validate_assert_false_on_value_false() {
        let result = false.validate("unchecked", &AssertFalse).result();

        assert_eq!(result.unwrap().unwrap(), false);
    }

    #[test]
    fn validate_assert_false_on_value_true() {
        let result = true.validate("unchecked", &AssertFalse).result();

        assert_eq!(
            result,
            Err(ValidationError {
                message: None,
                violations: vec![ConstraintViolation::Field(InvalidValue {
                    code: "invalid.assert.false".into(),
                    field: Field {
                        name: "unchecked".into(),
                        actual: Some(Value::Boolean(true)),
                        expected: Some(Value::Boolean(false)),
                    }
                })]
            })
        );
    }
}
