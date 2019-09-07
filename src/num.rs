use crate::property::HasZeroValue;
use num_traits::Zero;

impl<T> HasZeroValue for T
where
    T: Zero,
{
    fn is_zero_value(&self) -> bool {
        self.is_zero()
    }
}
