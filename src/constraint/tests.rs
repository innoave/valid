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
                    code: "invalid-assert-true".into(),
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
                    code: "invalid-assert-false".into(),
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

mod not_empty {
    use super::*;
    use std::collections::{HashMap, HashSet};

    #[test]
    fn validate_not_empty_on_empty_string() {
        let input = String::new();

        let result = input.validate("text_field", &NotEmpty).result();

        assert_eq!(
            result,
            Err(ValidationError {
                message: None,
                violations: vec![ConstraintViolation::Field(InvalidValue {
                    code: "invalid-not-empty".into(),
                    field: Field {
                        name: "text_field".into(),
                        actual: None,
                        expected: None,
                    }
                })]
            })
        )
    }

    proptest! {
        #[test]
        fn validate_not_empty_on_non_empty_string(
            input in ".{1,100}"
        ) {
            let original = input.clone();

            let result = input.validate("text_field", &NotEmpty).result();

            prop_assert_eq!(result.unwrap().unwrap(), original);
        }
    }

    #[test]
    fn validate_not_empty_on_empty_vec() {
        let input = Vec::<u16>::new();

        let result = input.validate("collection", &NotEmpty).result();

        assert_eq!(
            result,
            Err(ValidationError {
                message: None,
                violations: vec![ConstraintViolation::Field(InvalidValue {
                    code: "invalid-not-empty".into(),
                    field: Field {
                        name: "collection".into(),
                        actual: None,
                        expected: None,
                    }
                })]
            })
        )
    }

    proptest! {
        #[test]
        fn validate_not_empty_on_non_empty_vec(
           input in prop::collection::vec(any::<u16>(), 1..100)
        ) {
            let original = input.clone();

            let result = input.validate("collection", &NotEmpty).result();

            prop_assert_eq!(result.unwrap().unwrap(), original);
        }
    }

    #[test]
    fn validate_not_empty_on_empty_hash_set() {
        let input = HashSet::<u16>::new();

        let result = input.validate("collection", &NotEmpty).result();

        assert_eq!(
            result,
            Err(ValidationError {
                message: None,
                violations: vec![ConstraintViolation::Field(InvalidValue {
                    code: "invalid-not-empty".into(),
                    field: Field {
                        name: "collection".into(),
                        actual: None,
                        expected: None,
                    }
                })]
            })
        )
    }

    proptest! {
        #[test]
        fn validate_not_empty_on_non_empty_hash_set(
           input in prop::collection::hash_set(any::<u16>(), 1..100)
        ) {
            let original = input.clone();

            let result = input.validate("collection", &NotEmpty).result();

            prop_assert_eq!(result.unwrap().unwrap(), original);
        }
    }

    #[test]
    fn validate_not_empty_on_empty_hash_map() {
        let input = HashMap::<u16, i64>::new();

        let result = input.validate("collection", &NotEmpty).result();

        assert_eq!(
            result,
            Err(ValidationError {
                message: None,
                violations: vec![ConstraintViolation::Field(InvalidValue {
                    code: "invalid-not-empty".into(),
                    field: Field {
                        name: "collection".into(),
                        actual: None,
                        expected: None,
                    }
                })]
            })
        )
    }

    proptest! {
        #[test]
        fn validate_not_empty_on_non_empty_hash_map(
           input in prop::collection::hash_map(any::<u16>(), any::<i64>(), 1..100)
        ) {
            let original = input.clone();

            let result = input.validate("collection", &NotEmpty).result();

            prop_assert_eq!(result.unwrap().unwrap(), original);
        }
    }

    #[test]
    fn validate_not_empty_on_option_none() {
        let input: Option<String> = None;

        let result = input.validate("optional_text", &NotEmpty).result();

        assert_eq!(
            result,
            Err(ValidationError {
                message: None,
                violations: vec![ConstraintViolation::Field(InvalidValue {
                    code: "invalid-not-empty".into(),
                    field: Field {
                        name: "optional_text".into(),
                        actual: None,
                        expected: None,
                    }
                })]
            })
        )
    }

    #[test]
    fn validate_not_empty_on_option_of_some_empty_string() {
        let input: Option<String> = Some(String::new());

        let result = input.validate("optional_text", &NotEmpty).result();

        assert_eq!(
            result,
            Err(ValidationError {
                message: None,
                violations: vec![ConstraintViolation::Field(InvalidValue {
                    code: "invalid-not-empty".into(),
                    field: Field {
                        name: "optional_text".into(),
                        actual: None,
                        expected: None,
                    }
                })]
            })
        )
    }

    proptest! {
        #[test]
        fn validate_not_empty_on_option_of_some_non_empty_string(
            input in ".{1,100}"
        ) {
            let original = Some(input.clone());

            let result = Some(input).validate("text_field", &NotEmpty).result();

            prop_assert_eq!(result.unwrap().unwrap(), original);
        }
    }
}
