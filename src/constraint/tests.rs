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

mod length {
    use super::*;

    proptest! {
        #[test]
        fn validate_exact_length_on_a_vec_of_correct_len(
            target_len in 0usize..1000
        ) {
            let input = vec![1; target_len];
            let original = input.clone();

            let result = input.validate("text_field", &Length::Exact(target_len)).result();

            prop_assert_eq!(result.unwrap().unwrap(), original);
        }

        #[test]
        fn validate_exact_length_on_a_vec_of_different_len(
            (target_len, input_len) in (0i32..=i32::max_value()).prop_flat_map(|t_len|
                (Just(t_len as usize), (0usize..1000).prop_filter("input len must be different than target length",
                    move |i_len| *i_len != t_len as usize
                ))
            ),
        ) {
            let input = vec![1; input_len];

            let result = input.validate("text_field", &Length::Exact(target_len)).result();

            assert_eq!(
                result,
                Err(ValidationError {
                    message: None,
                    violations: vec![ConstraintViolation::Field(InvalidValue {
                        code: "invalid-length-exact".into(),
                        field: Field {
                            name: "text_field".into(),
                            actual: Some(Value::Integer(input_len as i32)),
                            expected: Some(Value::Integer(target_len as i32)),
                        }
                    })]
                })
            )
        }

        #[test]
        fn validate_max_length_on_a_vec_of_valid_len(
            (max_len, input_len) in (0usize..=1000).prop_flat_map(|t_len|
                (Just(t_len), 0..=t_len)
            ),
        ) {
            let input = vec![1; input_len];
            let original = input.clone();

            let result = input.validate("text_field", &Length::Max(max_len)).result();

            prop_assert_eq!(result.unwrap().unwrap(), original);
        }

        #[test]
        fn validate_max_length_on_a_vec_of_invalid_len(
            (max_len, input_len) in (0usize..=1000).prop_flat_map(|t_len|
                (Just(t_len), t_len + 1..=t_len + 100)
            ),
        ) {
            let input = vec![1; input_len];

            let result = input.validate("text_field", &Length::Max(max_len)).result();

            assert_eq!(
                result,
                Err(ValidationError {
                    message: None,
                    violations: vec![ConstraintViolation::Field(InvalidValue {
                        code: "invalid-length-max".into(),
                        field: Field {
                            name: "text_field".into(),
                            actual: Some(Value::Integer(input_len as i32)),
                            expected: Some(Value::Integer(max_len as i32)),
                        }
                    })]
                })
            )
        }

        #[test]
        fn validate_min_length_on_a_vec_of_valid_len(
            (min_len, input_len) in (0usize..=1000).prop_flat_map(|t_len|
                (Just(t_len), t_len..=t_len + 100)
            ),
        ) {
            let input = vec![1; input_len];
            let original = input.clone();

            let result = input.validate("text_field", &Length::Min(min_len)).result();

            prop_assert_eq!(result.unwrap().unwrap(), original);
        }

        #[test]
        fn validate_min_length_on_a_vec_of_invalid_len(
            (min_len, input_len) in (1usize..=1000).prop_flat_map(|t_len|
                (Just(t_len), 0..t_len)
            ),
        ) {
            let input = vec![1; input_len];

            let result = input.validate("text_field", &Length::Min(min_len)).result();

            assert_eq!(
                result,
                Err(ValidationError {
                    message: None,
                    violations: vec![ConstraintViolation::Field(InvalidValue {
                        code: "invalid-length-min".into(),
                        field: Field {
                            name: "text_field".into(),
                            actual: Some(Value::Integer(input_len as i32)),
                            expected: Some(Value::Integer(min_len as i32)),
                        }
                    })]
                })
            )
        }

        #[test]
        fn validate_minmax_length_on_a_vec_of_valid_len(
            (min_len, max_len, input_len) in (0usize..=100).prop_flat_map(|min|
                (min..=min + 1000).prop_flat_map(move |max|
                    (Just(min), Just(max), min..=max)
                )
            ),
        ) {
            let input = vec![1; input_len];
            let original = input.clone();

            let result = input.validate("text_field", &Length::MinMax(min_len, max_len)).result();

            prop_assert_eq!(result.unwrap().unwrap(), original);
        }

        #[test]
        fn validate_minmax_length_on_a_too_small_vec(
            (min_len, max_len, input_len) in (1usize..=100).prop_flat_map(|min|
                (min..=min + 1000).prop_flat_map(move |max|
                    (Just(min), Just(max), 0..min)
                )
            ),
        ) {
            let input = vec![1; input_len];

            let result = input.validate("text_field", &Length::MinMax(min_len, max_len)).result();

            assert_eq!(
                result,
                Err(ValidationError {
                    message: None,
                    violations: vec![ConstraintViolation::Field(InvalidValue {
                        code: "invalid-length-min".into(),
                        field: Field {
                            name: "text_field".into(),
                            actual: Some(Value::Integer(input_len as i32)),
                            expected: Some(Value::Integer(min_len as i32)),
                        }
                    })]
                })
            )
        }

        #[test]
        fn validate_minmax_length_on_a_too_big_vec(
            (min_len, max_len, input_len) in (1usize..=100).prop_flat_map(|min|
                (min..=min + 1000).prop_flat_map(move |max|
                    (Just(min), Just(max), max + 1..max + 100)
                )
            ),
        ) {
            let input = vec![1; input_len];

            let result = input.validate("text_field", &Length::MinMax(min_len, max_len)).result();

            assert_eq!(
                result,
                Err(ValidationError {
                    message: None,
                    violations: vec![ConstraintViolation::Field(InvalidValue {
                        code: "invalid-length-max".into(),
                        field: Field {
                            name: "text_field".into(),
                            actual: Some(Value::Integer(input_len as i32)),
                            expected: Some(Value::Integer(max_len as i32)),
                        }
                    })]
                })
            )
        }
    }
}

