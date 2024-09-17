use crate::units::{self, Observable};
use chrono::Datelike;
use std::{any, collections, iter};
// use units::{Currency, Price};
//

// if have strike, can allow:
//
// price - strike, strike - price => ??
// or maybe just use price for strikes
//
// impl PartialEq, Eq, PartialOrd, Ord for amount, price, forex, quantity
//

pub struct BlankProvider;

// this should collect up and report on
// all the required dates and assets etc required
// to complete the settlements or mark to market process

#[derive(PartialEq, Eq, Hash)]
pub struct PriceKey {
    pub effective: any::TypeId, // the contract delivery resolution
    pub delivery: any::TypeId,  // the contract delivery resolution
    pub from: &'static str,     // the `from` asset
    pub to: &'static str,       // the `to` asset\
    pub contract: String,       // the contract string
}

#[derive(PartialEq, Eq, Hash)]
pub struct QuantityKey {
    pub resolution: any::TypeId, // the contract resolution
    pub asset: &'static str,     // the asset
    pub contract: String,        // the contract string
}

#[derive(PartialEq, Eq, Hash)]
pub struct CalendarKey {
    pub resolution: any::TypeId, // the resolution
    pub calendar: String,        // the calendar string
}

#[derive(PartialEq, Eq, Hash)]
pub struct ObservationKey {
    pub resolution: any::TypeId,  // the contract resolution
    pub observable: &'static str, // the observable
    pub location: String,         // the location
}

pub struct RequirementsProvider {
    price_requests: collections::HashMap<PriceKey, collections::BTreeSet<(i64, i64)>>,
    quantity_requests: collections::HashMap<QuantityKey, collections::BTreeSet<i64>>,
    calendar_requests: collections::HashMap<CalendarKey, collections::BTreeSet<i64>>,
    observation_requests: collections::HashMap<ObservationKey, collections::BTreeSet<i64>>,
}

impl RequirementsProvider {
    pub fn raw_price_requests(
        &self,
    ) -> &collections::HashMap<PriceKey, collections::BTreeSet<(i64, i64)>> {
        &self.price_requests
    }
    pub fn raw_quantity_requests(
        &self,
    ) -> &collections::HashMap<QuantityKey, collections::BTreeSet<i64>> {
        &self.quantity_requests
    }
    pub fn raw_calendar_requests(
        &self,
    ) -> &collections::HashMap<CalendarKey, collections::BTreeSet<i64>> {
        &self.calendar_requests
    }
    pub fn raw_observation_requests(
        &self,
    ) -> &collections::HashMap<ObservationKey, collections::BTreeSet<i64>> {
        &self.observation_requests
    }
}

#[async_trait::async_trait]
impl PriceProvider for RequirementsProvider {
    async fn get_prices<A, Y, E, P>(
        &mut self,
        request: &PriceRequest<E, P>,
        contract: &str,
    ) -> crate::Result<Prices<E, A, Y, P>>
    where
        P: resolution::TimeResolution + 'static,
        E: resolution::TimeResolution + 'static,
        A: units::Asset + 'static,
        Y: units::Asset + 'static,
    {
        self.price_requests
            .entry(PriceKey {
                effective: any::TypeId::of::<E>(),
                delivery: any::TypeId::of::<P>(),
                from: A::code(),
                to: Y::code(),
                contract: contract.to_string(),
            })
            .and_modify(|val| val.extend(request.request()))
            .or_insert_with(|| request.request());

        Ok(Prices::default())
    }
}

#[async_trait::async_trait]
impl QuantityProvider for RequirementsProvider {
    async fn get_quantities<P, U>(
        &mut self,
        period: resolution::TimeRange<P>,
        contract: &str,
    ) -> crate::Result<collections::BTreeMap<P, units::Quantity<U>>>
    where
        P: resolution::TimeResolution + 'static,
        U: units::StaticDisplay + 'static,
    {
        self.quantity_requests
            .entry(QuantityKey {
                resolution: any::TypeId::of::<P>(),
                asset: U::code(),
                contract: contract.to_string(),
            })
            .and_modify(|val| val.extend(period.iter().map(|tr| tr.to_monotonic())))
            .or_insert_with(|| period.iter().map(|tr| tr.to_monotonic()).collect());

        Ok(collections::BTreeMap::new())
    }
}

