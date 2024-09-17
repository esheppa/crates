use derivatives::{
    providers, reports,
    units::{self, currency},
};

use resolution::DateResolution;
use std::marker;

fn main() {}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Brent;

impl units::StaticDisplay for Brent {
    fn code() -> &'static str {
        "BC"
    }
    fn description() -> &'static str {
        "Brent Crude"
    }
}

impl units::Asset for Brent {
    fn symbol() -> &'static str {
        "BC"
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct WmReuters10amFix;

impl units::StaticDisplay for WmReuters10amFix {
    fn code() -> &'static str {
        "WMFX10AU"
    }
    fn description() -> &'static str {
        "WM Retuers 10am Fix"
    }
}

impl units::Asset for WmReuters10amFix {
    fn symbol() -> &'static str {
        "AUDUSD"
    }
}

struct MixedCurrencyTrade<Asset, Delivery, AssetCurrency, LocalCurrency>
where
    Asset: units::Asset + 'static,
    Delivery: DateResolution + 'static,
    AssetCurrency: units::Asset + 'static,
    LocalCurrency: units::Asset + 'static,
{
    asset: Asset,
    delivery: Delivery,
    position: derivatives::Position,
    entity: String,
    counterparty: String,
    calendar: String,
    traded: chrono::NaiveDate,     // date traded
    settlement: chrono::NaiveDate, // date to be settled
    asset_currency: marker::PhantomData<AssetCurrency>,
    local_currency: marker::PhantomData<LocalCurrency>,
}

// struct AudUsdCurrencySwap<A>
// where
//     A: units::Contract,
// {
//     trade: MixedCurrencyTrade<A, currency::Usd, currency::Aud>,
//     strike_price: units::Price<A, currency::Aud>,
//     quantity: units::Quantity<ScaleOf<A>, UnitOf<A>>,
// }

// #[async_trait::async_trait]
// impl<A> reports::Settlement for AudUsdCurrencySwap<A>
// where
//     A: units::Contract + 'static,
//     DeliveryOf<A>: resolution::DateResolution,
// {
//     type Currency = currency::Aud; // output is in local currency
//     fn settlement_date(&self) -> chrono::NaiveDate {
//         self.trade.settlement
//     }
//     async fn settle<P>(
//         &self,
//         data_provider: &mut P,
//     ) -> Result<units::Amount<Self::Currency>, derivatives::DynErr>
//     where
//         P: providers::DataProvider,
//     {
//         // calculate working days in the calculation month
//         // later replace with .as_date_range() or .rescale()
//         let calculation_period = resolution::TimeRange::from_start_end(
//             resolution::Date::from(self.trade.calculation.start()),
//             resolution::Date::from(self.trade.calculation.end()),
//         )
//         .unwrap();
//         let holidays = data_provider
//             .get_holidays(calculation_period, &self.trade.calendar)
//             .await?;

//         let price_request = calculation_period
//             .iter()
//             .filter(|d| !holidays.contains(&d.start()))
//             .fold(providers::PriceRequest::empty(), |mut acc, d| {
//                 acc.add(providers::PriceMetadata {
//                     effective: resolution::Date::from(d.start()),
//                     delivery: self.trade.delivery,
//                 });
//                 acc
//             });
//         // get the daily prices for the target product across the delivery period
//         let prices = data_provider
//             .get_prices::<_, A, currency::Usd>(&price_request)
//             .await?;

//         let currency_request = calculation_period
//             .iter()
//             .filter(|d| !holidays.contains(&d.start()))
//             .fold(providers::PriceRequest::empty(), |mut acc, d| {
//                 acc.add(providers::PriceMetadata {
//                     effective: resolution::Date::from(d.start()),
//                     delivery: resolution::Date::from(d.start()),
//                 });
//                 acc
//             });

//         // get the daily prices for the currency conversion across the delivery period
//         let exchange = data_provider
//             .get_currencies::<_, _, _, WmReuters10amFix>(&currency_request)
//             .await?;
//         // average the product prices and exchange rates then convert to local currency
//         let avg_price = {
//             let avg = prices
//                 .iter()
//                 .map(|(_, p)| p.get())
//                 .sum::<rust_decimal::Decimal>()
//                 / rust_decimal::Decimal::from(prices.len());
//             units::Price::<A, currency::Usd>::new(avg)
//         };
//         let avg_forex = {
//             let avg = exchange
//                 .iter()
//                 .map(|(_, p)| p.get())
//                 .sum::<rust_decimal::Decimal>()
//                 / rust_decimal::Decimal::from(exchange.len());
//             units::Forex::<_, _, WmReuters10amFix>::new(avg)
//         };

//         // let local = units::Price::<A, currency::Aud>::new(avg_price.get() * avg_forex.get());
//         let local = avg_price * avg_forex;

//         match self.trade.position {
//             derivatives::Position::Long => Ok((local - self.strike_price) * self.quantity),
//             derivatives::Position::Short => Ok((self.strike_price - local) * self.quantity),
//         }
//     }
// }

// struct MixedCurrencyCap<C>
// where
//     C: units::Asset,
// {
//     trade: MixedCurrencyTrade<C>,
//     strike_price: rust_decimal::Decimal,
//     quantity: units::Quantity<scale::Id, C>,
// }

// struct MixedCurrencyFloor<C>
// where
//     C: units::Asset,
// {
//     trade: MixedCurrencyTrade<C>,
//     strike_price: rust_decimal::Decimal,
//     quantity: units::Quantity<scale::Id, C>,
// }

// struct MixedCurrencyCollar<C>
// where
//     C: units::Asset,
// {
//     trade: MixedCurrencyTrade<C>,
//     call_strike: rust_decimal::Decimal,
//     put_strike: rust_decimal::Decimal,
//     quantity: units::Quantity<scale::Id, C>,
// }
