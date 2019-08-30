use super::*;
use proptest::prelude::*;

mod validation {
    use super::*;
    use crate::constraint::{Bound, NotEmpty};

    #[test]
    fn unfortunately_we_can_construct_an_instance_of_validated_without_doing_any_validation() {
        //TODO find a way to prevent this from compiling and still support the
        //     possibility for custom implementations of the `Validate` trait

        let value: Validated<Bound<i32>, i32> = Validation::success(42).result().unwrap();

        assert_eq!(value.unwrap(), 42);

        let value: Validated<NotEmpty, String> =
            Validation::success("invalid".to_string()).result().unwrap();

        assert_eq!(value.unwrap(), "invalid");
    }
}

mod value {
    use super::*;

    #[cfg(not(any(feature = "bigdecimal", feature = "chrono")))]
    #[test]
    fn exhaustive_match_over_value_variants_for_default_features() {
        fn exhaustive_match(value: Value) -> i32 {
            match value {
                Value::String(_) => 1,
                Value::Integer(_) => 2,
                Value::Long(_) => 3,
                Value::Float(_) => 4,
                Value::Double(_) => 5,
                Value::Boolean(_) => 6,
            }
        }
        assert_eq!(exhaustive_match(Value::Integer(0)), 2);
    }

    #[cfg(all(feature = "bigdecimal", not(feature = "chrono")))]
    #[test]
    fn exhaustive_match_over_value_variants_with_bigdecimal_feature() {
        fn exhaustive_match(value: Value) -> i32 {
            match value {
                Value::String(_) => 1,
                Value::Integer(_) => 2,
                Value::Long(_) => 3,
                Value::Float(_) => 4,
                Value::Double(_) => 5,
                Value::Boolean(_) => 6,
                Value::Decimal(_) => 7,
            }
        }
        assert_eq!(exhaustive_match(Value::Integer(0)), 2);
    }

    #[cfg(all(not(feature = "bigdecimal"), feature = "chrono"))]
    #[test]
    fn exhaustive_match_over_value_variants_with_chrono_feature() {
        fn exhaustive_match(value: Value) -> i32 {
            match value {
                Value::String(_) => 1,
                Value::Integer(_) => 2,
                Value::Long(_) => 3,
                Value::Float(_) => 4,
                Value::Double(_) => 5,
                Value::Boolean(_) => 6,
                Value::Date(_) => 8,
                Value::DateTime(_) => 9,
            }
        }
        assert_eq!(exhaustive_match(Value::Integer(0)), 2);
    }

    #[cfg(all(feature = "bigdecimal", feature = "chrono"))]
    #[test]
    fn exhaustive_match_over_value_variants_with_bigdecimal_and_chrono_features() {
        fn exhaustive_match(value: Value) -> i32 {
            match value {
                Value::String(_) => 1,
                Value::Integer(_) => 2,
                Value::Long(_) => 3,
                Value::Float(_) => 4,
                Value::Double(_) => 5,
                Value::Boolean(_) => 6,
                Value::Decimal(_) => 7,
                Value::Date(_) => 8,
                Value::DateTime(_) => 9,
            }
        }
        assert_eq!(exhaustive_match(Value::Integer(0)), 2);
    }

    proptest! {
        #[test]
        fn can_convert_i8_values_into_integer_value(
            param in any::<i8>()
        ) {
            let value = Value::from(param);

            prop_assert_eq!(value, Value::Integer(i32::from(param)));
        }

        #[test]
        fn can_convert_i16_values_into_integer_value(
            param in any::<i16>()
        ) {
            let value = Value::from(param);

            prop_assert_eq!(value, Value::Integer(i32::from(param)));
        }

        #[test]
        fn can_convert_i32_values_into_integer_value(
            param in any::<i16>()
        ) {
            let value = Value::from(param);

            prop_assert_eq!(value, Value::Integer(i32::from(param)));
        }

        #[test]
        fn can_convert_i64_values_into_long_value(
            param in any::<i64>()
        ) {
            let value = Value::from(param);

            prop_assert_eq!(value, Value::Long(param));
        }

        #[test]
        fn can_convert_u8_values_into_integer_value(
            param in any::<u8>()
        ) {
            let value = Value::from(param);

            prop_assert_eq!(value, Value::Integer(i32::from(param)));
        }

        #[test]
        fn can_convert_u16_values_into_integer_value(
            param in any::<u16>()
        ) {
            let value = Value::from(param);

            prop_assert_eq!(value, Value::Integer(i32::from(param)));
        }

        #[test]
        fn can_convert_u32_values_smaller_than_max_i32_into_integer_value(
            param in 0..=i32::max_value()
        ) {
            let value = Value::from(param as u32);

            prop_assert_eq!(value, Value::Integer(param));
        }

        #[test]
        fn can_convert_u32_values_greater_than_max_i32_into_long_value(
            param in (i32::max_value() as u32 + 1)..=u32::max_value()
        ) {
            let value = Value::from(param);

            prop_assert_eq!(value, Value::Long(param as i64));
        }

        #[test]
        fn can_convert_u64_values_smaller_than_max_i64_into_long_value(
            param in 0..=i64::max_value()
        ) {
            let value = Value::from(param as u64);

            prop_assert_eq!(value, Value::Long(param));
        }

        #[test]
        fn converting_a_u64_value_greater_than_max_i64_panics(
            param in (i64::max_value() as u64 + 1)..=u64::max_value()
        ) {
            let result = std::panic::catch_unwind(||
                Value::from(param)
            );

            prop_assert!(result.is_err());
        }
    }
}

