use crate::property::HasDecimalDigits;
use bigdecimal::BigDecimal;

impl HasDecimalDigits for BigDecimal {
    fn integer_digits(&self) -> u64 {
        let (_, exponent) = self.as_bigint_and_exponent();
        let num_digits = self.digits();
        if exponent > 0 {
            num_digits - exponent as u64
        } else if exponent < 0 {
            num_digits + exponent.abs() as u64
        } else {
            num_digits
        }
    }

    fn fraction_digits(&self) -> u64 {
        let (_, exponent) = self.as_bigint_and_exponent();
        if exponent > 0 {
            exponent as u64
        } else {
            0
        }
    }
}

#[cfg(not(feature = "num-traits"))]
mod without_num_traits {
    use crate::property::HasZeroValue;
    use bigdecimal::{BigDecimal, Zero};

    impl HasZeroValue for BigDecimal {
        fn is_zero_value(&self) -> bool {
            self.is_zero()
        }
    }
}