#[async_trait::async_trait]
impl CalendarProvider for RequirementsProvider {
    async fn get_holidays<P: resolution::TimeResolution + 'static>(
        &mut self,
        period: resolution::TimeRange<P>,
        calendar: &str,
    ) -> crate::Result<collections::BTreeSet<chrono::NaiveDate>> {
        self.calendar_requests
            .entry(CalendarKey {
                resolution: any::TypeId::of::<P>(),
                calendar: calendar.to_string(),
            })
            .and_modify(|val| val.extend(period.iter().map(|tr| tr.to_monotonic())))
            .or_insert_with(|| period.iter().map(|tr| tr.to_monotonic()).collect());

        Ok(collections::BTreeSet::new())
    }
}

#[async_trait::async_trait]
impl ObservationProvider for RequirementsProvider {
    async fn get_observations<P, O>(
        &mut self,
        period: resolution::TimeRange<P>,
        location: &str,
    ) -> crate::Result<collections::BTreeMap<P, units::Observation<O>>>
    where
        P: resolution::TimeResolution + 'static,
        O: units::Observable + 'static,
    {
        self.observation_requests
            .entry(ObservationKey {
                resolution: any::TypeId::of::<P>(),
                observable: O::code(),
                location: location.to_string(),
            })
            .and_modify(|val| val.extend(period.iter().map(|tr| tr.to_monotonic())))
            .or_insert_with(|| period.iter().map(|tr| tr.to_monotonic()).collect());

        Ok(collections::BTreeMap::new())
    }
}

struct FileProvider {
    currencies: Vec<()>,
    prices: Vec<()>,
    quantities: Vec<()>,
    calendars: Vec<()>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub struct PriceMetadata<E: resolution::TimeResolution, D: resolution::TimeResolution> {
    pub effective: E,
    pub delivery: D,
}

impl<E, D> PriceMetadata<E, D>
where
    E: resolution::TimeResolution,
    D: resolution::TimeResolution,
{
    fn as_tuple(&self) -> (i64, i64) {
        (self.effective.to_monotonic(), self.delivery.to_monotonic())
    }
    fn from_tuple(tup: (i64, i64)) -> Self {
        PriceMetadata {
            effective: resolution::Monotonic::from_monotonic(tup.0),
            delivery: resolution::Monotonic::from_monotonic(tup.1),
        }
    }
}

pub struct PriceRequest<E: resolution::TimeResolution, D: resolution::TimeResolution> {
    series: std::collections::BTreeSet<PriceMetadata<E, D>>,
}

impl<D> PriceRequest<D, D>
where
    D: resolution::TimeResolution,
{
    pub fn spot_range(range: resolution::TimeRange<D>) -> PriceRequest<D, D> {
        PriceRequest {
            series: range
                .iter()
                .map(|x| PriceMetadata {
                    effective: x,
                    delivery: x,
                })
                .collect(),
        }
    }
}

impl<E, D> PriceRequest<E, D>
where
    E: resolution::TimeResolution,
    D: resolution::TimeResolution,
{
    pub fn empty() -> PriceRequest<E, D> {
        PriceRequest {
            series: collections::BTreeSet::new(),
        }
    }
    pub fn single(metadata: PriceMetadata<E, D>) -> PriceRequest<E, D> {
        PriceRequest {
            series: iter::once(metadata).collect(),
        }
    }
    pub fn from_range(effective: E, range: resolution::TimeRange<D>) -> PriceRequest<E, D> {
        PriceRequest {
            series: range
                .iter()
                .map(|x| PriceMetadata {
                    effective,
                    delivery: x,
                })
                .collect(),
        }
    }

    pub fn add(&mut self, metadata: PriceMetadata<E, D>) {
        self.series.insert(metadata);
    }

    // temp
    pub fn request(&self) -> collections::BTreeSet<(i64, i64)> {
        // this will succeed as we can only create one of these via the From impls
        self.series
            .iter()
            .map(|md| (md.effective.to_monotonic(), md.delivery.to_monotonic()))
            .collect()
    }
}

impl<R> From<resolution::TimeRange<R>> for PriceRequest<R, R>
where
    R: resolution::TimeResolution,
{
    fn from(range: resolution::TimeRange<R>) -> PriceRequest<R, R> {
        PriceRequest {
            series: range
                .iter()
                .map(|r| PriceMetadata {
                    effective: r,
                    delivery: r,
                })
                .collect(),
        }
    }
}

impl<R, D> From<collections::BTreeSet<PriceMetadata<R, D>>> for PriceRequest<R, D>
where
    R: resolution::TimeResolution,
    D: resolution::TimeResolution,
{
    fn from(map: collections::BTreeSet<PriceMetadata<R, D>>) -> PriceRequest<R, D> {
        PriceRequest { series: map }
    }
}

impl<R, D> From<collections::BTreeSet<(i64, i64)>> for PriceRequest<R, D>
where
    R: resolution::TimeResolution,
    D: resolution::TimeResolution,
{
    fn from(map: collections::BTreeSet<(i64, i64)>) -> PriceRequest<R, D> {
        PriceRequest {
            series: map.into_iter().map(PriceMetadata::from_tuple).collect(),
        }
    }
}

// forward prices should generally only be one price a day
// later we may want to consider intraday pricing??
pub type Prices<E, A, C, P> = collections::BTreeMap<PriceMetadata<E, P>, units::Price<A, C>>;

// pub fn prices_to_effective_map<P, A, Y>(
//     prices: &Prices<P, A, Y>,
// ) -> collections::BTreeMap<P, units::Price<A, Y>>
// where
//     P: resolution::TimeResolution + 'static,
//     A: units::Contract + 'static,
//     Y: units::Currency + 'static,
// {
//     prices.iter().map(|(md, pr)| (md.effective, *pr)).collect()
// }

pub trait DataProvider:
    PriceProvider + QuantityProvider + CalendarProvider + ObservationProvider
{
}

impl DataProvider for BlankProvider {}

#[async_trait::async_trait]
impl QuantityProvider for BlankProvider {
    async fn get_quantities<P, U>(
        &mut self,
        period: resolution::TimeRange<P>,
        contract: &str,
    ) -> crate::Result<collections::BTreeMap<P, units::Quantity<U>>>
    where
        P: resolution::TimeResolution + 'static,
        U: units::StaticDisplay + 'static,
    {
        todo!()
    }
}

#[async_trait::async_trait]
impl CalendarProvider for BlankProvider {
    async fn get_holidays<P: resolution::TimeResolution + 'static>(
        &mut self,
        period: resolution::TimeRange<P>,
        calendar: &str,
    ) -> crate::Result<collections::BTreeSet<chrono::NaiveDate>> {
        todo!()
    }
}

