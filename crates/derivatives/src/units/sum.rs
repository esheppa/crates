use crate::units::{Quantity, StaticDisplay};
use std::iter::Sum;

impl<U> Sum for Quantity<U>
where
    U: StaticDisplay,
{
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        Quantity::new(iter.map(|v| v.value).sum())
    }
}

trait StatsIter<T>: std::iter::Iterator<Item = T>
where
    T: std::ops::Add
        + std::ops::Div<rust_decimal::Decimal>
        + num_traits::identities::Zero
        + Sized
        + std::cmp::Ord,
{
    // Calculate histogram using online algorithm
    fn histogram(&mut self, _bucket: rust_decimal::Decimal) -> std::collections::BTreeMap<T, u64> {
        todo!()
    }

    // Calculate median using online algorithm
    fn median(&mut self) -> T {
        todo!()
    }

    // Calculate mode using online algorithm
    fn mode(&mut self) -> T {
        todo!()
    }

    // Calculate average using online algorithm
    fn average(&mut self) -> T {
        todo!()
    }

    // Calculate stdev using online algorithm
    fn standard_deviation(&mut self) -> T {
        todo!()
    }

    // Calculate stdev using online algorithm
    fn variance(&mut self) -> T {
        todo!()
    }
}

impl<T, U> StatsIter<U> for T
where
    T: std::iter::Iterator<Item = U>,
    U: std::ops::Add
        + std::ops::Div<rust_decimal::Decimal>
        + num_traits::identities::Zero
        + Sized
        + std::cmp::Ord,
{
}

// convenient average for set of prices / obs / forex needed
