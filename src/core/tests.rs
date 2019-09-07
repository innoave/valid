use super::*;
use proptest::prelude::*;

mod validated {
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

    #[test]
    fn can_be_dereferenced_to_its_inner_value() {
        let validated: Validated<Bound<String>, _> =
            Validated(PhantomData, "some validated text".to_string());

        let inner_value: &str = &validated;

        assert_eq!(inner_value, "some validated text");
    }
}

mod context {
    use super::*;

    #[test]
    fn can_convert_a_str_into_a_field_name_context() {
        let field_name: FieldName = "your_name".into();

        assert_eq!(field_name, FieldName("your_name".into()));
    }

    #[test]
    fn can_dereference_a_fieldname_context_to_its_inner_value() {
        let field_name = FieldName("your_message".into());

        let inner_value: &str = &field_name;

        assert_eq!(inner_value, "your_message");
    }

    #[test]
    fn can_unwrap_a_field_name_context_into_its_inner_value() {
        let field_name = FieldName("your_comment".into());

        let inner = field_name.unwrap();

        assert_eq!(inner, Cow::Borrowed("your_comment"));
    }

    #[test]
    fn can_convert_a_tuple_of_str_into_a_related_fields_context() {
        let related_fields: RelatedFields = ("valid_from", "valid_until").into();

        assert_eq!(
            related_fields,
            RelatedFields("valid_from".into(), "valid_until".into())
        );
    }

    #[test]
    fn can_get_a_reference_to_the_first_field_of_a_relatedfields_context() {
        let related_fields = RelatedFields("valid_from".into(), "valid_until".into());

        let first_field: &str = related_fields.first();

        assert_eq!(first_field, "valid_from");
    }

    #[test]
    fn can_get_a_reference_to_the_second_field_of_a_relatedfields_context() {
        let related_fields = RelatedFields("valid_from".into(), "valid_until".into());

        let second_field: &str = related_fields.second();

        assert_eq!(second_field, "valid_until");
    }

    #[test]
    fn can_unwrap_a_related_fields_context_into_a_tuple() {
        let related_fields = RelatedFields("password".into(), "password2".into());

        let inner_tuple = related_fields.unwrap();

        assert_eq!(
            inner_tuple,
            (Cow::Borrowed("password"), Cow::Borrowed("password2"))
        );
    }

    #[test]
    fn can_convert_a_custom_value_into_a_state_context() {
        let state: State<Vec<_>> = vec![25, 50, 75].into();

        assert_eq!(state, State(vec![25, 50, 75]));
    }

    #[test]
    fn can_dereference_a_state_context_to_its_inner_value() {
        let state: State<Vec<_>> = vec![25, 50, 75].into();

        let inner_value: &[_] = &state;

        assert_eq!(inner_value, &[25, 50, 75]);
    }

    #[test]
    fn can_unwrap_a_state_context_into_its_inner_value() {
        let state: State<Vec<_>> = vec![25, 50, 75].into();

        let inner_value: Vec<_> = state.unwrap();

        assert_eq!(inner_value, vec![25, 50, 75]);
    }
}

mod validation {
    use super::*;

    #[test]
    fn get_the_result_of_a_successful_validation_without_a_message() {
        let validation: Validation<(), _> = Validation::success("valid text".to_string());

        let result = validation.result();

        assert_eq!(result, Ok(Validated(PhantomData, "valid text".to_string())));
    }

    #[test]
    fn get_the_result_of_a_failed_validation_without_a_message() {
        let validation: Validation<(), String> =
            Validation::failure(vec![invalid_state("invalid-unique-username", vec![])]);

        let result = validation.result();

        assert_eq!(
            result,
            Err(ValidationError {
                message: None,
                violations: vec![InvalidState {
                    code: "invalid-unique-username".into(),
                    params: vec![],
                }
                .into()]
            })
        );
    }

    #[test]
    fn get_the_result_of_a_successful_validation_with_a_message() {
        let validation: Validation<(), _> = Validation::success("valid text".to_string());

        let result = validation.with_message("validating register new user command");

        assert_eq!(result, Ok(Validated(PhantomData, "valid text".to_string())));
    }

