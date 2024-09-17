use crate::units::{Asset, Observable, Observation, Price, Quantity};
use std::ops;

use super::StaticDisplay;

impl<A, Y> ops::Add<Price<A, Y>> for Price<A, Y>
where
    Y: Asset,
    A: Asset,
{
    type Output = Price<A, Y>;
    fn add(self, rhs: Price<A, Y>) -> Self::Output {
        Price::new(self.value + rhs.value)
    }
}

impl<A, Y> ops::Add<rust_decimal::Decimal> for Price<A, Y>
where
    Y: Asset,
    A: Asset,
{
    type Output = Price<A, Y>;
    fn add(self, rhs: rust_decimal::Decimal) -> Self::Output {
        Price::new(self.value + rhs)
    }
}

impl<A, Y> ops::Add<Price<A, Y>> for rust_decimal::Decimal
where
    Y: Asset,
    A: Asset,
{
    type Output = Price<A, Y>;
    fn add(self, rhs: Price<A, Y>) -> Self::Output {
        Price::new(self + rhs.value)
    }
}

impl<U> ops::Add for Quantity<U>
where
    U: StaticDisplay,
{
    type Output = Quantity<U>;
    fn add(self, rhs: Quantity<U>) -> Self::Output {
        Quantity::new(self.value + rhs.value)
    }
}

impl<U> ops::Add<rust_decimal::Decimal> for Quantity<U>
where
    U: StaticDisplay,
{
    type Output = Quantity<U>;
    fn add(self, rhs: rust_decimal::Decimal) -> Self::Output {
        Quantity::new(self.value + rhs)
    }
}

impl<U> ops::Add<Quantity<U>> for rust_decimal::Decimal
where
    U: StaticDisplay,
{
    type Output = Quantity<U>;
    fn add(self, rhs: Quantity<U>) -> Self::Output {
        Quantity::new(self + rhs.value)
    }
}

impl<U> ops::AddAssign for Quantity<U>
where
    U: StaticDisplay,
{
    fn add_assign(&mut self, rhs: Quantity<U>) {
        self.value += rhs.value;
    }
}

impl<O> ops::Add for Observation<O>
where
    O: Observable,
{
    type Output = Observation<O>;
    fn add(self, rhs: Observation<O>) -> Self::Output {
        Observation::new(self.value + rhs.value)
    }
}

impl<O> ops::Add<rust_decimal::Decimal> for Observation<O>
where
    O: Observable,
{
    type Output = Observation<O>;
    fn add(self, rhs: rust_decimal::Decimal) -> Self::Output {
        Observation::new(self.value + rhs)
    }
}

impl<O> ops::Add<Observation<O>> for rust_decimal::Decimal
where
    O: Observable,
{
    type Output = Observation<O>;
    fn add(self, rhs: Observation<O>) -> Self::Output {
        Observation::new(self + rhs.value)
    }
}
