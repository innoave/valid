use crate::constraint::{Digits, INVALID_DIGITS_FRACTION, INVALID_DIGITS_INTEGER};
use crate::{invalid_value, FieldName, Validate, Validation};
use bigdecimal::BigDecimal;

impl Validate<Digits, FieldName> for BigDecimal {
    fn validate(self, name: impl Into<FieldName>, constraint: &Digits) -> Validation<Self> {
        let (_, exponent) = self.as_bigint_and_exponent();
        let num_digits = self.digits();
        let (integer, fraction) = if exponent > 0 {
            let fraction = exponent as u64;
            (num_digits - fraction, fraction)
        } else if exponent < 0 {
            let integer = num_digits + exponent.abs() as u64;
            (integer, 0)
        } else {
            (num_digits, 0)
        };
        if integer <= constraint.integer {
            if fraction <= constraint.fraction {
                Validation::Success(self)
            } else {
                Validation::Failure(vec![invalid_value(
                    INVALID_DIGITS_FRACTION,
                    name,
                    fraction,
                    constraint.fraction,
                )])
            }
        } else if fraction <= constraint.fraction {
            Validation::Failure(vec![invalid_value(
                INVALID_DIGITS_INTEGER,
                name,
                num_digits,
                constraint.integer,
            )])
        } else {
            let name = name.into();
            Validation::Failure(vec![
                invalid_value(
                    INVALID_DIGITS_INTEGER,
                    name.clone(),
                    integer,
                    constraint.integer,
                ),
                invalid_value(INVALID_DIGITS_FRACTION, name, fraction, constraint.fraction),
            ])
        }
    }
}
