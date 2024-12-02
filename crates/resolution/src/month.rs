use crate::date_impl::{DayOfMonth, MonthOfYear};
use crate::{
    DateResolution, DateResolutionExt, Day, Error, FiveMinute, FromMonotonic, HalfHour, Hour,
    Minute, Monotonic, Quarter, TimeResolution, Year,
};
use alloc::{
    fmt, format, str,
    string::{String, ToString},
};
#[cfg(feature = "chrono")]
use chrono::{DateTime, Datelike, NaiveDate, NaiveTime, Utc};
use core::{convert::TryFrom, result};
#[cfg(feature = "serde")]
use serde::de;

#[cfg(feature = "serde")]
impl<'de> de::Deserialize<'de> for Month {
    fn deserialize<D>(deserializer: D) -> result::Result<Month, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let date = s.parse::<Month>().map_err(serde::de::Error::custom)?;
        Ok(date)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Month {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = self.to_string();
        serializer.serialize_str(&s)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Month(i32); // number of months +- since 0AD

impl TimeResolution for Month {
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

    const NAME: &str = "Month";

    fn start_minute(self) -> Minute {
        self.start_minute()
    }

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

impl Monotonic for Month {
    fn to_monotonic(self) -> i32 {
        self.to_monotonic()
    }
    fn between(self, other: Self) -> i32 {
        self.between(other)
    }
}

impl FromMonotonic for Month {
    fn from_monotonic(idx: i32) -> Self {
        Month::from_monotonic(idx)
    }
}

impl DateResolution for Month {
    fn start(self) -> Day {
        self.start()
    }

    type Params = ();

    fn params(self) -> Self::Params {}

    fn from_day(d: Day, _params: Self::Params) -> Self {
        Month::from_day(d)
    }
}

#[cfg(feature = "chrono")]
impl From<NaiveDate> for Month {
    fn from(value: NaiveDate) -> Month {
        Month::from_day(value, ())
    }
}

#[cfg(feature = "chrono")]
impl From<DateTime<Utc>> for Month {
    fn from(d: DateTime<Utc>) -> Self {
        d.date_naive().into()
    }
}

impl Month {
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
        todo!()
    }

    pub const fn month(self) -> Month {
        todo!()
    }

    pub const fn succ_n(self, n: u16) -> Self {
        Month(self.0 + n as i32)
    }
    pub const fn pred_n(self, n: u16) -> Self {
        Month(self.0 - n as i32)
    }
    pub const fn succ(self) -> Self {
        self.succ_n(1)
    }
    pub const fn pred(self) -> Self {
        self.pred_n(1)
    }
    pub const fn first_day(self) -> Day {
        self.start()
    }
    pub const fn last_day(self) -> Day {
        self.succ().start().pred()
    }
    pub const fn with_day(self, day: DayOfMonth) -> Day {
        self.start().succ_n(day.offset() as u16)
    }
    pub const fn from_year_month(y: i32, month: MonthOfYear) -> Self {
        Month((month.number() as i32 - 1) + y.saturating_mul(12))
    }
    pub const fn year(self) -> Year {
        Year::from_day(self.start())
    }
    pub const fn quarter(self) -> Quarter {
        Quarter::from_day(self.start())
    }
    pub const fn year_num(self) -> i32 {
        self.start().year_num()
    }
    pub const fn month_num(self) -> u8 {
        self.start().month_of_year().number()
    }
    pub const fn month_of_year(self) -> MonthOfYear {
        match self.month_num() {
            1 => MonthOfYear::Jan,
            2 => MonthOfYear::Feb,
            3 => MonthOfYear::Mar,
            4 => MonthOfYear::Apr,
            5 => MonthOfYear::May,
            6 => MonthOfYear::Jun,
            7 => MonthOfYear::Jul,
            8 => MonthOfYear::Aug,
            9 => MonthOfYear::Sep,
            10 => MonthOfYear::Oct,
            11 => MonthOfYear::Nov,
            12 => MonthOfYear::Dec,
            _ => unreachable!(),
        }
    }
    pub const fn from_day(day: Day) -> Self {
        Self::from_parts(day.year(), day.month_of_year())
    }
    pub const fn from_parts(year: Year, month: MonthOfYear) -> Self {
        Self::from_monotonic(year.to_monotonic() * 12 + month.months_from_jan() as i32)
    }

    pub const fn start(self) -> Day {
        self.with_day(DayOfMonth::D1)
    }

    pub const fn to_monotonic(self) -> i32 {
        self.0
    }
    pub const fn between(self, other: Self) -> i32 {
        other.0 - self.0
    }

    pub const fn from_monotonic(idx: i32) -> Self {
        Month(idx)
    }
}

impl str::FromStr for Month {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once('-') {
            Some((year, month)) => match (
                year.parse::<Year>(),
                month.parse::<u8>().ok().and_then(MonthOfYear::from_number),
            ) {
                (Ok(year), Some(month)) => Ok(Month::from_parts(year, month)),
                _ => Err(Error::ParseCustom {
                    ty_name: "Month",
                    input: s.to_string(),
                }),
            },
            None => s.parse::<Day>().map(|d| Month::from_day(d)),
        }
    }
}

impl fmt::Display for Month {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{:02}", self.year(), self.month().month_num())
    }
}

