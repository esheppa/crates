use date::Date;

use crate::{minutes::MINUTES_PER_DAY, prelude::*, *};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FinancialYear(i64);

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for FinancialYear {
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        s.parse().map_err(serde::de::Error::custom)
    }
}

#[cfg(feature = "serde")]
impl Serialize for FinancialYear {
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

const MIN: i64 = 1;
const MAX: i64 = 9999; // TODO

impl FinancialYear {
    pub const MIN: Self = Self(MIN);
    pub const MAX: Self = Self(MAX);
    pub const fn from_monotonic(idx: i64) -> Option<Self> {
        // TODO: use MIN..=MAX here when it is const
        if idx >= MIN && idx <= MAX {
            Some(Self(idx))
        } else {
            None
        }
    }
    pub const fn to_monotonic(self) -> i64 {
        self.0
    }

    pub const fn between(self, other: Self) -> i64 {
        other.0 - self.0
    }
    pub const fn translate(self, n: i64) -> Option<Self> {
        let Some(new) = self.0.checked_add(n) else {
            return None;
        };
        Self::from_monotonic(new)
    }
    pub const fn start_minute(self) -> Minute {
        self.start_day().start_minute()
    }

    pub const fn end_minute(self) -> Minute {
        self.end_day().end_minute()
    }

    pub const fn start_day(self) -> Day {
        Day::from_date(
            Date::first_on_month(
                date::Year::new(self.to_monotonic() as i32 - 1),
                MonthOfYear::Jul,
            )
            .expect("Always valid"),
        )
        .expect("Always valid")
    }

    pub const fn end_day(self) -> Day {
        Day::from_date(
            Date::last_on_month(
                date::Year::new(self.to_monotonic() as i32),
                MonthOfYear::Jun,
            )
            .expect("Always valid"),
        )
        .expect("Always valid")
    }

    pub const fn from_day(day: Day) -> Option<Self> {
        let date = day.date();
        let year = Year::from_monotonic(date.year().num() as i64).expect("TODO");

        // we know year is at least 0.
        // must not have a month earlier than 7
        // day is valid by construction of `date`
        let (y, m, _) = date.to_ymd();
        if y <= 0 && m < 7 {
            return None;
        }

        match date.month_of_year() {
            MonthOfYear::Jan
            | MonthOfYear::Feb
            | MonthOfYear::Mar
            | MonthOfYear::Apr
            | MonthOfYear::May
            | MonthOfYear::Jun => Some(FinancialYear(year.to_monotonic())),
            MonthOfYear::Jul
            | MonthOfYear::Aug
            | MonthOfYear::Sep
            | MonthOfYear::Oct
            | MonthOfYear::Nov
            | MonthOfYear::Dec => Some({
                let Some(y) = year.translate(1) else {
                    return None;
                };

                FinancialYear(y.to_monotonic())
            }),
        }
    }
}

impl TimeResolution for FinancialYear {
    const NAME: &str = "FinancialYear";

    fn translate(self, n: i64) -> Option<Self> {
        self.translate(n)
    }

    fn start_minute(self) -> Minute {
        self.start_minute()
    }

    fn end_minute(self) -> Minute {
        self.end_minute()
    }
}

impl DateResolution for FinancialYear {
    type Params = ();

    type FromDay = Option<Self>;

    fn params(self) -> Self::Params {
        ()
    }

    fn from_day(day: Day, _params: Self::Params) -> Self::FromDay {
        FinancialYear::from_day(day)
    }

    fn start_day(self) -> Day {
        self.start_day()
    }
    fn end_day(self) -> Day {
        self.end_day()
    }
}

impl Monotonic for FinancialYear {
    fn to_monotonic(self) -> i64 {
        self.to_monotonic()
    }

    fn between(self, other: Self) -> i64 {
        self.between(other)
    }
}

impl FromMonotonic for FinancialYear {
    fn from_monotonic(idx: i64) -> Option<Self> {
        Self::from_monotonic(idx)
    }
}

impl fmt::Display for FinancialYear {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FY{:04}", self.0)
    }
}

impl str::FromStr for FinancialYear {
    type Err = Error;
    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        let get_err = || {
            Err(Error::ParseCustom {
                ty_name: "FinancialYear",
                input: s.to_string(),
            })
        };
        let Some((_, y)) = s.split_once("FY") else {
            return get_err();
        };

        match FinancialYear::from_monotonic(y.parse()?) {
            Some(y) => Ok(y),
            None => get_err(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DateResolutionExt;

    #[test]
    fn exhaustive() {
        extern crate std;
        for i in MIN..=MAX {
            let y = FinancialYear::from_monotonic(i).unwrap();
            assert_eq!(FinancialYear::MIN.translate(i - MIN).unwrap(), y);
            std::dbg!(
                y.start_day().date().to_ymd(),
                y.end_day().date().to_ymd(),
                y,
                i
            );
            assert_eq!(FinancialYear::from_day(y.start_day()), Some(y));
            assert_eq!(FinancialYear::from_day(y.end_day()), Some(y));
            _ = y.start_minute();
            _ = y.start_day();
            _ = y.start_p::<Month>();
            _ = y.start_p::<Day>();
            _ = y.end_minute();
            _ = y.end_day();
            _ = y.end_p::<Month>();
            _ = y.end_p::<Day>();
        }
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_roundtrip() {
        for i in MIN..=MAX {
            let y = FinancialYear::from_monotonic(i).unwrap();
            let ser = serde_json::to_string(&y).unwrap();
            assert_eq!(serde_json::from_str::<FinancialYear>(&ser).unwrap(), y);
        }
    }

    #[test]
    fn test_parse_fmt() {
        for x in MIN..=MAX {
            let y = FinancialYear::from_monotonic(x).unwrap();
            assert_eq!(format!("FY{x:04}").parse::<FinancialYear>().unwrap(), y,);

            assert_eq!(y.to_string().parse::<FinancialYear>().unwrap(), y);
        }
    }
}
