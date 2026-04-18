use chrono::Timelike;
use derivatives::{
    providers::{
        self, CalendarProvider, Holidays, ObservationProvider, PriceProvider, QuantityProvider,
    },
    reports,
    units::{self, currency},
};
use num_traits::Zero;
use resolution::{SubDateResolution, TimeResolution};
use std::collections;

fn main() {}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct DegreesCelsius;

impl units::Observable for DegreesCelsius {
    fn symbol() -> &'static str {
        "°C"
    }
    fn code() -> &'static str {
        "deg"
    }
    fn description() -> &'static str {
        "degrees celsius"
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Vic1Node();

impl units::StaticDisplay for Vic1Node {
    fn code() -> &'static str {
        "VIC1"
    }
    fn description() -> &'static str {
        "Victorian Electricity Node"
    }
}

impl units::Asset for Vic1Node {
    fn symbol() -> &'static str {
        "VIC1"
    }

    // units MWh
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Nsw1Node();

impl units::StaticDisplay for Nsw1Node {
    fn code() -> &'static str {
        "NSW1"
    }
    fn description() -> &'static str {
        "New South Wales Electricity Node"
    }
}

impl units::Asset for Nsw1Node {
    fn symbol() -> &'static str {
        "NSW1"
    }

    // units MWh
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Qld1Node();

impl units::StaticDisplay for Qld1Node {
    fn code() -> &'static str {
        "QLD1"
    }
    fn description() -> &'static str {
        "Queensland Electricity Node"
    }
}

impl units::Asset for Qld1Node {
    fn symbol() -> &'static str {
        "QLD1"
    }

    // units MWh
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Sa1Node();

impl units::StaticDisplay for Sa1Node {
    fn code() -> &'static str {
        "SA1"
    }
    fn description() -> &'static str {
        "South Australia Electricity Node"
    }
}

impl units::Asset for Sa1Node {
    fn symbol() -> &'static str {
        "SA1"
    }

    // units MWh
}

// #[derive(derivatives_macros::DataProvider)]
enum LocalProvider {
    Blank(providers::BlankProvider),
    Requirements(providers::RequirementsProvider),
}

impl providers::DataProvider for LocalProvider {}

#[async_trait::async_trait]
impl providers::PriceProvider for LocalProvider {
    async fn get_prices<A, Y, E, P>(
        &mut self,
        request: &providers::PriceRequest<E, P>,
        contract: &str,
    ) -> derivatives::Result<providers::Prices<E, A, Y, P>>
    where
        P: resolution::TimeResolution + 'static,
        E: resolution::TimeResolution + 'static,
        A: units::Asset + 'static,
        Y: units::Asset + 'static,
    {
        match self {
            LocalProvider::Blank(p) => p.get_prices(request, contract).await,
            LocalProvider::Requirements(p) => p.get_prices(request, contract).await,
        }
    }
}

#[async_trait::async_trait]
impl providers::QuantityProvider for LocalProvider {
    async fn get_quantities<P, U>(
        &mut self,
        period: resolution::TimeRange<P>,
        contract: &str,
    ) -> derivatives::Result<collections::BTreeMap<P, units::Quantity<U>>>
    where
        P: resolution::TimeResolution + 'static,
        U: units::StaticDisplay + 'static,
    {
        match self {
            LocalProvider::Blank(p) => p.get_quantities(period, contract).await,
            LocalProvider::Requirements(p) => p.get_quantities(period, contract).await,
        }
    }
}

#[async_trait::async_trait]
impl providers::CalendarProvider for LocalProvider {
    async fn get_holidays<P: resolution::TimeResolution + 'static>(
        &mut self,
        period: resolution::TimeRange<P>,
        calendar: &str,
    ) -> derivatives::Result<collections::BTreeSet<chrono::NaiveDate>> {
        match self {
            LocalProvider::Blank(p) => p.get_holidays(period, calendar).await,
            LocalProvider::Requirements(p) => p.get_holidays(period, calendar).await,
        }
    }
}

#[async_trait::async_trait]
impl providers::ObservationProvider for LocalProvider {
    async fn get_observations<P, O>(
        &mut self,
        period: resolution::TimeRange<P>,
        location: &str,
    ) -> derivatives::Result<collections::BTreeMap<P, units::Observation<O>>>
    where
        P: resolution::TimeResolution + 'static,
        O: units::Observable + 'static,
    {
        match self {
            LocalProvider::Blank(p) => p.get_observations(period, location).await,
            LocalProvider::Requirements(p) => p.get_observations(period, location).await,
        }
    }
}

