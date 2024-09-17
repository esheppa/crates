use crate::{providers, units};
// DataProvider must be an associated type to allow the trait to be object safe;
// this is acceptable however it requires a slightly different design than if it had a type paramter.

// The idea behind data providers is that differnet T: DataProvider can be used for example
// one for actual prices mark to market, another for simulation, etc.
// instead what must be done is that at the initialization stage of the data provider,
// the users would select whehter it will be simulation / actual / etc.
// But at the top level, each user of the library should have just one data provider type,
// which internally can delegate to other data providers.

// makes sense for Otc, Futures and other kinds of contracts
#[async_trait::async_trait]
pub trait Settlement {
    type Currency: units::Asset;
    type DataProvider: providers::DataProvider;
    fn settlement_date(&self) -> resolution::Day;
    async fn settle(
        &self,
        data_provider: &mut Self::DataProvider,
    ) -> Result<units::Quantity<Self::Currency>, crate::DynErr>;
}

// makes sense for Otc, Futures and other kinds of contracts
// #[async_trait::async_trait]
// pub trait Settlement {
//     type Currency: units::Asset;
//     fn settlement_date(&self) -> resolution::Day;
//     async fn settle<P>(
//         &self,
//         data_provider: &mut P,
//     ) -> Result<units::Quantity<Self::Currency>, crate::DynErr>
//     // should this be (Date, Amount) or even Map<Date, Amount> ?
//     where
//         P: providers::DataProvider;
// }

// makes sense for Otc
#[async_trait::async_trait]
pub trait Valuation {
    type Currency: units::Asset;
    type DataProvider: providers::DataProvider;
    async fn value(
        &self,
        data_provider: &mut Self::DataProvider,
        effective: impl Into<resolution::Day>,
    ) -> Result<units::Quantity<Self::Currency>, crate::DynErr>;
}

// makes sense for Futures
#[async_trait::async_trait]
trait VariationMargin {}

// makes sense for Futures
#[async_trait::async_trait]
trait InitialMargin {}
