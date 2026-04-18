use crate::units::{Asset, Observable, Observation, Price, Quantity, StaticDisplay};
use std::ops;

// impl<A, Y> ops::Div<SpotPrice<A, Y>> for SpotPrice<A, Y>
// where
//     Y: Asset,
//     A: Asset,
// {
//     type Output = SpotPrice<A, Y>;
//     fn div(self, rhs: SpotPrice<A, Y>) -> Self::Output {
//         SpotPrice::new(self.value / rhs.value)
//     }
// }

impl<A, Y> ops::Div<rust_decimal::Decimal> for Price<A, Y>
where
    Y: Asset,
    A: Asset,
{
    type Output = Price<A, Y>;
    fn div(self, rhs: rust_decimal::Decimal) -> Self::Output {
        Price::new(self.value / rhs)
    }
}

impl<A, Y> ops::Div<Price<A, Y>> for rust_decimal::Decimal
where
    Y: Asset,
    A: Asset,
{
    type Output = Price<A, Y>;
    fn div(self, rhs: Price<A, Y>) -> Self::Output {
        Price::new(self / rhs.value)
    }
}

// impl<U> ops::Div for Quantity<U>
// where
//     U: StaticDisplay,
// {
//     type Output = Quantity<U>;
//     fn div(self, rhs: Quantity<U>) -> Self::Output {
//         Quantity::new(self.value / rhs.value)
//     }
// }

impl<U> ops::Div<rust_decimal::Decimal> for Quantity<U>
where
    U: StaticDisplay,
{
    type Output = Quantity<U>;
    fn div(self, rhs: rust_decimal::Decimal) -> Self::Output {
        Quantity::new(self.value / rhs)
    }
}

impl<U> ops::Div<Quantity<U>> for rust_decimal::Decimal
where
    U: StaticDisplay,
{
    type Output = Quantity<U>;
    fn div(self, rhs: Quantity<U>) -> Self::Output {
        Quantity::new(self / rhs.value)
    }
}

impl<O> ops::Div for Observation<O>
where
    O: Observable,
{
    type Output = Observation<O>;
    fn div(self, rhs: Observation<O>) -> Self::Output {
        Observation::new(self.value / rhs.value)
    }
}

impl<O> ops::Div<rust_decimal::Decimal> for Observation<O>
where
    O: Observable,
{
    type Output = Observation<O>;
    fn div(self, rhs: rust_decimal::Decimal) -> Self::Output {
        Observation::new(self.value / rhs)
    }
}

impl<O> ops::Div<Observation<O>> for rust_decimal::Decimal
where
    O: Observable,
{
    type Output = Observation<O>;
    fn div(self, rhs: Observation<O>) -> Self::Output {
        Observation::new(self / rhs.value)
    }
}
