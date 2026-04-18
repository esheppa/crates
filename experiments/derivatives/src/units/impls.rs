use crate::units::{Asset, Observable, Observation, Price, Quantity, StaticDisplay};

use std::marker;

impl<A, C> Price<A, C>
where
    A: Asset,
    C: Asset,
{
    pub fn new(value: rust_decimal::Decimal) -> Price<A, C> {
        Price {
            from: marker::PhantomData,
            to: marker::PhantomData,
            value,
        }
    }
    pub fn get(&self) -> rust_decimal::Decimal {
        self.value
    }
}

impl<O> Observation<O>
where
    O: Observable,
{
    pub fn new(value: rust_decimal::Decimal) -> Observation<O> {
        Observation {
            observable: marker::PhantomData,
            value,
        }
    }
    pub fn get(&self) -> rust_decimal::Decimal {
        self.value
    }
}

impl<U> Quantity<U>
where
    U: StaticDisplay,
{
    pub fn new(value: rust_decimal::Decimal) -> Quantity<U> {
        Quantity {
            value,
            units: marker::PhantomData,
        }
    }
    pub fn get(&self) -> rust_decimal::Decimal {
        self.value
    }
}
