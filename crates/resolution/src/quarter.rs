use crate::{month, year, DateResolutionExt, Day, Year};
use alloc::{
    fmt, str,
    string::{String, ToString},
    vec::Vec,
};
#[cfg(feature = "chrono")]
use chrono::{DateTime, Datelike, NaiveDate, NaiveTime, Utc};
use core::convert::TryFrom;
use date_impl::MonthOfYear;
#[cfg(feature = "serde")]
use serde::de;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Quarter(i32);

impl crate::TimeResolution for Quarter {
    fn succ_n(&self, n: u16) -> Self {
        self.succ_n(n)
    }
    fn pred_n(&self, n: u16) -> Self {
        self.pred_n(n)
    }
    #[cfg(feature = "chrono")]
    fn start_datetime(&self) -> DateTime<Utc> {
        self.start_datetime()
    }

    const NAME: &str = "Quarter";

    fn start_minute(&self) -> crate::Minute {
        self.start_minute()
    }

    fn five_minute(self) -> crate::FiveMinute {
        self.five_minute()
    }

    fn half_hour(self) -> crate::HalfHour {
        self.half_hour()
    }

    fn hour(self) -> crate::Hour {
        self.hour()
    }

    fn day(self) -> Day {
        self.day()
    }

    fn month(self) -> crate::Month {
        self.month()
    }

    fn year(self) -> Year {
        self.year()
    }
}

impl crate::Monotonic for Quarter {
    fn to_monotonic(self) -> i32 {
        self.to_monotonic()
    }
    fn between(self, other: Self) -> i32 {
        self.between(other)
    }
}

impl crate::FromMonotonic for Quarter {
    fn from_monotonic(idx: i32) -> Self {
        Self::from_monotonic(idx)
    }
}

impl crate::DateResolution for Quarter {
    fn start(&self) -> Day {
        self.start()
    }

    type Params = ();

    fn params(&self) -> Self::Params {}

    fn from_day(d: Day, _params: Self::Params) -> Self {
        Quarter::from_day(d)
    }
}
#[cfg(feature = "chrono")]
impl From<NaiveDate> for Quarter {
    fn from(value: NaiveDate) -> Quarter {
        Quarter::from_day(value)
    }
}

impl Quarter {
    pub const fn start_minute(&self) -> crate::Minute {
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
        todo!()
    }

    pub const fn succ(self) -> Quarter {
        self.succ_n(1)
    }
    pub const fn pred(self) -> Quarter {
        self.pred_n(1)
    }
    pub const fn succ_n(self, n: u16) -> Self {
        Quarter(self.0 + n as i32)
    }
    pub const fn pred_n(self, n: u16) -> Self {
        Quarter(self.0 - n as i32)
    }
    #[cfg(feature = "chrono")]
    pub const fn start_datetime(self) -> DateTime<Utc> {
        self.start().date().and_time(NaiveTime::MIN).and_utc()
    }
    pub const fn to_monotonic(self) -> i32 {
        self.0
    }
    pub const fn between(self, other: Self) -> i32 {
        other.0 - self.0
    }
    pub const fn from_monotonic(idx: i32) -> Self {
        Quarter(idx)
    }
    pub const fn start(self) -> Day {
        self.first_month().start()
    }
    pub const fn from_day(d: Day) -> Self {
        match d.month().month_of_year() {
            MonthOfYear::Jan | MonthOfYear::Feb | MonthOfYear::Mar => {
                Self::from_parts(d.year(), QuarterOfYear::Q1)
            }
            MonthOfYear::Apr | MonthOfYear::May | MonthOfYear::Jun => {
                Self::from_parts(d.year(), QuarterOfYear::Q2)
            }
            MonthOfYear::Jul | MonthOfYear::Aug | MonthOfYear::Sep => {
                Self::from_parts(d.year(), QuarterOfYear::Q3)
            }
            MonthOfYear::Oct | MonthOfYear::Nov | MonthOfYear::Dec => {
                Self::from_parts(d.year(), QuarterOfYear::Q4)
            }
        }
    }

