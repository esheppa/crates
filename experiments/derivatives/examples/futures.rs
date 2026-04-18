use derivatives::{reports, units};
use resolution::DateResolution;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct BrentFuture;

impl units::StaticDisplay for BrentFuture {
    fn code() -> &'static str {
        "QA"
    }
    fn description() -> &'static str {
        "Brent Barrels"
    }
}

impl units::Asset for BrentFuture {
    fn symbol() -> &'static str {
        "QA"
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Vic1Node;

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
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct VicBase;

impl units::StaticDisplay for VicBase {
    fn code() -> &'static str {
        "BV"
    }
    fn description() -> &'static str {
        "Victorian Base Load Electricity"
    }
}

impl units::Asset for VicBase {
    fn symbol() -> &'static str {
        "BV"
    }
}

struct Future<Delivery, Deriv, Undl, Currency>
where
    Delivery: DateResolution + 'static,
    Deriv: units::Asset + 'static,
    Currency: units::Asset + 'static,
    Undl: units::Asset + 'static,
{
    base: derivatives::FuturesBase<Delivery>,
    quantity: units::Quantity<Deriv>,
    valuation_strike: units::Price<Deriv, Currency>,
    settlement_strike: units::Price<Undl, Currency>,
}

fn main() {}
