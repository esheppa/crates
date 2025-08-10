use date::Date;

use crate::{minutes::MINUTES_PER_DAY, prelude::*, *};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FinancialYear(i64);

impl<'de> Deserialize<'de> for FinancialYear {
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        s.parse().map_err(serde::de::Error::custom)
    }
}

impl Serialize for FinancialYear {
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

const MIN: i64 = -23639;
const MAX: i64 = 96347; // TODO

impl FinancialYear {
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
        Some(Self(new))
    }
    pub const fn start_minute(self) -> Minute {
        Minutes::<1>::from_monotonic(self.0 * MINUTES_PER_DAY).expect("")
    }

    pub const fn end_minute(self) -> Minute {
        Minutes::<1>::from_monotonic(self.0 * MINUTES_PER_DAY + MINUTES_PER_DAY).expect("")
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

    fn from_day(_day: Day, _params: Self::Params) -> Self::FromDay {
        todo!()
    }

    fn start_day(self) -> Day {
        Day::from_date(
            Date::first_on_month(
                date::Year::new(self.to_monotonic() as i32).unwrap(),
                MonthOfYear::Jul,
            )
            .expect("Always valid"),
        )
    }
    fn end_day(self) -> Day {
        Day::from_date(
            Date::last_on_month(
                date::Year::new(self.to_monotonic() as i32).unwrap(),
                MonthOfYear::Jun,
            )
            .expect("Always valid"),
        )
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
    use date::DayOfMonth;

    use super::*;
    use crate::DateResolution;
    use crate::DateResolutionExt;

    // #[test]
    // #[cfg(feature = "serde")]
    // fn test_roundtrip() {
    //     let dt = chrono::NaiveDate::from_ymd_opt(2021, 12, 6).unwrap();

    //     let yr = Year::from_day(Day::from_chrono_date(dt));
    //     assert!(yr.start_day().chrono_date() <= dt && yr.end_day().chrono_date() >= dt);

    //     assert_eq!(
    //         yr,
    //         serde_json::from_str(&serde_json::to_string(&yr).unwrap()).unwrap()
    //     )
    // }

    // #[test]
    // fn test_parse() {
    //     assert_eq!(
    //         "2021".parse::<Year>().unwrap().start(),
    //         Day::ymd(2021, MonthOfYear::Jan, DayOfMonth::D1),
    //     );
    //     assert_eq!(
    //         "2021".parse::<Year>().unwrap().succ().start(),
    //         Day::ymd(2022, MonthOfYear::Jan, DayOfMonth::D1),
    //     );
    //     assert_eq!(
    //         "2021".parse::<Year>().unwrap().succ().pred().start(),
    //         Day::ymd(2021, MonthOfYear::Jan, DayOfMonth::D1),
    //     );

    //     assert!("a2021".parse::<Year>().is_err(),);
    // }
}