// requirements
// Vic, Nsw, Qld, Sa
// -> holidays
// -> weather obs at daily
// -> quantities at 5m

enum Instrument<C>
where
    C: units::Asset + 'static,
{
    Base {
        strike: units::Price<C, currency::Aud>,
    },
    Peak {
        strike: units::Price<C, currency::Aud>,
        calendar: String,
    },
    Cap {
        purchase_price: units::Price<C, currency::Aud>,
        // don't need a valuation strike as the purchase price is used for valuation
        settlement_strike: units::Price<C, currency::Aud>,
    },
}

struct StandardContract<C>
where
    C: units::Asset + 'static,
{
    base: derivatives::OtcBase<resolution::FiveMinute>,
    contract: String,
    quantity: units::Quantity<C>,
    instrument: Instrument<C>,
}

#[async_trait::async_trait]
impl<C> reports::Settlement for StandardContract<C>
where
    C: units::Asset + 'static,
{
    type Currency = currency::Aud;
    type DataProvider = LocalProvider;
    fn settlement_date(&self) -> resolution::Day {
        self.base.settlement
    }
    async fn settle(
        &self,
        data_provider: &mut Self::DataProvider,
    ) -> Result<units::Quantity<currency::Aud>, derivatives::DynErr> {
        // get all prices across the delivery period
        let request = providers::PriceRequest::spot_range(self.base.delivery);
        let prices = data_provider
            // .get_prices::<_, _, C, _>(&request)
            .get_prices(&request, &self.contract)
            .await?;

        let amount = match &self.instrument {
            Instrument::Base { strike } => prices
                .into_iter()
                .map(|(_, p)| {
                    self.quantity
                        * match self.base.position {
                            derivatives::Position::Long => p - *strike,
                            derivatives::Position::Short => *strike - p,
                        }
                })
                .sum(),
            Instrument::Peak { strike, calendar } => {
                let holidays = data_provider
                    .get_holidays(self.base.delivery, calendar)
                    .await?;

                let weekend_definition = [chrono::Weekday::Sat, chrono::Weekday::Sun]
                    .iter()
                    .copied()
                    .collect();

                prices
                    .into_iter()
                    // remove all days that are holidays, keeping all business days
                    .filter(|(t, _)| {
                        holidays.is_business_day(&weekend_definition, t.delivery.occurs_on_date())
                    })
                    // remove all times not in the specified range
                    .filter(|(t, _)| {
                        t.delivery.start_datetime().time().hour() >= 7
                            && t.delivery.start_datetime().time().hour() <= 22
                    })
                    .map(|(_, p)| {
                        self.quantity
                            * match self.base.position {
                                derivatives::Position::Long => p - *strike,
                                derivatives::Position::Short => *strike - p,
                            }
                    })
                    .sum()
            }
            Instrument::Cap {
                settlement_strike, ..
            } => {
                prices
                    .into_iter()
                    // contract only triggered when the price is above the strike price
                    .filter(|(_, p)| p > settlement_strike)
                    .map(|(_, p)| {
                        self.quantity
                            * match self.base.position {
                                derivatives::Position::Long => p - *settlement_strike,
                                derivatives::Position::Short => *settlement_strike - p,
                            }
                    })
                    .sum()
            }
        };

        Ok(amount)
    }
}

struct WeatherCap<C, T>
where
    C: units::Asset + 'static,
    T: units::Observable + 'static,
{
    base: derivatives::OtcBase<resolution::FiveMinute>,
    quantity: units::Quantity<C>,
    contract: String,
    location: String,
    purchase_price: units::Price<C, currency::Aud>,
    settlement_strike: units::Price<C, currency::Aud>,
    temperature_trigger: units::Observation<T>,
}

