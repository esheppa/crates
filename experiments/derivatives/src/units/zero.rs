use crate::units::{Quantity, StaticDisplay};
use num_traits::identities::Zero;

impl<Y> Zero for Quantity<Y>
where
    Y: StaticDisplay,
{
    fn zero() -> Self::Output {
        Quantity::new(rust_decimal::Decimal::zero())
    }
    fn is_zero(&self) -> bool {
        self.value.is_zero()
    }
}
