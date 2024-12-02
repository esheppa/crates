use crate::{Error, FiveMinute, HalfHour, Hour, Minute};

use crate::{
    DateResolution, FromMonotonic, Monotonic, Month, Quarter, StartDay, TimeResolution, Week, Year,
};
use alloc::{fmt, str};
#[cfg(feature = "chrono")]
use chrono::{DateTime, Datelike, NaiveDate, NaiveTime, Utc};
use date_impl::{Day, DayOfMonth, MonthOfYear};
#[cfg(feature = "serde")]
use serde::de;

pub mod date_impl;

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

impl str::FromStr for Day {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()

        // let date = chrono::NaiveDate::parse_from_str(s, DATE_FORMAT)?;
        // Ok(date.into())
    }
}

impl fmt::Display for Day {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (year, month, day) = self.to_ymd();
        write!(f, "{year:04}-{month:02}-{day:02}")
    }
}

impl DateResolution for Day {
    fn start(self) -> Day {
        self
    }
    type Params = ();

    fn params(self) -> Self::Params {}

    fn from_day(d: Day, _params: Self::Params) -> Self {
        d
    }
}
#[cfg(feature = "chrono")]
impl<D: chrono::Datelike> From<D> for Day {
    fn from(value: D) -> Day {
        Day::new(
            chrono::NaiveDate::from_ymd_opt(
                chrono::Datelike::year(&value),
                chrono::Datelike::month(&value),
                chrono::Datelike::day(&value),
            )
            .unwrap(),
        )
    }
}

impl TimeResolution for Day {
    fn succ_n(self, n: u16) -> Self {
        self.succ_n(n)
    }
    fn pred_n(self, n: u16) -> Self {
        self.pred_n(n)
    }
    #[cfg(feature = "chrono")]
    fn start_datetime(self) -> DateTime<Utc> {
        self.start().and_time(NaiveTime::MIN).and_utc()
    }

    fn start_minute(self) -> Minute {
        self.start_minute()
    }

    const NAME: &str = "Year";
    fn day(self) -> Day {
        self.day()
    }

    fn month(self) -> Month {
        self.month()
    }

    fn year(self) -> Year {
        self.year()
    }
}

impl Monotonic for Day {
    fn to_monotonic(self) -> i32 {
        self.to_monotonic()
    }
    fn between(self, other: Self) -> i32 {
        self.between(other)
    }
}

impl FromMonotonic for Day {
    fn from_monotonic(idx: i32) -> Self {
        Self::from_monotonic(idx)
    }
}
impl Day {
    pub const fn from_monotonic(idx: i32) -> Self {
        Day(idx)
    }
    pub const fn to_monotonic(self) -> i32 {
        self.0
    }
    pub const fn between(self, other: Self) -> i32 {
        other.0 - self.0
    }

    // pub const fn start_datetime(self) -> DateTime<Utc> {
    //     self.date().and_time(NaiveTime::MIN).and_utc()
    // }

    pub const fn year(self) -> super::Year {
        super::Year::new(self.year_num())
    }
    pub const fn quarter(self) -> super::Quarter {
        Quarter::from_day(self)
    }

    pub const fn week<D: StartDay>(self) -> Week<D> {
        Week::from_day(self)
    }

    pub const fn month_num(self) -> u8 {
        self.month_of_year().number()
    }

    pub const fn start_minute(self) -> Minute {
        todo!()
    }

    pub const fn five_minute(self) -> FiveMinute {
        todo!()
    }

    pub const fn half_hour(self) -> HalfHour {
        todo!()
    }

    pub const fn hour(self) -> Hour {
        todo!()
    }

    pub const fn day(self) -> Day {
        self
    }

    pub const fn month(self) -> Month {
        Month::from_day(self)
    }
}

// date impl
// i16 for year
// date stored as i32
// year + type for 1970..2070 (optimized section)
// others -> manual calc
// Result<DayOfMonth, [29,30,31]> -> allows infallibe D.O.M methods, even at runtime.

#[cfg(test)]
mod tests {

    use super::*;
    use TimeResolution;

    #[cfg(feature = "serde")]
    #[test]
    fn test_roundtrip() {
        use DateResolutionExt;

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
            "2021-01-01".parse::<Day>().unwrap(),
            Day::first_on_year(2021),
        );
        assert_eq!(
            "2021-01-01".parse::<Day>().unwrap().succ(),
            Day::ymd(2021, MonthOfYear::Jan, DayOfMonth::D2),
        );
        assert_eq!(
            "2021-01-01".parse::<Day>().unwrap().succ().pred(),
            Day::first_on_year(2021),
        );
    }

    #[test]
    fn test_start() {
        assert_eq!(Day(2), Day::ymd(0, MonthOfYear::Jan, DayOfMonth::D3));
        assert_eq!(Day(1), Day::ymd(0, MonthOfYear::Jan, DayOfMonth::D2));
        assert_eq!(Day(0), Day::ymd(0, MonthOfYear::Jan, DayOfMonth::D1));
        assert_eq!(Day(-1), Day::last_on_month(-1, MonthOfYear::Dec));
        assert_eq!(Day(-2), Day::last_on_month(-1, MonthOfYear::Dec).pred_n(1));
    }
}
