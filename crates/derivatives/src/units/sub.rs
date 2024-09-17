use crate::units::{Asset, Observable, Observation, Price, Quantity};
use std::ops;

use super::StaticDisplay;

impl<A, Y> ops::Sub<Price<A, Y>> for Price<A, Y>
where
    Y: Asset,
    A: Asset,
{
    type Output = Price<A, Y>;
    fn sub(self, rhs: Price<A, Y>) -> Self::Output {
        Price::new(self.value - rhs.value)
    }
}

impl<A, Y> ops::Sub<rust_decimal::Decimal> for Price<A, Y>
where
    Y: Asset,
    A: Asset,
{
    type Output = Price<A, Y>;
    fn sub(self, rhs: rust_decimal::Decimal) -> Self::Output {
        Price::new(self.value - rhs)
    }
}

impl<A, Y> ops::Sub<Price<A, Y>> for rust_decimal::Decimal
where
    Y: Asset,
    A: Asset,
{
    type Output = Price<A, Y>;
    fn sub(self, rhs: Price<A, Y>) -> Self::Output {
        Price::new(self - rhs.value)
    }
}

impl<U> ops::Sub for Quantity<U>
where
    U: StaticDisplay,
{
    type Output = Quantity<U>;
    fn sub(self, rhs: Quantity<U>) -> Self::Output {
        Quantity::new(self.value - rhs.value)
    }
}

impl<U> ops::Sub<rust_decimal::Decimal> for Quantity<U>
where
    U: StaticDisplay,
{
    type Output = Quantity<U>;
    fn sub(self, rhs: rust_decimal::Decimal) -> Self::Output {
        Quantity::new(self.value - rhs)
    }
}

impl<U> ops::Sub<Quantity<U>> for rust_decimal::Decimal
where
    U: StaticDisplay,
{
    type Output = Quantity<U>;
    fn sub(self, rhs: Quantity<U>) -> Self::Output {
        Quantity::new(self - rhs.value)
    }
}

impl<O> ops::Sub for Observation<O>
where
    O: Observable,
{
    type Output = Observation<O>;
    fn sub(self, rhs: Observation<O>) -> Self::Output {
        Observation::new(self.value - rhs.value)
    }
}

impl<O> ops::Sub<rust_decimal::Decimal> for Observation<O>
where
    O: Observable,
{
    type Output = Observation<O>;
    fn sub(self, rhs: rust_decimal::Decimal) -> Self::Output {
        Observation::new(self.value - rhs)
    }
}

impl<O> ops::Sub<Observation<O>> for rust_decimal::Decimal
where
    O: Observable,
{
    type Output = Observation<O>;
    fn sub(self, rhs: Observation<O>) -> Self::Output {
        Observation::new(self - rhs.value)
    }
}