    #[test]
    fn get_the_result_of_a_failed_validation_with_a_message() {
        let validation: Validation<(), String> =
            Validation::failure(vec![invalid_state("invalid-unique-username", vec![])]);

        let result = validation.with_message("validating register new user command");

        assert_eq!(
            result,
            Err(ValidationError {
                message: Some("validating register new user command".into()),
                violations: vec![InvalidState {
                    code: "invalid-unique-username".into(),
                    params: vec![],
                }
                .into()]
            })
        );
    }

    #[test]
    fn combine_a_successful_validation_with_another_value_that_needs_no_further_validation() {
        let validation: Validation<(), _> = Validation::success("valid text".to_string());

        let combined = validation.combine(42);

        assert_eq!(
            combined,
            Validation::success((42, "valid text".to_string()))
        );
    }

    #[test]
    fn combine_a_failed_validation_with_another_value_has_no_effect() {
        let validation: Validation<(), i32> = Validation::failure(vec![invalid_state(
            "invalid-unique",
            vec![Parameter::new("reference_code", 42)],
        )]);

        let combined = validation.combine("another value");

        assert_eq!(
            combined,
            Validation::failure(vec![invalid_state(
                "invalid-unique",
                vec![Parameter::new("reference_code", 42)]
            )])
        );
    }

    #[test]
    fn map_the_values_of_a_successful_validation_into_a_custom_struct() {
        #[derive(Debug, PartialEq)]
        struct RegisterUserForm {
            username: String,
            age: i32,
        }

        let validation: Validation<(), _> = Validation::success((42, "jane.doe".to_string()));

        let mapped: Validation<(), _> =
            validation.map(|(age, username)| RegisterUserForm { username, age });

        assert_eq!(
            mapped,
            Validation::success(RegisterUserForm {
                username: "jane.doe".into(),
                age: 42,
            })
        );
    }

    #[test]
    fn mapping_the_value_of_a_failed_validation_has_no_effect() {
        #[derive(Debug, PartialEq)]
        struct MyStruct(i32);

        let validation: Validation<(), i32> = Validation::failure(vec![invalid_state(
            "invalid-unique",
            vec![Parameter::new("reference_code", 42)],
        )]);

        let mapped: Validation<(), _> = validation.map(MyStruct);

        assert_eq!(
            mapped,
            Validation::failure(vec![invalid_state(
                "invalid-unique",
                vec![Parameter::new("reference_code", 42)]
            )])
        );
    }

    #[test]
    fn combine_two_validations_with_and_where_both_are_successful() {
        let username = String::from("jane.doe");
        let age = 42;

        let validation1: Validation<(), _> = Validation::success(username);
        let validation2: Validation<(), _> = Validation::success(age);

        let resulting_validation = validation1.and(validation2);

        assert_eq!(
            resulting_validation,
            Validation::success((String::from("jane.doe"), 42))
        );
    }

    #[test]
    fn combine_two_validations_with_and_where_the_first_failed() {
        let _username = String::from("jane:doe");
        let age = 42;

        let validation1: Validation<(), String> = Validation::failure(vec![invalid_value(
            "invalid-character",
            "username",
            ":".to_string(),
            "valid username".to_string(),
        )]);
        let validation2: Validation<(), _> = Validation::success(age);

        let resulting_validation = validation1.and(validation2);

        assert_eq!(
            resulting_validation,
            Validation::failure(vec![invalid_value(
                "invalid-character",
                "username",
                ":".to_string(),
                "valid username".to_string(),
            )])
        );
    }

    #[test]
    fn combine_two_validations_with_and_where_the_second_failed() {
        let username = String::from("jane.doe");
        let _age = 7;

        let validation1: Validation<(), _> = Validation::success(username);
        let validation2: Validation<(), i32> =
            Validation::failure(vec![invalid_value("invalid-age", "age", 7, 13)]);

        let resulting_validation = validation1.and(validation2);

        assert_eq!(
            resulting_validation,
            Validation::failure(vec![invalid_value("invalid-age", "age", 7, 13,)])
        );
    }

