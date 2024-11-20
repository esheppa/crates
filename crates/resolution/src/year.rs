use crate::{
    month::{self},
    quarter::{self, QuarterOfYear},
    DateResolution, DateResolutionExt, Day, FiveMinute, FromMonotonic, HalfHour, Hour, Month,
    Quarter, TimeResolution,
};
#[cfg(feature = "chrono")]
use chrono::{DateTime, Datelike, NaiveDate, NaiveTime, Utc};
use core::{convert::TryFrom, fmt, str};
use date_impl::{Date, MonthOfYear};

#[derive(Clone, Copy, Debug, Eq, PartialOrd, PartialEq, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct Year(i32);

impl crate::DateResolution for Year {
    fn start(self) -> Day {
        self.start()
    }
    type Params = ();

    fn params(self) -> Self::Params {}

    fn from_day(d: Day, _params: Self::Params) -> Self {
        Self::from_day(d)
    }
}

#[cfg(feature = "chrono")]
impl From<NaiveDate> for Year {
    fn from(value: NaiveDate) -> Year {
        Year::from_day(value)
    }
}

impl crate::TimeResolution for Year {
    fn succ_n(self, n: u16) -> Year {
        self.succ_n(n)
    }
    fn pred_n(self, n: u16) -> Year {
        self.pred_n(n)
    }
    #[cfg(feature = "chrono")]
    fn start_datetime(self) -> DateTime<Utc> {
        self.start().and_time(NaiveTime::MIN).and_utc()
    }

    fn start_minute(self) -> crate::Minute {
        todo!()
    }

    const NAME: &str = "Year";

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

    fn month(self) -> Month {
        todo!()
    }

    fn year(self) -> Year {
        todo!()
    }
}

impl crate::Monotonic for Year {
    fn to_monotonic(self) -> i32 {
        self.to_monotonic()
    }
    fn between(self, other: Self) -> i32 {
        self.between(other)
    }
}

impl crate::FromMonotonic for Year {
    fn from_monotonic(idx: i32) -> Self {
        Self::from_monotonic(idx)
    }
}

impl From<i32> for Year {
    fn from(value: i32) -> Self {
        Year::from_monotonic(value.into())
    }
}

impl From<i16> for Year {
    fn from(value: i16) -> Self {
        Year::from_monotonic(value.into())
    }
}

impl From<u16> for Year {
    fn from(value: u16) -> Self {
        Year::from_monotonic(value.into())
    }
}

impl Year {
    fn five_minute(self) -> FiveMinute {
        FiveMinute::first_on_day(self.day())
    }

    fn half_hour(self) -> HalfHour {
        HalfHour::first_on_day(self.day())
    }

    fn hour(self) -> Hour {
        Hour::first_on_day(self.day())
    }

    fn day(self) -> Day {
        self.start()
    }

    fn month(self) -> Month {
        self.first_month()
    }

    fn year(self) -> Year {
        self
    }

    pub const fn start(self) -> Day {
        Day::new(Date::first_on_year(self.0))
    }

    pub const fn from_monotonic(idx: i32) -> Self {
        Year(idx)
    }
    pub const fn to_monotonic(self) -> i32 {
        self.0
    }
    pub const fn between(self, other: Self) -> i32 {
        other.0 - self.0
    }
    pub const fn succ_n(self, n: u16) -> Year {
        Year(self.0 + n as i32)
    }
    pub const fn pred_n(self, n: u16) -> Year {
        Year(self.0 - n as i32)
    }
    pub const fn succ(self) -> Year {
        self.succ_n(1)
    }
    pub const fn pred(self) -> Year {
        self.pred_n(1)
    }
    pub const fn first_month(self) -> month::Month {
        self.jan()
    }
    pub const fn first_quarter(self) -> Quarter {
        self.q1()
    }
    pub const fn last_month(self) -> month::Month {
        self.dec()
    }
    pub const fn last_quarter(self) -> Quarter {
        self.q4()
    }
    pub const fn year_num(self) -> i32 {
        self.0
    }
    pub const fn from_day(day: Day) -> Year {
        Year(day.year_num())
    }
    pub const fn new(year: i32) -> Self {
        Year(year as i32)
    }
    pub const fn with_quarter(self, quarter: QuarterOfYear) -> Quarter {
        Quarter::from_parts(self, quarter)
    }
    pub const fn with_month(self, month: MonthOfYear) -> Month {
        self.jan().succ_n(month.months_from_jan() as u16)
    }

