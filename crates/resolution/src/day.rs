use crate::{DateResolution, Month, TimeResolution};
use alloc::{
    fmt, str,
    string::{String, ToString},
};
use chrono::{DateTime, Datelike, NaiveDate, NaiveTime, Utc};
use core::result;
#[cfg(feature = "serde")]
use serde::de;

const DATE_FORMAT: &str = "%Y-%m-%d";

#[cfg(feature = "serde")]
impl<'de> de::Deserialize<'de> for Day {
    fn deserialize<D>(deserializer: D) -> result::Result<Day, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let date =
            chrono::NaiveDate::parse_from_str(&s, DATE_FORMAT).map_err(serde::de::Error::custom)?;
        Ok(date.into())
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Day {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = self.to_string();
        serializer.serialize_str(&s)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Day(i64);

pub enum DayOfMonth {
    D1,
    D2,
    D3,
    D4,
    D5,
    D6,
    D7,
    D8,
    D9,
    D10,
    D11,
    D12,
    D13,
    D14,
    D15,
    D16,
    D17,
    D18,
    D19,
    D20,
    D21,
    D22,
    D23,
    D24,
    D25,
    D26,
    D27,
    D28,
}

impl DayOfMonth {
    pub fn number(&self) -> u32 {
        match self {
            DayOfMonth::D1 => 1,
            DayOfMonth::D2 => 2,
            DayOfMonth::D3 => 3,
            DayOfMonth::D4 => 4,
            DayOfMonth::D5 => 5,
            DayOfMonth::D6 => 6,
            DayOfMonth::D7 => 7,
            DayOfMonth::D8 => 8,
            DayOfMonth::D9 => 9,
            DayOfMonth::D10 => 10,
            DayOfMonth::D11 => 11,
            DayOfMonth::D12 => 12,
            DayOfMonth::D13 => 13,
            DayOfMonth::D14 => 14,
            DayOfMonth::D15 => 15,
            DayOfMonth::D16 => 16,
            DayOfMonth::D17 => 17,
            DayOfMonth::D18 => 18,
            DayOfMonth::D19 => 19,
            DayOfMonth::D20 => 20,
            DayOfMonth::D21 => 21,
            DayOfMonth::D22 => 22,
            DayOfMonth::D23 => 23,
            DayOfMonth::D24 => 24,
            DayOfMonth::D25 => 25,
            DayOfMonth::D26 => 26,
            DayOfMonth::D27 => 27,
            DayOfMonth::D28 => 28,
        }
    }
}

fn base() -> chrono::NaiveDate {
    chrono::NaiveDate::from_ymd_opt(0, 1, 1).expect("valid date")
}

impl str::FromStr for Day {
    type Err = crate::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let date = chrono::NaiveDate::parse_from_str(s, DATE_FORMAT)?;
        Ok(date.into())
    }
}

impl fmt::Display for Day {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.start())
    }
}

impl crate::DateResolution for Day {
    fn start(&self) -> chrono::NaiveDate {
        base() + chrono::Duration::days(self.0)
    }

    type Params = ();

    fn params(&self) -> Self::Params {}

    fn from_date(date: NaiveDate, _params: Self::Params) -> Self {
        Day((date - base()).num_days())
    }
}

// impl From<DateTime<Utc>> for Day {
//     fn from(d: DateTime<Utc>) -> Self {
//         d.date_naive().into()
//     }
// }

impl<D: Datelike> From<D> for Day {
    fn from(value: D) -> Day {
        Day::from_date(
            chrono::NaiveDate::from_ymd_opt(value.year(), value.month(), value.day()).unwrap(),
            (),
        )
    }
}

impl crate::TimeResolution for Day {
    fn succ_n(&self, n: u64) -> Day {
        Day(self.0 + i64::try_from(n).unwrap())
    }
    fn pred_n(&self, n: u64) -> Day {
        Day(self.0 - i64::try_from(n).unwrap())
    }
    fn start_datetime(&self) -> DateTime<Utc> {
        self.start().and_time(NaiveTime::MIN).and_utc()
    }
    fn name(&self) -> String {
        "Day".to_string()
    }
}

impl crate::Monotonic for Day {
    fn to_monotonic(&self) -> i64 {
        self.0
    }
    fn between(&self, other: Self) -> i64 {
        other.0 - self.0
    }
}

impl crate::FromMonotonic for Day {
    fn from_monotonic(idx: i64) -> Self {
        Day(idx)
    }
}

impl Day {
    pub fn with_day(self, d: DayOfMonth) -> Day {
        self.month().first_day().pred().succ_n(d.number().into())
    }
    pub fn year(&self) -> super::Year {
        self.start().into()
    }
    pub fn quarter(&self) -> super::Quarter {
        self.start().into()
    }
    pub fn month(&self) -> super::Month {
        self.start().into()
    }
    pub fn week<D: super::StartDay>(&self) -> super::Week<D> {
        self.start().into()
    }
    pub fn year_num(&self) -> i32 {
        self.start().year()
    }
    pub fn month_num(&self) -> u32 {
        self.start().month()
    }
    pub fn new(date: NaiveDate) -> Self {
        date.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DateResolution, TimeResolution};

    #[cfg(feature = "serde")]
    #[test]
    fn test_roundtrip() {
        use crate::DateResolutionExt;

        let dt = chrono::NaiveDate::from_ymd_opt(2021, 12, 6).unwrap();

        let wk = Day::from(dt);
        assert!(wk.start() <= dt && wk.end() >= dt);

        assert_eq!(
            wk,
            serde_json::from_str(&serde_json::to_string(&wk).unwrap()).unwrap()
        )
    }

    #[test]
    fn test_parse_date_syntax() {
        assert_eq!(
            "2021-01-01".parse::<Day>().unwrap().start(),
            chrono::NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
        );
        assert_eq!(
            "2021-01-01".parse::<Day>().unwrap().succ().start(),
            chrono::NaiveDate::from_ymd_opt(2021, 1, 2).unwrap(),
        );
        assert_eq!(
            "2021-01-01".parse::<Day>().unwrap().succ().pred().start(),
            chrono::NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
        );
    }

    #[test]
    fn test_start() {
        assert_eq!(
            Day(2).start(),
            chrono::NaiveDate::from_ymd_opt(0, 1, 3).unwrap()
        );
        assert_eq!(
            Day(1).start(),
            chrono::NaiveDate::from_ymd_opt(0, 1, 2).unwrap()
        );
        assert_eq!(
            Day(0).start(),
            chrono::NaiveDate::from_ymd_opt(0, 1, 1).unwrap()
        );
        assert_eq!(
            Day(-1).start(),
            chrono::NaiveDate::from_ymd_opt(-1, 12, 31).unwrap()
        );
        assert_eq!(
            Day(-2).start(),
            chrono::NaiveDate::from_ymd_opt(-1, 12, 30).unwrap()
        );
    }
}