    #[test]
    fn combine_two_validations_with_and_where_both_are_failing() {
        let _username = String::from("jane:doe");
        let _age = 7;

        let validation1: Validation<(), String> = Validation::failure(vec![invalid_value(
            "invalid-character",
            "username",
            ":".to_string(),
            "valid username".to_string(),
        )]);
        let validation2: Validation<(), i32> =
            Validation::failure(vec![invalid_value("invalid-age", "age", 7, 13)]);

        let resulting_validation = validation1.and(validation2);

        assert_eq!(
            resulting_validation,
            Validation::failure(vec![
                invalid_value(
                    "invalid-character",
                    "username",
                    ":".to_string(),
                    "valid username".to_string(),
                ),
                invalid_value("invalid-age", "age", 7, 13,)
            ])
        );
    }

    #[test]
    fn combine_two_validations_with_and_then_where_both_are_successful() {
        let password = String::from("s3cr3t");
        let password2 = String::from("s3cr3t");

        let validation1: Validation<(), _> = Validation::success(password);

        let resulting_validation: Validation<(), _> =
            validation1.and_then(|password| Validation::success((password, password2)));

        assert_eq!(
            resulting_validation,
            Validation::success((String::from("s3cr3t"), String::from("s3cr3t")))
        );
    }

    #[test]
    fn combine_two_validations_with_and_then_where_the_first_failed() {
        let _password = String::from("s3");
        let password2 = String::from("s3");

        let validation1: Validation<(), String> =
            Validation::failure(vec![invalid_value("invalid-length-min", "password", 2, 6)]);

        let resulting_validation: Validation<(), _> =
            validation1.and_then(|password| Validation::success((password, password2)));

        assert_eq!(
            resulting_validation,
            Validation::failure(vec![invalid_value("invalid-length-min", "password", 2, 6)])
        );
    }

    #[test]
    fn combine_two_validations_with_and_then_where_the_second_failed() {
        let password = String::from("s3cr3t");
        let _password2 = String::from("s3crEt");

        let validation1: Validation<(), _> = Validation::success(password);

        let resulting_validation: Validation<(), String> = validation1.and_then(|_password| {
            Validation::failure(vec![invalid_relation(
                "invalid-must-match",
                "password",
                "s3cr3t".to_string(),
                "password2",
                "s3crEt".to_string(),
            )])
        });

        assert_eq!(
            resulting_validation,
            Validation::failure(vec![invalid_relation(
                "invalid-must-match",
                "password",
                "s3cr3t".to_string(),
                "password2",
                "s3crEt".to_string(),
            )])
        );
    }

    #[test]
    fn combine_two_validations_with_and_then_where_both_are_failing() {
        let _password = String::from("s3");
        let _password2 = String::from("s3crEt");

        let validation1: Validation<(), String> =
            Validation::failure(vec![invalid_value("invalid-length-min", "password", 2, 6)]);

        let resulting_validation: Validation<(), String> = validation1.and_then(|_password| {
            Validation::failure(vec![invalid_relation(
                "invalid-must-match",
                "password",
                "s3cr3t".to_string(),
                "password2",
                "s3crEt".to_string(),
            )])
        });

        assert_eq!(
            resulting_validation,
            Validation::failure(vec![invalid_value("invalid-length-min", "password", 2, 6)])
        );
    }
}

mod value {
    use super::*;

    #[cfg(not(any(feature = "bigdecimal", feature = "chrono", feature = "num-bigint")))]
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