mod char_count {
    use super::*;

    #[test]
    fn validate_exact_char_count_on_a_compliant_string() {
        let text = "I ❤ you";
        assert_eq!(text.len(), 9);
        let original = text.clone();

        let result = text.validate("message", &CharCount::Exact(7)).result();

        assert_eq!(result.unwrap().unwrap(), original);
    }

    #[test]
    fn validate_exact_char_count_on_a_to_short_string() {
        let text = "I ❤ u";
        assert_eq!(text.len(), 7);

        let result = text.validate("message", &CharCount::Exact(7)).result();

        assert_eq!(
            result,
            Err(ValidationError {
                message: None,
                violations: vec![ConstraintViolation::Field(InvalidValue {
                    code: "invalid-char-count-exact".into(),
                    field: Field {
                        name: "message".into(),
                        actual: Some(Value::Integer(5)),
                        expected: Some(Value::Integer(7)),
                    }
                })]
            })
        )
    }

    #[test]
    fn validate_exact_char_count_on_a_to_long_string() {
        let text = "I ❤ you!";
        assert_eq!(text.len(), 10);

        let result = text.validate("message", &CharCount::Exact(7)).result();

        assert_eq!(
            result,
            Err(ValidationError {
                message: None,
                violations: vec![ConstraintViolation::Field(InvalidValue {
                    code: "invalid-char-count-exact".into(),
                    field: Field {
                        name: "message".into(),
                        actual: Some(Value::Integer(8)),
                        expected: Some(Value::Integer(7)),
                    }
                })]
            })
        )
    }

    #[test]
    fn validate_max_char_count_on_a_compliant_string() {
        let text = "I ❤ you";
        assert_eq!(text.len(), 9);
        let original = text.clone();

        let result = text.validate("message", &CharCount::Max(7)).result();

        assert_eq!(result.unwrap().unwrap(), original);
    }

    #[test]
    fn validate_max_char_count_on_a_to_long_string() {
        let text = "I ❤ you!";
        assert_eq!(text.len(), 10);

        let result = text.validate("message", &CharCount::Max(7)).result();

        assert_eq!(
            result,
            Err(ValidationError {
                message: None,
                violations: vec![ConstraintViolation::Field(InvalidValue {
                    code: "invalid-char-count-max".into(),
                    field: Field {
                        name: "message".into(),
                        actual: Some(Value::Integer(8)),
                        expected: Some(Value::Integer(7)),
                    }
                })]
            })
        )
    }

