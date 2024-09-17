use crate::units::{Asset, Observable, Observation, Price, Quantity, StaticDisplay};

use serde::{
    de,
    ser::{self, SerializeStruct},
};

#[derive(serde::Serialize, serde::Deserialize)]
struct _Price {
    from: String,
    to: String,
    value: rust_decimal::Decimal,
    // #[serde(bound(deserialize = "P: resolution::TimeResolution"))]
    // delivery: resolution::TimeRange<P>,
}

impl<'de, A, Y> serde::Deserialize<'de> for Price<A, Y>
where
    Y: Asset,
    A: Asset,
{
    fn deserialize<D>(deserializer: D) -> Result<Price<A, Y>, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let raw_price = _Price::deserialize(deserializer)?;
        if A::code() == raw_price.from && Y::code() == raw_price.to {
            Ok(Price::new(raw_price.value))
        } else {
            Err(de::Error::custom(format!(
                "Expected from-asset {from} but got {raw_from}, expected to-asset {to} but got {raw_to}.",
                from = A::code(),
                raw_from = raw_price.from,
                to = Y::code(),
                raw_to = raw_price.to,
                )))
        }
    }
}

impl<A, Y> serde::Serialize for Price<A, Y>
where
    Y: Asset,
    A: Asset,
{
    fn serialize<SER>(&self, serializer: SER) -> Result<SER::Ok, SER::Error>
    where
        SER: ser::Serializer,
    {
        let mut state = serializer.serialize_struct("SpotPrice", 3)?;
        state.serialize_field("from", A::code())?;
        state.serialize_field("to", Y::code())?;
        state.serialize_field("value", &self.value)?;
        state.end()
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
struct _Observation {
    observable: String,
    value: rust_decimal::Decimal,
}

impl<'de, O> serde::Deserialize<'de> for Observation<O>
where
    O: Observable,
{
    fn deserialize<D>(deserializer: D) -> Result<Observation<O>, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let raw_obs = _Observation::deserialize(deserializer)?;
        if O::code() == raw_obs.observable {
            Ok(Observation::new(raw_obs.value))
        } else {
            Err(de::Error::custom(format!(
                "Expected observation {observable} but got {raw_observable}.",
                observable = O::code(),
                raw_observable = raw_obs.observable,
            )))
        }
    }
}

impl<O> serde::Serialize for Observation<O>
where
    O: Observable,
{
    fn serialize<SER>(&self, serializer: SER) -> Result<SER::Ok, SER::Error>
    where
        SER: ser::Serializer,
    {
        let mut state = serializer.serialize_struct("Observation", 2)?;
        state.serialize_field("observable", O::code())?;
        state.serialize_field("value", &self.value)?;
        state.end()
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
struct _Quantity {
    unit: String,
    value: rust_decimal::Decimal,
}

impl<'de, U> serde::Deserialize<'de> for Quantity<U>
where
    U: StaticDisplay,
{
    fn deserialize<D>(deserializer: D) -> Result<Quantity<U>, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let raw_quantity = _Quantity::deserialize(deserializer)?;
        if U::code() == raw_quantity.unit {
            Ok(Quantity::new(raw_quantity.value))
        } else {
            Err(de::Error::custom(format!(
                "Expected unit {unit} but got {raw_unit}",
                unit = U::code(),
                raw_unit = raw_quantity.unit,
            )))
        }
    }
}

impl<U> serde::Serialize for Quantity<U>
where
    U: StaticDisplay,
{
    fn serialize<SER>(&self, serializer: SER) -> Result<SER::Ok, SER::Error>
    where
        SER: ser::Serializer,
    {
        let mut state = serializer.serialize_struct("Quantity", 2)?;
        state.serialize_field("unit", U::code())?;
        state.serialize_field("value", &self.value)?;
        state.end()
    }
}
