use super::*;

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
}

mod validation {
    use super::*;
    use crate::constraint::{Bound, NotEmpty};

    #[test]
    fn unfortunately_we_can_construct_an_instance_of_validated_without_doing_any_validation() {
        //TODO find a way to prevent this from compiling and still support the
        //     the possibility for custom implementations of the `Validate` trait

        let value: Validated<Bound<i32>, i32> = Validation::success(42)
            .result(Some("its not really validated".into()))
            .unwrap();

        assert_eq!(value.unwrap(), 42);

        let value: Validated<NotEmpty, String> = Validation::success("invalid".to_string())
            .result(Some("its not really validated".into()))
            .unwrap();

        assert_eq!(value.unwrap(), "invalid");
    }
}
