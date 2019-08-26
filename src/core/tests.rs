use super::*;
use proptest::prelude::*;

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
