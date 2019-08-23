use crate::constraint::Decimal;
use bigdecimal::BigDecimal;

impl Decimal for BigDecimal {
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
