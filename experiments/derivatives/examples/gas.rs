use derivatives::units;

fn main() {}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct VicGas();

impl units::StaticDisplay for VicGas {
    fn code() -> &'static str {
        "VicGas"
    }
    fn description() -> &'static str {
        "Victorian Electricity Node"
    }
}

impl units::Asset for VicGas {
    fn symbol() -> &'static str {
        "VicGas"
    }
}