    #[cfg(all(
        feature = "bigdecimal",
        not(feature = "chrono"),
        not(feature = "num-bigint")
    ))]
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

    #[cfg(all(
        not(feature = "bigdecimal"),
        feature = "chrono",
        not(feature = "num-bigint")
    ))]
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

    #[cfg(all(
        not(feature = "bigdecimal"),
        not(feature = "chrono"),
        feature = "num-bigint"
    ))]
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
                Value::BigInteger(_) => 10,
            }
        }
        assert_eq!(exhaustive_match(Value::Integer(0)), 2);
    }

    #[cfg(all(feature = "bigdecimal", feature = "chrono", feature = "num-bigint"))]
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
                Value::BigInteger(_) => 10,
            }
        }
        assert_eq!(exhaustive_match(Value::Integer(0)), 2);
    }

    #[test]
    fn display_format_a_value_of_string() {
        let value = Value::String("some text".into());

        assert_eq!(value.to_string(), "some text");
    }

    #[test]
    fn display_format_a_value_of_integer() {
        let value = Value::Integer(42);

        assert_eq!(value.to_string(), "42");
    }

    #[test]
    fn display_format_a_value_of_long() {
        let value = Value::Long(-293_848_928_192);

        assert_eq!(value.to_string(), "-293848928192");
    }

    #[test]
    fn display_format_a_value_of_float() {
        let value = Value::Float(-2.54);

        assert_eq!(value.to_string(), "-2.54");
    }

    #[test]
    fn display_format_a_value_of_double() {
        let value = Value::Double(0.012_345_678_9);

        assert_eq!(value.to_string(), "0.0123456789");
    }

    #[test]
    fn display_format_a_value_of_boolean() {
        let value = Value::Boolean(true);

        assert_eq!(value.to_string(), "true");
    }

    #[cfg(feature = "bigdecimal")]
    #[test]
    fn display_format_a_value_of_bigdecimal() {
        use std::str::FromStr;

        let value = Value::Decimal(BigDecimal::from_str("1280.77101").unwrap());

        assert_eq!(value.to_string(), "1280.77101");
    }

    #[cfg(feature = "chrono")]
    #[test]
    fn display_format_a_value_of_date() {
        let value = Value::Date(NaiveDate::from_ymd(2019, 8, 31));

        assert_eq!(value.to_string(), "2019-08-31");
    }

    #[cfg(feature = "chrono")]
    #[test]
    fn display_format_a_value_of_date_time() {
        let value = Value::DateTime(Utc.ymd(2019, 8, 31).and_hms(12, 02, 59));

        assert_eq!(value.to_string(), "2019-08-31 12:02:59 UTC");
    }

    #[cfg(feature = "num-bigint")]
    #[test]
    fn display_format_a_value_of_big_integer() {
        use std::str::FromStr;

        let value = Value::BigInteger(BigInt::from_str("128077101").unwrap());

        assert_eq!(value.to_string(), "128077101");
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

            prop_assert_eq!(value, Value::Long(i64::from(param)));
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

        #[ignore] //TODO decide whether to keep From<u64> which might panic or support TryFrom<u64> only
        #[test]
        fn try_from_u64_never_panics(
            value in any::<u64>()
        ) {
            let _result = Value::try_from(value);
        }

        #[test]
        fn try_from_usize_never_panics(
            value in any::<usize>()
        ) {
            let _result = Value::try_from(value);
        }

        #[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
        #[test]
        fn try_from_usize_value_less_than_or_equal_i32_max(
            value in 0..=i32::max_value()
        ) {
            let result = Value::try_from(value as usize);

            prop_assert_eq!(result, Ok(Value::Integer(value)));
        }

        #[cfg(target_pointer_width = "64")]
        #[test]
        fn try_from_usize_value_greater_than_i32_max_and_less_than_or_equal_i64_max(
            value in i64::from(i32::max_value()) + 1..=i64::max_value()
        ) {
            let result = Value::try_from(value as usize);

            prop_assert_eq!(result, Ok(Value::Long(value)));
        }

        #[cfg(target_pointer_width = "64")]
        #[test]
        fn try_from_usize_value_greater_than_i64_max(
            value in i64::max_value() as u64 + 1..=u64::max_value()
        ) {
            let result = Value::try_from(value as usize);

            prop_assert_eq!(result, Err("usize value too big to be converted to i64"));
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
            code: "invalid-must-define-range-inclusive".into(),
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
            "invalid-must-define-range-inclusive of percent_from which is 50 and percent_to which is 20"
        );
    }
}

mod invalid_state {
    use super::*;