    #[test]
    fn validate_min_char_count_on_a_compliant_string() {
        let text = "I ❤ you!";
        assert_eq!(text.len(), 10);
        let original = text.clone();

        let result = text.validate("message", &CharCount::Min(8)).result();

        assert_eq!(result.unwrap().unwrap(), original);
    }

    #[test]
    fn validate_min_char_count_on_a_to_short_string() {
        let text = "I ❤ you";
        assert_eq!(text.len(), 9);

        let result = text.validate("message", &CharCount::Min(8)).result();

        assert_eq!(
            result,
            Err(ValidationError {
                message: None,
                violations: vec![ConstraintViolation::Field(InvalidValue {
                    code: "invalid-char-count-min".into(),
                    field: Field {
                        name: "message".into(),
                        actual: Some(Value::Integer(7)),
                        expected: Some(Value::Integer(8)),
                    }
                })]
            })
        )
    }

    #[test]
    fn validate_minmax_char_count_on_a_compliant_string() {
        let text = "I ❤ you";
        assert_eq!(text.len(), 9);
        let original = text.clone();

        let result = text.validate("message", &CharCount::MinMax(6, 7)).result();

        assert_eq!(result.unwrap().unwrap(), original);
    }

    #[test]
    fn validate_minmax_char_count_on_a_to_long_string() {
        let text = "I ❤ you!";
        assert_eq!(text.len(), 10);

        let result = text.validate("message", &CharCount::MinMax(6, 7)).result();

        assert_eq!(
            result,
            Err(ValidationError {
                message: None,
                violations: vec![ConstraintViolation::Field(InvalidValue {
                    code: "invalid-char-count-max".into(),
                    field: Field {
                        name: "message".into(),
                        actual: Some(Value::Integer(8)),
                        expected: Some(Value::Integer(7)),
                    }
                })]
            })
        )
    }

    #[test]
    fn validate_minmax_char_count_on_a_to_short_string() {
        let text = "I ❤ u";
        assert_eq!(text.len(), 7);

        let result = text.validate("message", &CharCount::MinMax(6, 7)).result();

        assert_eq!(
            result,
            Err(ValidationError {
                message: None,
                violations: vec![ConstraintViolation::Field(InvalidValue {
                    code: "invalid-char-count-min".into(),
                    field: Field {
                        name: "message".into(),
                        actual: Some(Value::Integer(5)),
                        expected: Some(Value::Integer(6)),
                    }
                })]
            })
        )
    }
}

mod bound {
    use super::*;