#[async_trait::async_trait]
impl<C, T> reports::Settlement for WeatherCap<C, T>
where
    C: units::Asset + 'static,
    T: units::Observable + 'static,
{
    type Currency = currency::Aud;
    type DataProvider = LocalProvider;
    fn settlement_date(&self) -> resolution::Day {
        self.base.settlement
    }
    async fn settle(
        &self,
        data_provider: &mut Self::DataProvider,
    ) -> Result<units::Quantity<currency::Aud>, derivatives::DynErr> {
        let request = providers::PriceRequest::spot_range(self.base.delivery);
        // get all prices across the delivery period
        let prices = data_provider
            // .get_prices::<_, _, C, currency::Aud>(&request, &self.contract)
            .get_prices(&request, &self.contract)
            .await?;

        let date_range = resolution::TimeRange::from_start_end(
            self.base.delivery.start().occurs_on_date().into(),
            self.base.delivery.end().occurs_on_date().into(),
        )
        .unwrap();

        let temperatures = data_provider
            .get_observations::<resolution::Day, T>(date_range, &self.location)
            .await?;

        let mut amount = units::Quantity::zero();

        for (meta, price) in prices {
            let relevant_temperature = temperatures
                .get(&meta.delivery.occurs_on_date().into())
                .unwrap();

            if price > self.settlement_strike && relevant_temperature > &self.temperature_trigger {
                let value = self.quantity
                    * match self.base.position {
                        derivatives::Position::Long => price - self.settlement_strike,
                        derivatives::Position::Short => self.settlement_strike - price,
                    };

                amount += value;
            }
        }

        Ok(amount)
    }
}

struct LoadFollowingSwap<C>
where
    C: units::Asset + 'static,
{
    base: derivatives::OtcBase<resolution::FiveMinute>,
    strike: units::Price<C, currency::Aud>,
    contract: String,
    quantity_contract: String,
}

#[async_trait::async_trait]
impl<C> reports::Settlement for LoadFollowingSwap<C>
where
    C: units::Asset + 'static,
{
    type Currency = currency::Aud;
    type DataProvider = LocalProvider;
    fn settlement_date(&self) -> resolution::Day {
        self.base.settlement
    }
    async fn settle(
        &self,
        data_provider: &mut Self::DataProvider,
    ) -> Result<units::Quantity<currency::Aud>, derivatives::DynErr> {
        let request = providers::PriceRequest::spot_range(self.base.delivery);
        // get all prices across the delivery period
        // let prices = data_provider.get_prices::<_, _, C, _>(&request, &self.contract).await?;
        let prices = data_provider.get_prices(&request, &self.contract).await?;

        let quantities = data_provider
            // .get_quantities::<_, C>(self.base.delivery, &self.quantity_contract)
            .get_quantities(self.base.delivery, &self.quantity_contract)
            .await?;

        let mut amount = units::Quantity::zero();

        for (meta, price) in prices {
            let q = quantities.get(&meta.delivery).unwrap();

            let value = *q
                * match self.base.position {
                    derivatives::Position::Long => price - self.strike,
                    derivatives::Position::Short => self.strike - price,
                };

            amount += value;
        }

        Ok(amount)
    }
}

struct InterRegionSwap<F, T>
where
    F: units::Asset + 'static,
    T: units::Asset + 'static,
{
    base: derivatives::OtcBase<resolution::FiveMinute>,
    rate: units::Price<F, T>,
    from_contract: String,
    to_contract: String,
    quantity: units::Quantity<F>,
}

#[async_trait::async_trait]
impl<F, T> reports::Settlement for InterRegionSwap<F, T>
where
    F: units::Asset + 'static,
    T: units::Asset + 'static,
{
    type Currency = currency::Aud;
    type DataProvider = LocalProvider;
    fn settlement_date(&self) -> resolution::Day {
        self.base.settlement
    }
    async fn settle(
        &self,
        data_provider: &mut Self::DataProvider,
    ) -> Result<units::Quantity<currency::Aud>, derivatives::DynErr> {
        let request = providers::PriceRequest::spot_range(self.base.delivery);
        // get all prices across the delivery period
        // let prices = data_provider.get_prices::<_, _, C, _>(&request, &self.contract).await?;
        let from_prices = data_provider
            .get_prices::<F, currency::Aud, _, _>(&request, &self.from_contract)
            .await?;
        let to_prices = data_provider
            .get_prices::<T, currency::Aud, _, _>(&request, &self.to_contract)
            .await?;

        let mut amount = units::Quantity::zero();

        for (meta, from_price) in from_prices {
            let to_price = to_prices.get(&meta).unwrap();

            let dyn_price = from_price / *to_price;

            let value = self.quantity
                * match self.base.position {
                    derivatives::Position::Long => dyn_price - self.rate,
                    derivatives::Position::Short => self.rate - dyn_price,
                };

            amount += value * *to_price;
        }

        Ok(amount)
    }
}