    #[test]
    fn display_format_invalid_state_can_format_a_list_of_parameters() {
        let invalid_state = InvalidState {
            code: "invalid-username-is-unique".into(),
            params: vec![Parameter {
                name: "username".into(),
                value: "jon.doe".to_string().into(),
            }],
        };

        assert_eq!(
            invalid_state.to_string(),
            "invalid-username-is-unique for parameters: [ username=jon.doe ]"
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
                    vec![Parameter {
                        name: "username".into(),
                        value: Value::String("jon.doe".into()),
                    }],
                ),
            ],
        };

        assert_eq!(validation_error.to_string(), "validating my form: [ invalid-bound-max of age which is 131, expected to be 130 / invalid-unique-username for parameters: [ username=jon.doe ] ]");
    }

    #[test]
    fn display_format_validation_error_no_message_and_one_constraint_violation() {
        let validation_error = ValidationError {
            message: None,
            violations: vec![invalid_value("invalid-bound-min", "age", 12, 13)],
        };

        assert_eq!(
            validation_error.to_string(),
            "[ invalid-bound-min of age which is 12, expected to be 13 ]"
        );
    }

    #[test]
    fn merge_two_validation_errors_with_messages_into_one() {
        let validation_error1 = ValidationError {
            message: Some("validating a user's age".into()),
            violations: vec![invalid_value("invalid-bound-min", "age", 12, 13)],
        };
        let validation_error2 = ValidationError {
            message: Some("validating a user registration command".into()),
            violations: vec![invalid_value("invalid-length-min", "username", 3, 4)],
        };

        let merged_error = validation_error2.merge(validation_error1);

        assert_eq!(
            merged_error,
            ValidationError {
                message: Some(
                    "validating a user registration command / validating a user's age".into()
                ),
                violations: vec![
                    invalid_value("invalid-length-min", "username", 3, 4),
                    invalid_value("invalid-bound-min", "age", 12, 13),
                ]
            }
        );
    }

    #[test]
    fn merge_two_validation_errors_where_the_first_contains_a_message() {
        let validation_error1 = ValidationError {
            message: Some("validating a user's age".into()),
            violations: vec![invalid_value("invalid-bound-min", "age", 12, 13)],
        };
        let validation_error2 = ValidationError {
            message: None,
            violations: vec![invalid_value("invalid-length-min", "username", 3, 4)],
        };

        let merged_error = validation_error2.merge(validation_error1);

        assert_eq!(
            merged_error,
            ValidationError {
                message: Some("validating a user's age".into()),
                violations: vec![
                    invalid_value("invalid-length-min", "username", 3, 4),
                    invalid_value("invalid-bound-min", "age", 12, 13),
                ]
            }
        );
    }

    #[test]
    fn merge_two_validation_errors_where_the_second_contains_a_message() {
        let validation_error1 = ValidationError {
            message: None,
            violations: vec![invalid_value("invalid-bound-min", "age", 12, 13)],
        };
        let validation_error2 = ValidationError {
            message: Some("validating a user registration command".into()),
            violations: vec![invalid_value("invalid-length-min", "username", 3, 4)],
        };

        let merged_error = validation_error2.merge(validation_error1);

        assert_eq!(
            merged_error,
            ValidationError {
                message: Some("validating a user registration command".into()),
                violations: vec![
                    invalid_value("invalid-length-min", "username", 3, 4),
                    invalid_value("invalid-bound-min", "age", 12, 13),
                ]
            }
        );
    }

    #[test]
    fn merge_two_validation_errors_where_none_of_them_contains_a_message() {
        let validation_error1 = ValidationError {
            message: None,
            violations: vec![invalid_value("invalid-bound-min", "age", 12, 13)],
        };
        let validation_error2 = ValidationError {
            message: None,
            violations: vec![invalid_value("invalid-length-min", "username", 3, 4)],
        };

        let merged_error = validation_error2.merge(validation_error1);

        assert_eq!(
            merged_error,
            ValidationError {
                message: None,
                violations: vec![
                    invalid_value("invalid-length-min", "username", 3, 4),
                    invalid_value("invalid-bound-min", "age", 12, 13),
                ]
            }
        );
    }
}