    proptest! {
        #[test]
        fn validate_bound_exact_on_a_compliant_float_value(
            exact_bound in any::<f32>()
        ) {
            let float_value = exact_bound;

            let result = float_value.validate("float_value", &Bound::Exact(exact_bound)).result();

            prop_assert_eq!(result.unwrap().unwrap(), float_value);
        }

        #[test]
        fn validate_bound_exact_on_a_float_with_different_value(
            exact_bound in any::<f32>(),
            difference in any::<bool>(),
        ) {
            let float_value = if difference {
                exact_bound * 0.999 - 0.001
            } else {
                exact_bound * 1.001 + 0.001
            };

            let result = float_value.validate("float_value", &Bound::Exact(exact_bound)).result();

            prop_assert_eq!(result, Err(ValidationError {
                message: None,
                violations: vec![ConstraintViolation::Field(InvalidValue {
                    code: "invalid-bound-exact".into(),
                    field: Field {
                        name: "float_value".into(),
                        actual: Some(Value::Float(float_value)),
                        expected: Some(Value::Float(exact_bound)),
                    }
                })]
            }));
        }

        #[test]
        fn validate_bound_closed_range_on_a_long_value_that_is_within_bounds(
            (lower, upper, long_value) in any::<i64>()
                .prop_flat_map(|min| (Just(min), min..=i64::max_value()) )
                .prop_flat_map(|(min, max)| (Just(min), Just(max), min..=max) )
        ) {
            let result = long_value.validate("long_value", &Bound::ClosedRange(lower, upper)).result();

            prop_assert_eq!(result.unwrap().unwrap(), long_value);
        }

        #[test]
        fn validate_bound_closed_range_on_a_long_value_that_is_less_than_the_lower_bound(
            (lower, upper, long_value) in (i64::min_value() + 1..=i64::max_value())
                .prop_flat_map(|min| (Just(min), min..=i64::max_value()) )
                .prop_flat_map(|(min, max)| (Just(min), Just(max), i64::min_value()..min) )
        ) {
            let result = long_value.validate("long_value", &Bound::ClosedRange(lower, upper)).result();

            prop_assert_eq!(result, Err(ValidationError {
                message: None,
                violations: vec![ConstraintViolation::Field(InvalidValue {
                    code: "invalid-bound-closed-min".into(),
                    field: Field {
                        name: "long_value".into(),
                        actual: Some(Value::Long(long_value)),
                        expected: Some(Value::Long(lower)),
                    }
                })]
            }))
        }

        #[test]
        fn validate_bound_closed_range_on_a_long_value_that_is_greater_than_the_upper_bound(
            (lower, upper, long_value) in (i64::min_value()..i64::max_value())
                .prop_flat_map(|max| (i64::min_value()..=max, Just(max)) )
                .prop_flat_map(|(min, max)| (Just(min), Just(max), max + 1..i64::max_value()) )
        ) {
            let result = long_value.validate("long_value", &Bound::ClosedRange(lower, upper)).result();

            prop_assert_eq!(result, Err(ValidationError {
                message: None,
                violations: vec![ConstraintViolation::Field(InvalidValue {
                    code: "invalid-bound-closed-max".into(),
                    field: Field {
                        name: "long_value".into(),
                        actual: Some(Value::Long(long_value)),
                        expected: Some(Value::Long(upper)),
                    }
                })]
            }))
        }

        #[test]
        fn validate_bound_closedopen_range_on_a_long_value_that_is_within_bounds(
            (lower, upper, long_value) in any::<i64>()
                .prop_flat_map(|min| (Just(min), min..=i64::max_value()) )
                .prop_flat_map(|(min, max)| (Just(min), Just(max), min..max) )
        ) {
            let result = long_value.validate("long_value", &Bound::ClosedOpenRange(lower, upper)).result();

            prop_assert_eq!(result.unwrap().unwrap(), long_value);
        }

        #[test]
        fn validate_bound_closedopen_range_on_a_long_value_that_is_less_than_the_lower_bound(
            (lower, upper, long_value) in (i64::min_value() + 1..=i64::max_value())
                .prop_flat_map(|min| (Just(min), min..=i64::max_value()) )
                .prop_flat_map(|(min, max)| (Just(min), Just(max), i64::min_value()..min) )
        ) {
            let result = long_value.validate("long_value", &Bound::ClosedOpenRange(lower, upper)).result();

            prop_assert_eq!(result, Err(ValidationError {
                message: None,
                violations: vec![ConstraintViolation::Field(InvalidValue {
                    code: "invalid-bound-closed-min".into(),
                    field: Field {
                        name: "long_value".into(),
                        actual: Some(Value::Long(long_value)),
                        expected: Some(Value::Long(lower)),
                    }
                })]
            }))
        }

        #[test]
        fn validate_bound_closedopen_range_on_a_long_value_that_is_greater_than_or_equal_the_upper_bound(
            (lower, upper, long_value) in (i64::min_value()..=i64::max_value())
                .prop_flat_map(|max| (i64::min_value()..=max, Just(max)) )
                .prop_flat_map(|(min, max)| (Just(min), Just(max), max..i64::max_value()) )
        ) {
            let result = long_value.validate("long_value", &Bound::ClosedOpenRange(lower, upper)).result();

            prop_assert_eq!(result, Err(ValidationError {
                message: None,
                violations: vec![ConstraintViolation::Field(InvalidValue {
                    code: "invalid-bound-open-max".into(),
                    field: Field {
                        name: "long_value".into(),
                        actual: Some(Value::Long(long_value)),
                        expected: Some(Value::Long(upper)),
                    }
                })]
            }))
        }

        #[test]
        fn validate_bound_openclosed_range_on_a_long_value_that_is_within_bounds(
            (lower, upper, long_value) in any::<i64>()
                .prop_flat_map(|min| (Just(min), min..=i64::max_value()) )
                .prop_flat_map(|(min, max)| (Just(min), Just(max), min + 1..=max) )
        ) {
            let result = long_value.validate("long_value", &Bound::OpenClosedRange(lower, upper)).result();

            prop_assert_eq!(result.unwrap().unwrap(), long_value);
        }

        #[test]
        fn validate_bound_openclosed_range_on_a_long_value_that_is_less_than_or_equal_the_lower_bound(
            (lower, upper, long_value) in (i64::min_value()..=i64::max_value())
                .prop_flat_map(|min| (Just(min), min..=i64::max_value()) )
                .prop_flat_map(|(min, max)| (Just(min), Just(max), i64::min_value()..=min) )
        ) {
            let result = long_value.validate("long_value", &Bound::OpenClosedRange(lower, upper)).result();

            prop_assert_eq!(result, Err(ValidationError {
                message: None,
                violations: vec![ConstraintViolation::Field(InvalidValue {
                    code: "invalid-bound-open-min".into(),
                    field: Field {
                        name: "long_value".into(),
                        actual: Some(Value::Long(long_value)),
                        expected: Some(Value::Long(lower)),
                    }
                })]
            }))
        }

        #[test]
        fn validate_bound_openclosed_range_on_a_long_value_that_is_greater_than_the_upper_bound(
            (lower, upper, long_value) in (i64::min_value()..i64::max_value())
                .prop_flat_map(|max| (i64::min_value()..=max, Just(max)) )
                .prop_flat_map(|(min, max)| (Just(min), Just(max), max + 1..i64::max_value()) )
        ) {
            let result = long_value.validate("long_value", &Bound::OpenClosedRange(lower, upper)).result();

            prop_assert_eq!(result, Err(ValidationError {
                message: None,
                violations: vec![ConstraintViolation::Field(InvalidValue {
                    code: "invalid-bound-closed-max".into(),
                    field: Field {
                        name: "long_value".into(),
                        actual: Some(Value::Long(long_value)),
                        expected: Some(Value::Long(upper)),
                    }
                })]
            }))
        }

        #[test]
        fn validate_bound_open_range_on_a_long_value_that_is_within_bounds(
            (lower, upper, long_value) in any::<i64>()
                .prop_flat_map(|min| (Just(min), min..=i64::max_value()) )
                .prop_flat_map(|(min, max)| (Just(min), Just(max), min + 1..max) )
        ) {
            let result = long_value.validate("long_value", &Bound::OpenRange(lower, upper)).result();

            prop_assert_eq!(result.unwrap().unwrap(), long_value);
        }

        #[test]
        fn validate_bound_open_range_on_a_long_value_that_is_less_than_or_equal_the_lower_bound(
            (lower, upper, long_value) in (i64::min_value()..=i64::max_value())
                .prop_flat_map(|min| (Just(min), min..=i64::max_value()) )
                .prop_flat_map(|(min, max)| (Just(min), Just(max), i64::min_value()..=min) )
        ) {
            let result = long_value.validate("long_value", &Bound::OpenRange(lower, upper)).result();

            prop_assert_eq!(result, Err(ValidationError {
                message: None,
                violations: vec![ConstraintViolation::Field(InvalidValue {
                    code: "invalid-bound-open-min".into(),
                    field: Field {
                        name: "long_value".into(),
                        actual: Some(Value::Long(long_value)),
                        expected: Some(Value::Long(lower)),
                    }
                })]
            }))
        }

        #[test]
        fn validate_bound_open_range_on_a_long_value_that_is_greater_than_or_equal_the_upper_bound(
            (lower, upper, long_value) in (i64::min_value()..=i64::max_value())
                .prop_flat_map(|max| (i64::min_value()..=max, Just(max)) )
                .prop_flat_map(|(min, max)| (Just(min), Just(max), max..i64::max_value()) )
        ) {
            let result = long_value.validate("long_value", &Bound::OpenRange(lower, upper)).result();

            prop_assert_eq!(result, Err(ValidationError {
                message: None,
                violations: vec![ConstraintViolation::Field(InvalidValue {
                    code: "invalid-bound-open-max".into(),
                    field: Field {
                        name: "long_value".into(),
                        actual: Some(Value::Long(long_value)),
                        expected: Some(Value::Long(upper)),
                    }
                })]
            }))
        }
    }
}

