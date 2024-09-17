use std::{fmt, marker, ops};

pub mod currency;

mod add;
mod div;
mod mul;
mod ord;
mod sub;

// rethink this?
// mod rescale;

mod sum;
mod zero; // we impl zero, but one one, as we don't have impl for Price * Price // for <impl Iterator>.sum()

mod impls;

mod serde;

// should we remove mod::scale??
// pub mod scale;
// mod private {
//     pub trait Sealed {}
// }

// pub trait Scale:
//     Send + Sync + Clone + Copy + fmt::Debug + PartialEq + Eq + private::Sealed
// {
//     fn decimal() -> rust_decimal::Decimal;
//     fn pow() -> i32;
//     fn code() -> &'static str;
//     fn description() -> &'static str;
// }

pub trait StaticDisplay: PartialEq + Eq + Send + Sync + Clone + Copy {
    fn code() -> &'static str;
    fn description() -> &'static str;
}

// asset that we can possess  (physically or virtually)

// Ideally,
// The combination of two assets ( From and To) and a TimeResolution/DateResolution
// should be sufficient to uniquely identify a price.
// However,
// This is unlikely to work in practice, especially for 'option' style contracts
// which we still need to be able to get prices for. These 'option' contracts may have
// many different possible strike prices, each one having/inferring a different price
// The solution to this is to always use the "Underlying" asset, even for "Derivative"
// prices. This is not likely to cause particularly many erros in practice as it would
// be unsual to be dealing with both derivative prices and underlying prices at the same
// time and hence they are unlikely to be mixed up.
pub trait Asset: fmt::Debug + StaticDisplay {
    fn symbol() -> &'static str;
    // fn units() -> &'static str;
    // alternative to uom is for the user to build specific functions
    // that allow conversions between assets
    // type Unit: uom::si::Unit; ???
    // doesn't make sense to bake the scale into the type
    // as then it can't change...
    // type Scale: Scale;
}

// // Multiple derivatives can exist on the same asset,
// // but a given derivative always refers to a single underlying
// // This enables us to find the underlying given the derivative asset.
// pub trait Derivative: Asset {
//     type Underlying: Asset;

//     // this could be broken out to a seperate trait
//     fn price_to_underlying<Y>(price: Price<Self, Y>) -> Price<Self::Underlying, Y>
//     where
//         Y: Asset,
//     {
//         Price {
//             from: marker::PhantomData,
//             to: marker::PhantomData,
//             value: price.value,
//         }
//     }
//     fn quantity_to_underlying(quantity: Quantity<Self>) -> Quantity<Self::Underlying> {
//         Quantity {
//             units: marker::PhantomData,
//             value: quantity.value,
//         }
//     }
// }

// Multiply by the 'From' to get the 'To'
// This represents a swap between From to the To right now
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Price<F, T>
where
    // S1: Scale,
    F: Asset,
    // S2: Scale,
    T: Asset,
{
    // scale1: marker::PhantomData<S1>,
    from: marker::PhantomData<F>,
    // scale2: marker::PhantomData<S2>,
    to: marker::PhantomData<T>,
    // originally had delivery here,
    // however for a number of reasons it makes sense to store this information outside the price
    // 1. anyway it's not reflected in the type system so we have to use asserts
    // 2. it will sometimes make sense to operate together with different deliveries (eg average 4qtrs to make yearly price)
    // 3. this makes the impl similar to the others
    // delivery: resolution::TimeRange<A::DeliveryResolution>,
    value: rust_decimal::Decimal,
}

// Can be an amount of Asset or Contract, hence no restriction on the U.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Quantity<U>
where
    U: StaticDisplay,
{
    // scale: marker::PhantomData<S>,
    units: marker::PhantomData<U>,
    value: rust_decimal::Decimal,
}

// should we have an "inverse" method on SpotPrice that takes the reciprocal and swaps the `Asset`s?

// should we allow spot * forward? yes.
// should we require deliveries be matching when using forward * foward?
// usually the onlyh time we want mismatching is when we do spot * forwrd.
// so it makes sense to disallow mismatching resolutions if we do allow spot * forward

// spot * forward => spot
// forward * spot => spot
// forward * forward => forward
// forward / forward => forward
// spot / forward => spot
// forward / spot => spot

// forward * quantity => quantity
// quantity * forward => quantity
// quantity / quantity => forward (where U: Contract)
// quantity / forward => forward

// this works symmetrically
// cancels out the 'C' so that we get a F -> T
impl<C, F, T> ops::Mul<Price<C, T>> for Price<F, C>
where
    C: Asset,
    F: Asset,
    T: Asset,
{
    type Output = Price<F, T>;
    fn mul(self, rhs: Price<C, T>) -> Self::Output {
        Price::new(self.value * rhs.value)
    }
}

// cancels out the 'C' so that we get a F -> T
impl<C, F, T> ops::Div<Price<T, C>> for Price<F, C>
where
    C: Asset,
    F: Asset,
    T: Asset,
{
    type Output = Price<F, T>;
    fn div(self, rhs: Price<T, C>) -> Self::Output {
        Price::new(self.value / rhs.value)
    }
}

// We convert the Quantity from 'F' to 'T' using the Price
impl<F, T> ops::Mul<Quantity<F>> for Price<F, T>
where
    F: Asset,
    T: Asset,
{
    type Output = Quantity<T>;
    fn mul(self, rhs: Quantity<F>) -> Self::Output {
        Quantity::new(self.value * rhs.value)
    }
}

impl<F, T> ops::Mul<Price<F, T>> for Quantity<F>
where
    F: Asset,
    T: Asset,
{
    type Output = Quantity<T>;
    fn mul(self, rhs: Price<F, T>) -> Self::Output {
        Quantity::new(self.value * rhs.value)
    }
}

// Use the price in reverse, so now it converts the T to the F
impl<F, T> ops::Div<Price<F, T>> for Quantity<T>
where
    F: Asset,
    T: Asset,
{
    type Output = Quantity<F>;
    fn div(self, rhs: Price<F, T>) -> Self::Output {
        Quantity::new(self.value / rhs.value)
    }
}

// create a price using two quantities
impl<F, T> ops::Div<Quantity<F>> for Quantity<T>
where
    F: Asset,
    T: Asset,
{
    type Output = Price<F, T>;
    fn div(self, rhs: Quantity<F>) -> Self::Output {
        Price::new(self.value / rhs.value)
    }
}

//
pub trait Observable: fmt::Debug + PartialEq + Eq + Send + Sync + Clone + Copy {
    fn symbol() -> &'static str;
    fn code() -> &'static str;
    fn description() -> &'static str;
    // type Scale: Scale;
}

/// A concrete observation, for example:
///
/// 11 degrees celsuius (O: degrees celsius, S: Id)
/// 55 km/h wind gust (O: m/h wind gust, S: Kilo)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Observation<O>
where
    O: Observable,
{
    observable: marker::PhantomData<O>,
    value: rust_decimal::Decimal,
}
