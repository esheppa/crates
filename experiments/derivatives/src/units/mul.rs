use crate::units::{Asset, Observable, Observation, Price, Quantity, StaticDisplay};
use std::ops;

// impl<A, Y> ops::Mul<SpotPrice<A, Y>> for SpotPrice<A, Y>
// where
//     Y: Asset,
//     A: Asset,
// {
//     type Output = SpotPrice<A, Y>;
//     fn mul(self, rhs: SpotPrice<A, Y>) -> Self::Output {
//         SpotPrice::new(self.value * rhs.value)
//     }
// }

impl<A, Y> ops::Mul<rust_decimal::Decimal> for Price<A, Y>
where
    Y: Asset,
    A: Asset,
{
    type Output = Price<A, Y>;
    fn mul(self, rhs: rust_decimal::Decimal) -> Self::Output {
        Price::new(self.value * rhs)
    }
}

impl<A, Y> ops::Mul<Price<A, Y>> for rust_decimal::Decimal
where
    Y: Asset,
    A: Asset,
{
    type Output = Price<A, Y>;
    fn mul(self, rhs: Price<A, Y>) -> Self::Output {
        Price::new(self * rhs.value)
    }
}

impl<U> ops::Mul for Quantity<U>
where
    U: StaticDisplay,
{
    type Output = Quantity<U>;
    fn mul(self, rhs: Quantity<U>) -> Self::Output {
        Quantity::new(self.value * rhs.value)
    }
}

impl<U> ops::Mul<rust_decimal::Decimal> for Quantity<U>
where
    U: StaticDisplay,
{
    type Output = Quantity<U>;
    fn mul(self, rhs: rust_decimal::Decimal) -> Self::Output {
        Quantity::new(self.value * rhs)
    }
}

impl<U> ops::Mul<Quantity<U>> for rust_decimal::Decimal
where
    U: StaticDisplay,
{
    type Output = Quantity<U>;
    fn mul(self, rhs: Quantity<U>) -> Self::Output {
        Quantity::new(self * rhs.value)
    }
}

impl<O> ops::Mul for Observation<O>
where
    O: Observable,
{
    type Output = Observation<O>;
    fn mul(self, rhs: Observation<O>) -> Self::Output {
        Observation::new(self.value * rhs.value)
    }
}

impl<O> ops::Mul<rust_decimal::Decimal> for Observation<O>
where
    O: Observable,
{
    type Output = Observation<O>;
    fn mul(self, rhs: rust_decimal::Decimal) -> Self::Output {
        Observation::new(self.value * rhs)
    }
}

impl<O> ops::Mul<Observation<O>> for rust_decimal::Decimal
where
    O: Observable,
{
    type Output = Observation<O>;
    fn mul(self, rhs: Observation<O>) -> Self::Output {
        Observation::new(self * rhs.value)
    }
}