#[async_trait::async_trait]
impl ObservationProvider for BlankProvider {
    async fn get_observations<P, O>(
        &mut self,
        period: resolution::TimeRange<P>,
        location: &str,
    ) -> crate::Result<collections::BTreeMap<P, units::Observation<O>>>
    where
        P: resolution::TimeResolution + 'static,
        O: units::Observable + 'static,
    {
        todo!()
    }
}

#[async_trait::async_trait]
pub trait PriceProvider: Send {
    // spot price if the `P` falls on the Date.
    async fn get_prices<A, Y, E, P>(
        &mut self,
        request: &PriceRequest<E, P>,
        contract: &str,
    ) -> crate::Result<Prices<E, A, Y, P>>
    where
        E: resolution::TimeResolution + 'static,
        P: resolution::TimeResolution + 'static,
        A: units::Asset + 'static,
        Y: units::Asset + 'static;
}

type CacheKey = (
    any::TypeId, // the contract delivery resolution
    any::TypeId, // the contract delivery resolution
    any::TypeId, // the `from` asset
    any::TypeId, // the `to` asset\
    String,      // the contract string
);

struct PriceProviderCache<P1: PriceProvider> {
    provider: P1,
    // the A itself may be generic across Resolution, Scale or Unit, so
    // we must take their TypeId's as well
    cache: collections::HashMap<CacheKey, resolution::Cache<(i64, i64), rust_decimal::Decimal>>,
}