#[cfg(feature = "bigdecimal")]
mod digits {
    use super::*;

    #[test]
    fn validate_digits_of_bigdecimal_that_is_compliant() {
        use bigdecimal::BigDecimal;
        use std::str::FromStr;

        let account_balance = BigDecimal::from_str("12345678.99").unwrap();

        let result = account_balance
            .validate(
                "account_balance",
                &Digits {
                    integer: 8,
                    fraction: 2,
                },
            )
            .result();

        assert_eq!(
            result.unwrap().unwrap(),
            BigDecimal::from_str("12345678.99").unwrap()
        );
    }

    #[test]
    fn validate_digits_of_bigdecimal_with_too_many_integer_digits() {
        use bigdecimal::BigDecimal;
        use std::str::FromStr;

        let account_balance = BigDecimal::from_str("123456780.99").unwrap();

        let result = account_balance
            .validate(
                "account_balance",
                &Digits {
                    integer: 8,
                    fraction: 2,
                },
            )
            .result();

        assert_eq!(
            result,
            Err(ValidationError {
                message: None,
                violations: vec![ConstraintViolation::Field(InvalidValue {
                    code: "invalid-digits-integer".into(),
                    field: Field {
                        name: "account_balance".into(),
                        actual: Some(Value::Long(9)),
                        expected: Some(Value::Long(8)),
                    }
                })]
            })
        );
    }