    pub const fn q1(self) -> Quarter {
        Quarter::from_parts(self, quarter::QuarterOfYear::Q1)
    }
    pub const fn q2(self) -> Quarter {
        Quarter::from_parts(self, quarter::QuarterOfYear::Q2)
    }
    pub const fn q3(self) -> Quarter {
        Quarter::from_parts(self, quarter::QuarterOfYear::Q3)
    }
    pub const fn q4(self) -> Quarter {
        Quarter::from_parts(self, quarter::QuarterOfYear::Q4)
    }
    pub const fn jan(self) -> Month {
        Month::from_year_month(self.year_num(), MonthOfYear::Jan)
    }
    pub const fn feb(self) -> Month {
        Month::from_year_month(self.year_num(), MonthOfYear::Feb)
    }
    pub const fn mar(self) -> Month {
        Month::from_year_month(self.year_num(), MonthOfYear::Mar)
    }
    pub const fn apr(self) -> Month {
        Month::from_year_month(self.year_num(), MonthOfYear::Apr)
    }
    pub const fn may(self) -> Month {
        Month::from_year_month(self.year_num(), MonthOfYear::May)
    }
    pub const fn jun(self) -> Month {
        Month::from_year_month(self.year_num(), MonthOfYear::Jun)
    }
    pub const fn jul(self) -> Month {
        Month::from_year_month(self.year_num(), MonthOfYear::Jul)
    }
    pub const fn aug(self) -> Month {
        Month::from_year_month(self.year_num(), MonthOfYear::Aug)
    }
    pub const fn sep(self) -> Month {
        Month::from_year_month(self.year_num(), MonthOfYear::Sep)
    }
    pub const fn oct(self) -> Month {
        Month::from_year_month(self.year_num(), MonthOfYear::Oct)
    }
    pub const fn nov(self) -> Month {
        Month::from_year_month(self.year_num(), MonthOfYear::Nov)
    }
    pub const fn dec(self) -> Month {
        Month::from_year_month(self.year_num(), MonthOfYear::Dec)
    }
}

impl crate::DateResolutionBuilder for Year {
    fn q1(self) -> Quarter {
        Year::q1(self)
    }
    fn q2(self) -> Quarter {
        Year::q2(self)
    }
    fn q3(self) -> Quarter {
        Year::q3(self)
    }
    fn q4(self) -> Quarter {
        Year::q4(self)
    }
    fn jan(self) -> Month {
        Year::jan(self)
    }
    fn feb(self) -> Month {
        Year::feb(self)
    }
    fn mar(self) -> Month {
        Year::mar(self)
    }
    fn apr(self) -> Month {
        Year::apr(self)
    }
    fn may(self) -> Month {
        Year::may(self)
    }
    fn jun(self) -> Month {
        Year::jun(self)
    }
    fn jul(self) -> Month {
        Year::jul(self)
    }
    fn aug(self) -> Month {
        Year::aug(self)
    }
    fn sep(self) -> Month {
        Year::sep(self)
    }
    fn oct(self) -> Month {
        Year::oct(self)
    }
    fn nov(self) -> Month {
        Year::nov(self)
    }
    fn dec(self) -> Month {
        Year::dec(self)
    }
}

impl fmt::Display for Year {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl str::FromStr for Year {
    type Err = crate::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Year(s.parse()?))
    }
}

#[cfg(test)]
mod tests {
    use date_impl::DayOfMonth;

    use super::*;
    use crate::{DateResolution, TimeResolution};

    #[test]
    #[cfg(feature = "serde")]
    fn test_roundtrip() {
        let dt = chrono::NaiveDate::from_ymd_opt(2021, 12, 6).unwrap();

        let yr = Year::from(dt);
        assert!(yr.start() <= dt && yr.end() >= dt);

        assert_eq!(
            yr,
            serde_json::from_str(&serde_json::to_string(&yr).unwrap()).unwrap()
        )
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            "2021".parse::<Year>().unwrap().start(),
            Day::new(Date::ymd(2021, MonthOfYear::Jan, DayOfMonth::D1)),
        );
        assert_eq!(
            "2021".parse::<Year>().unwrap().succ().start(),
            Day::new(Date::ymd(2022, MonthOfYear::Jan, DayOfMonth::D1)),
        );
        assert_eq!(
            "2021".parse::<Year>().unwrap().succ().pred().start(),
            Day::new(Date::ymd(2021, MonthOfYear::Jan, DayOfMonth::D1)),
        );

        assert!("a2021".parse::<Year>().is_err(),);
    }
}