#[async_trait::async_trait]
impl<P1: PriceProvider> PriceProvider for PriceProviderCache<P1> {
    async fn get_prices<A, Y, E, P>(
        &mut self,
        request: &PriceRequest<E, P>,
        contract: &str,
    ) -> crate::Result<Prices<E, A, Y, P>>
    where
        E: resolution::TimeResolution + 'static,
        P: resolution::TimeResolution + 'static,
        A: units::Asset + 'static,
        Y: units::Asset + 'static,
    {
        // the A itself may be generic across Resolution, Scale or Unit, so
        // we must take their TypeId's as well
        let key = (
            any::TypeId::of::<P>(),
            any::TypeId::of::<E>(),
            any::TypeId::of::<A>(),
            any::TypeId::of::<Y>(),
            String::from(contract),
        );
        if let Some(cache) = self.cache.get_mut(&key) {
            match cache.get(request.request()) {
                resolution::CacheResponse::Miss(new_requests) => {
                    for range in new_requests {
                        let prices = self
                            .provider
                            .get_prices::<A, Y, E, P>(&range.clone().into(), contract)
                            .await?;
                        cache.add(
                            range,
                            prices
                                .iter()
                                .map(|(k, v)| (k.as_tuple(), v.get()))
                                .collect(),
                        );
                    }
                    self.get_prices(request, contract).await
                }
                resolution::CacheResponse::Hit(data) => Ok(data
                    .into_iter()
                    .map(|(k, v)| (PriceMetadata::from_tuple(k), units::Price::new(v)))
                    .collect()),
            }
        } else {
            let prices = self.provider.get_prices(request, contract).await?;
            let mut cache = resolution::Cache::empty();
            cache.add(
                request.request(),
                prices
                    .iter()
                    .map(|(k, v)| (k.as_tuple(), v.get()))
                    .collect(),
            );
            self.cache.insert(key, cache);
            // cache prices
            Ok(prices)
        }
    }
}

#[async_trait::async_trait]
impl<R: PriceProvider, L: PriceProvider> PriceProvider for (R, L) {
    async fn get_prices<A, Y, E, P>(
        &mut self,
        request: &PriceRequest<E, P>,
        contract: &str,
    ) -> crate::Result<Prices<E, A, Y, P>>
    where
        E: resolution::TimeResolution + 'static,
        P: resolution::TimeResolution + 'static,
        A: units::Asset + 'static,
        Y: units::Asset + 'static,
    {
        match self.0.get_prices(request, contract).await {
            Ok(p) => Ok(p),
            Err(_) => self.1.get_prices(request, contract).await,
        }
    }
}

// convenience impl such that this can be passed when no provider is needed
#[async_trait::async_trait]
impl PriceProvider for BlankProvider {
    async fn get_prices<A, Y, E, P>(
        &mut self,
        request: &PriceRequest<E, P>,
        contract: &str,
    ) -> crate::Result<Prices<E, A, Y, P>>
    where
        E: resolution::TimeResolution + 'static,
        P: resolution::TimeResolution + 'static,
        A: units::Asset + 'static,
        Y: units::Asset + 'static,
    {
        todo!()
    }
}

#[async_trait::async_trait]
pub trait QuantityProvider: Send {
    // should this take a string of some kind?
    // to allow the user to signal the contract that it is for?
    async fn get_quantities<P, U>(
        &mut self,
        period: resolution::TimeRange<P>,
        contract: &str,
    ) -> crate::Result<collections::BTreeMap<P, units::Quantity<U>>>
    where
        P: resolution::TimeResolution + 'static,
        U: units::StaticDisplay + 'static;
}

#[async_trait::async_trait]
pub trait ObservationProvider: Send {
    async fn get_observations<P, O>(
        &mut self,
        period: resolution::TimeRange<P>,
        location: &str,
    ) -> crate::Result<collections::BTreeMap<P, units::Observation<O>>>
    where
        P: resolution::TimeResolution + 'static,
        O: units::Observable + 'static;
}

// have a `Id` quantity provider that just gives back the contracts's qty?

//pub struct Holidays<P: resolution::TimeResolution> {
//    source_period: P,
//    dates: collections::BTreeSet<chrono::NaiveDate>,
//}

impl Holidays for collections::BTreeSet<chrono::NaiveDate> {
    fn is_holiday(
        &self,
        weekend_definition: &collections::HashSet<chrono::Weekday>,
        day: chrono::NaiveDate,
    ) -> bool {
        weekend_definition.contains(&day.weekday()) || self.contains(&day)
    }
}

pub trait Holidays {
    fn is_business_day(
        &self,
        weekend_definition: &collections::HashSet<chrono::Weekday>,
        day: chrono::NaiveDate,
    ) -> bool {
        !self.is_holiday(weekend_definition, day)
    }
    fn is_holiday(
        &self,
        weekend_definition: &collections::HashSet<chrono::Weekday>,
        day: chrono::NaiveDate,
    ) -> bool;
}

#[async_trait::async_trait]
pub trait CalendarProvider: Send {
    async fn get_holidays<P: resolution::TimeResolution + 'static>(
        &mut self,
        period: resolution::TimeRange<P>,
        calendar: &str,
    ) -> crate::Result<collections::BTreeSet<chrono::NaiveDate>>;
}