    #[test]
    fn validate_digits_of_bigdecimal_with_too_many_fraction_digits() {
        use bigdecimal::BigDecimal;
        use std::str::FromStr;

        let account_balance = BigDecimal::from_str("12345678.995").unwrap();

        let result = account_balance
            .validate(
                "account_balance",
                &Digits {
                    integer: 8,
                    fraction: 2,
                },
            )
            .result();

        assert_eq!(
            result,
            Err(ValidationError {
                message: None,
                violations: vec![ConstraintViolation::Field(InvalidValue {
                    code: "invalid-digits-fraction".into(),
                    field: Field {
                        name: "account_balance".into(),
                        actual: Some(Value::Long(3)),
                        expected: Some(Value::Long(2)),
                    }
                })]
            })
        );
    }

    #[test]
    fn validate_digits_of_bigdecimal_with_too_many_integer_and_fraction_digits() {
        use bigdecimal::BigDecimal;
        use std::str::FromStr;

        let account_balance = BigDecimal::from_str("123456780.995").unwrap();

        let result = account_balance
            .validate(
                "account_balance",
                &Digits {
                    integer: 8,
                    fraction: 2,
                },
            )
            .result();

        assert_eq!(
            result,
            Err(ValidationError {
                message: None,
                violations: vec![
                    ConstraintViolation::Field(InvalidValue {
                        code: "invalid-digits-integer".into(),
                        field: Field {
                            name: "account_balance".into(),
                            actual: Some(Value::Long(9)),
                            expected: Some(Value::Long(8)),
                        }
                    }),
                    ConstraintViolation::Field(InvalidValue {
                        code: "invalid-digits-fraction".into(),
                        field: Field {
                            name: "account_balance".into(),
                            actual: Some(Value::Long(3)),
                            expected: Some(Value::Long(2)),
                        }
                    })
                ]
            })
        );
    }
}
