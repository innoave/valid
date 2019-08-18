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
                Value::Decimal(_) => 6,
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
                Value::Date(_) => 7,
                Value::DateTime(_) => 8,
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
                Value::Decimal(_) => 6,
                Value::Date(_) => 7,
                Value::DateTime(_) => 8,
            }
        }
        assert_eq!(exhaustive_match(Value::Integer(0)), 2);
    }
}

mod validation {
    use super::*;

    #[test]
    fn validation_of_type_that_does_not_implement_partial_eq() {
        struct TypeWithoutPartialEqImpl(i32);

        let value = Validation::Success(TypeWithoutPartialEqImpl(42));

        match value {
            Validation::Success(val) => assert_eq!(val.0, 42),
            Validation::Failure(_) => {}
        }
    }

    #[test]
    fn validation_of_type_that_implements_partial_eq() {
        #[derive(Debug, PartialEq)]
        struct TypeWithPartialEqImpl(i32);

        let value = Validation::Success(TypeWithPartialEqImpl(42));

        assert_eq!(value, Validation::Success(TypeWithPartialEqImpl(42)));
    }
}
