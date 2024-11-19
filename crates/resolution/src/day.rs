use crate::{DateResolution, FromMonotonic, Monotonic, Month, Quarter, TimeResolution};
use alloc::{fmt, str};
#[cfg(feature = "chrono")]
use chrono::{DateTime, Datelike, NaiveDate, NaiveTime, Utc};
use date_impl::{Date, DayOfMonth};
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
pub struct Day(i32);

impl str::FromStr for Day {
    type Err = crate::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()

        // let date = chrono::NaiveDate::parse_from_str(s, DATE_FORMAT)?;
        // Ok(date.into())
    }
}

impl fmt::Display for Day {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (year, month, day) = self.date().to_ymd();
        write!(f, "{year:04}-{month:02}-{day:02}")
    }
}

impl DateResolution for Day {
    fn start(&self) -> Day {
        *self
    }

    type Params = ();

    fn params(&self) -> Self::Params {}

    fn from_day(date: Day, _params: Self::Params) -> Self {
        date
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
    fn succ_n(self, n: u16) -> Day {
        self.succ_n(n)
    }
    fn pred_n(self, n: u16) -> Day {
        self.pred_n(n)
    }
    #[cfg(feature = "chrono")]
    fn start_datetime(self) -> DateTime<Utc> {
        self.start_datetime()
    }
    const NAME: &str = "Day";

    fn start_minute(self) -> crate::Minute {
        todo!()
    }

    fn five_minute(self) -> crate::FiveMinute {
        todo!()
    }

    fn half_hour(self) -> crate::HalfHour {
        todo!()
    }

    fn hour(self) -> crate::Hour {
        todo!()
    }

    fn day(self) -> Day {
        todo!()
    }

    fn month(self) -> crate::Month {
        todo!()
    }

    fn year(self) -> crate::Year {
        todo!()
    }
}

impl Monotonic for Day {
    fn to_monotonic(self) -> i32 {
        self.0
    }
    fn between(self, other: Self) -> i32 {
        other.0 - self.0
    }
}

impl FromMonotonic for Day {
    fn from_monotonic(idx: i32) -> Self {
        Day(idx)
    }
}

impl Day {
    pub const fn date(self) -> Date {
        Date::new(self.0)
    }
    pub const fn new(date: Date) -> Self {
        Day(date.inner())
    }

    pub const fn succ_n(self, n: u16) -> Day {
        Day(self.0 + n as i32)
    }
    pub const fn pred_n(self, n: u16) -> Day {
        Day(self.0 - n as i32)
    }

    pub const fn succ(self) -> Day {
        self.succ_n(1)
    }
    pub const fn pred(self) -> Day {
        self.pred_n(1)
    }
    // pub const fn start_datetime(self) -> DateTime<Utc> {
    //     self.date().and_time(NaiveTime::MIN).and_utc()
    // }

    pub const fn with_day(self, d: DayOfMonth) -> Day {
        Day(self.date().with_day(d).inner())
    }
    pub const fn year(self) -> super::Year {
        super::Year::new(self.date().year())
    }
    pub const fn quarter(self) -> super::Quarter {
        Quarter::from_day(*self, ())
    }

    pub const fn week<D: super::StartDay>(self) -> super::Week<D> {
        self.date().into()
    }
    pub const fn year_num(self) -> i32 {
        self.date().year()
    }
    pub const fn month_num(self) -> u8 {
        self.date().month()
    }

    pub const fn start_minute(self) -> crate::Minute {
        todo!()
    }

    pub const fn five_minute(self) -> crate::FiveMinute {
        todo!()
    }

    pub const fn half_hour(self) -> crate::HalfHour {
        todo!()
    }

    pub const fn hour(self) -> crate::Hour {
        todo!()
    }

    pub const fn day(self) -> Day {
        todo!()
    }

    pub const fn month(self) -> crate::Month {
        Month::from_day(self)
    }

    pub const fn year(self) -> crate::Year {
        todo!()
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
    use date_impl::MonthOfYear;

    use super::*;
    use crate::TimeResolution;

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
            "2021-01-01".parse::<Day>().unwrap().date(),
            Date::first_on_year(2021),
        );
        assert_eq!(
            "2021-01-01".parse::<Day>().unwrap().succ().date(),
            Date::ymd(2021, MonthOfYear::Jan, DayOfMonth::D2),
        );
        assert_eq!(
            "2021-01-01".parse::<Day>().unwrap().succ().pred().date(),
            Date::first_on_year(2021),
        );
    }

    #[test]
    fn test_start() {
        assert_eq!(
            Day(2).date(),
            Date::ymd(0, MonthOfYear::Jan, DayOfMonth::D3)
        );
        assert_eq!(
            Day(1).date(),
            Date::ymd(0, MonthOfYear::Jan, DayOfMonth::D2)
        );
        assert_eq!(
            Day(0).date(),
            Date::ymd(0, MonthOfYear::Jan, DayOfMonth::D1)
        );
        assert_eq!(Day(-1).date(), Date::last_on_month(-1, MonthOfYear::Dec));
        assert_eq!(
            Day(-2).date(),
            Date::last_on_month(-1, MonthOfYear::Dec).pred_n(1)
        );
    }
}
