use crate::units::{Asset, Observable, Observation, Price, Quantity, StaticDisplay};
use std::cmp;

impl<A, Y> cmp::PartialOrd for Price<A, Y>
where
    Y: Asset,
    A: Asset,
{
    fn partial_cmp(&self, rhs: &Price<A, Y>) -> Option<cmp::Ordering> {
        (self.value).partial_cmp(&rhs.value)
    }
}
impl<A, Y> cmp::Ord for Price<A, Y>
where
    Y: Asset,
    A: Asset,
{
    fn cmp(&self, rhs: &Price<A, Y>) -> cmp::Ordering {
        (self.value).cmp(&rhs.value)
    }
}

impl<O> cmp::PartialOrd for Observation<O>
where
    O: Observable,
{
    fn partial_cmp(&self, rhs: &Observation<O>) -> Option<cmp::Ordering> {
        self.value.partial_cmp(&rhs.value)
    }
}
impl<O> cmp::Ord for Observation<O>
where
    O: Observable,
{
    fn cmp(&self, rhs: &Observation<O>) -> cmp::Ordering {
        self.value.cmp(&rhs.value)
    }
}

impl<U> cmp::PartialOrd for Quantity<U>
where
    U: StaticDisplay,
{
    fn partial_cmp(&self, rhs: &Quantity<U>) -> Option<cmp::Ordering> {
        self.value.partial_cmp(&rhs.value)
    }
}
impl<U> cmp::Ord for Quantity<U>
where
    U: StaticDisplay,
{
    fn cmp(&self, rhs: &Quantity<U>) -> cmp::Ordering {
        self.value.cmp(&rhs.value)
    }
}
