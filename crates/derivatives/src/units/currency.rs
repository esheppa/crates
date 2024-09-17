// eventually codegen all common currency codes from a config source
use crate::units;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Aud;

impl units::Asset for Aud {
    fn symbol() -> &'static str {
        "$"
    }
}

impl units::StaticDisplay for Aud {
    fn code() -> &'static str {
        "AUD"
    }
    fn description() -> &'static str {
        "Australian Dollar"
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Usd;

impl units::Asset for Usd {
    fn symbol() -> &'static str {
        "$"
    }
}

impl units::StaticDisplay for Usd {
    fn code() -> &'static str {
        "USD"
    }
    fn description() -> &'static str {
        "United States Dollar"
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Jpy;

impl units::StaticDisplay for Jpy {
    fn code() -> &'static str {
        "JPY"
    }
    fn description() -> &'static str {
        "Japanese Yen"
    }
}

impl units::Asset for Jpy {
    fn symbol() -> &'static str {
        "円"
    }
}