    pub const fn first_month(self) -> month::Month {
        match self.quarter_of_year() {
            QuarterOfYear::Q1 => self.year().jan(),
            QuarterOfYear::Q2 => self.year().apr(),
            QuarterOfYear::Q3 => self.year().jul(),
            QuarterOfYear::Q4 => self.year().oct(),
        }
    }
    pub const fn last_month(self) -> month::Month {
        match self.quarter_of_year() {
            QuarterOfYear::Q1 => self.year().mar(),
            QuarterOfYear::Q2 => self.year().jun(),
            QuarterOfYear::Q3 => self.year().sep(),
            QuarterOfYear::Q4 => self.year().dec(),
        }
    }
    pub const fn year(self) -> year::Year {
        Year::new(self.year_num())
    }
    pub const fn year_num(self) -> i32 {
        self.0.div_euclid(4)
    }
    pub const fn quarter_num(self) -> u8 {
        (self.0.rem_euclid(4) + 1) as u8
    }
    pub const fn quarter_of_year(self) -> QuarterOfYear {
        match self.quarter_num() {
            1 => QuarterOfYear::Q1,
            2 => QuarterOfYear::Q2,
            3 => QuarterOfYear::Q3,
            4 => QuarterOfYear::Q4,
            _ => panic!("Unexpected quarter number"),
        }
    }
    pub const fn from_parts(year: Year, quarter: QuarterOfYear) -> Self {
        Self::from_monotonic(year.to_monotonic() * 4 + quarter.offset())
    }
}

pub enum QuarterOfYear {
    Q1,
    Q2,
    Q3,
    Q4,
}

impl QuarterOfYear {
    const fn offset(&self) -> i32 {
        match self {
            QuarterOfYear::Q1 => 0,
            QuarterOfYear::Q2 => 1,
            QuarterOfYear::Q3 => 2,
            QuarterOfYear::Q4 => 3,
        }
    }
}

impl fmt::Display for Quarter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Q{}-{}", self.quarter_num(), self.year_num())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DateResolution, TimeResolution};

    #[test]
    #[cfg(feature = "serde")]
    fn test_roundtrip() {
        let dt = chrono::NaiveDate::from_ymd_opt(2021, 12, 6).unwrap();

        let wk = Quarter::from(dt);
        assert!(wk.start() <= dt && wk.end() >= dt);

        assert_eq!(
            wk,
            serde_json::from_str(&serde_json::to_string(&wk).unwrap()).unwrap()
        )
    }
    #[test]
    fn test_parse_quarter_syntax() {
        assert_eq!(
            "Q1-2021".parse::<Quarter>().unwrap().start(),
            chrono::NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
        );
        assert_eq!(
            "Q1-2021".parse::<Quarter>().unwrap().succ().start(),
            chrono::NaiveDate::from_ymd_opt(2021, 4, 1).unwrap(),
        );
        assert_eq!(
            "Q1-2021".parse::<Quarter>().unwrap().succ().pred().start(),
            chrono::NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
        );
    }

    #[test]
    fn test_parse_date_syntax() {
        assert_eq!(
            "2021-01-01".parse::<Quarter>().unwrap().start(),
            chrono::NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
        );
        assert_eq!(
            "2021-01-01".parse::<Quarter>().unwrap().succ().start(),
            chrono::NaiveDate::from_ymd_opt(2021, 4, 1).unwrap(),
        );
        assert_eq!(
            "2021-01-01"
                .parse::<Quarter>()
                .unwrap()
                .succ()
                .pred()
                .start(),
            chrono::NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
        );
    }

    #[test]
    fn test_start() {
        assert_eq!(
            Quarter(2).start(),
            chrono::NaiveDate::from_ymd_opt(0, 7, 1).unwrap()
        );
        assert_eq!(
            Quarter(1).start(),
            chrono::NaiveDate::from_ymd_opt(0, 4, 1).unwrap()
        );
        assert_eq!(
            Quarter(0).start(),
            chrono::NaiveDate::from_ymd_opt(0, 1, 1).unwrap()
        );
        assert_eq!(
            Quarter(-1).start(),
            chrono::NaiveDate::from_ymd_opt(-1, 10, 1).unwrap()
        );
        assert_eq!(
            Quarter(-2).start(),
            chrono::NaiveDate::from_ymd_opt(-1, 7, 1).unwrap()
        );
    }
}

#[cfg(feature = "serde")]
impl<'de> de::Deserialize<'de> for Quarter {
    fn deserialize<D>(deserializer: D) -> result::Result<Quarter, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let date = s.parse::<Quarter>().map_err(serde::de::Error::custom)?;
        Ok(date)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Quarter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = self.to_string();
        serializer.serialize_str(&s)
    }
}

impl str::FromStr for Quarter {
    type Err = crate::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(parsed) = chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d") {
            Ok(parsed.into())
        } else {
            let split = s
                .split('-')
                .map(ToString::to_string)
                .collect::<Vec<String>>();
            if split.len() == 2 {
                let qtr = split[0]
                    .chars()
                    .nth(1)
                    .unwrap()
                    .to_string()
                    .parse::<u32>()?;
                let year = split[1].parse()?;
                let date =
                    chrono::NaiveDate::from_ymd_opt(year, qtr * 3 - 2, 1).expect("valid date");
                Ok(date.into())
            } else {
                Err(crate::Error::ParseCustom {
                    ty_name: "Quarter",
                    input: s.to_string(),
                })
            }
        }
    }
}