mod field {
    use super::*;

    #[test]
    fn display_format_field_with_no_values() {
        let field = Field {
            name: "your message".into(),
            actual: None,
            expected: None,
        };

        assert_eq!(
            field.to_string(),
            "field: your message, actual: (n.a.), expected: (n.a.)"
        );
    }

    #[test]
    fn display_format_field_with_some_values_should_print_the_values_without_some() {
        let field = Field {
            name: "your message".into(),
            actual: Some(Value::Float(2.41)),
            expected: Some(Value::Float(1.0)),
        };

        assert_eq!(
            field.to_string(),
            "field: your message, actual: 2.41, expected: 1"
        );
    }
}

mod invalid_value {
    use super::*;

    #[test]
    fn display_format_invalid_value_of_field_with_actual_and_expected_value() {
        let invalid_value = InvalidValue {
            code: "invalid-allowed-characters".into(),
            field: Field {
                name: "code".into(),
                actual: Some(Value::String("Wlske324$2Asd".into())),
                expected: Some(Value::String("letters and digits".into())),
            },
        };

        assert_eq!(
            invalid_value.to_string(),
            "invalid-allowed-characters of code which is Wlske324$2Asd, expected to be letters and digits"
        );
    }
}

mod invalid_relation {
    use super::*;

    #[test]
    fn display_format_invalid_relation_of_percent_range() {
        let invalid_relation = InvalidRelation {
            code: "invalid-from-to".into(),
            field1: Field {
                name: "percent_from".into(),
                actual: Some(Value::Integer(50)),
                expected: None,
            },
            field2: Field {
                name: "percent_to".into(),
                actual: Some(Value::Integer(20)),
                expected: None,
            },
        };

        assert_eq!(
            invalid_relation.to_string(),
            "invalid-from-to of percent_from which is 50 and percent_to which is 20"
        );
    }
}

mod invalid_state {
    use super::*;

    #[test]
    fn display_format_invalid_state_can_format_a_list_of_parameters() {
        let invalid_state = InvalidState {
            code: "invalid-username-is-unique".into(),
            params: vec![Field {
                name: "username".into(),
                actual: Some("jon.doe".to_string().into()),
                expected: None,
            }],
        };

        assert_eq!(
            invalid_state.to_string(),
            "invalid-username-is-unique for parameters: [ { field: username, actual: jon.doe, expected: (n.a.) } ]"
        );
    }
}

mod validation_error {
    use super::*;

    #[test]
    fn display_format_validation_error_with_message_and_multiple_constraint_violations() {
        let validation_error = ValidationError {
            message: Some("validating my form".into()),
            violations: vec![
                invalid_value("invalid-bound-max", "age", 131, 130),
                invalid_state(
                    "invalid-unique-username",
                    vec![Field {
                        name: "username".into(),
                        actual: Some(Value::String("jon.doe".into())),
                        expected: None,
                    }],
                ),
            ],
        };

        assert_eq!(validation_error.to_string(), "validating my form: [ { invalid-bound-max of age which is 131, expected to be 130 }, { invalid-unique-username for parameters: [ { field: username, actual: jon.doe, expected: (n.a.) } ] } ]");
    }

    #[test]
    fn display_format_validation_error_no_message_and_one_constraint_violation() {
        let validation_error = ValidationError {
            message: None,
            violations: vec![invalid_value("invalid-bound-min", "age", 12, 13)],
        };

        assert_eq!(
            validation_error.to_string(),
            "[ { invalid-bound-min of age which is 12, expected to be 13 } ]"
        );
    }
}