#[cfg(test)]
mod tests {
    use crate::date_impl::{DayOfMonth, MonthOfYear};

    use super::Month;
    use crate::{DateResolution, Day, TimeResolution};

    #[test]
    #[cfg(feature = "serde")]
    fn test_roundtrip() {
        use DateResolutionExt;

        let dt = chrono::NaiveDate::from_ymd_opt(2021, 12, 6).unwrap();

        let m1 = Month::from(dt);
        assert!(m1.start() <= dt && m1.end() >= dt);

        let dt = chrono::NaiveDate::from_ymd_opt(2019, 7, 1).unwrap();

        let m2 = Month::from(dt);

        assert!(m2.start() == dt);

        assert_eq!(
            m1,
            serde_json::from_str(&serde_json::to_string(&m1).unwrap()).unwrap()
        )
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            "Jan-2021".parse::<Month>().unwrap().start(),
            Day::ymd(2021, MonthOfYear::Jan, DayOfMonth::D1)
        );
        assert_eq!(
            "Jan-2021".parse::<Month>().unwrap().succ().start(),
            Day::ymd(2021, MonthOfYear::Feb, DayOfMonth::D1)
        );
        assert_eq!(
            "Jan-2021".parse::<Month>().unwrap().succ().pred().start(),
            Day::ymd(2021, MonthOfYear::Jan, DayOfMonth::D1)
        );
    }

    #[test]
    fn test_start() {
        assert_eq!(
            Month(24240).start(),
            Day::ymd(2020, MonthOfYear::Jan, DayOfMonth::D1)
        );
        assert_eq!(
            Month(24249).start(),
            Day::ymd(2020, MonthOfYear::Oct, DayOfMonth::D1)
        );
        assert_eq!(
            Month(15).start(),
            Day::ymd(1, MonthOfYear::Apr, DayOfMonth::D1)
        );
        assert_eq!(
            Month(2).start(),
            Day::ymd(0, MonthOfYear::Mar, DayOfMonth::D1)
        );
        assert_eq!(
            Month(1).start(),
            Day::ymd(0, MonthOfYear::Feb, DayOfMonth::D1)
        );
        assert_eq!(
            Month(0).start(),
            Day::ymd(0, MonthOfYear::Jan, DayOfMonth::D1)
        );
        assert_eq!(
            Month(-1).start(),
            Day::ymd(-1, MonthOfYear::Dec, DayOfMonth::D1)
        );
        assert_eq!(
            Month(-2).start(),
            Day::ymd(-1, MonthOfYear::Nov, DayOfMonth::D1)
        );
        assert_eq!(
            Month(-15).start(),
            Day::ymd(-2, MonthOfYear::Oct, DayOfMonth::D1)
        );
    }
}
