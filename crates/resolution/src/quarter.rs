use date::Date;

use crate::{Year, minutes::MINUTES_PER_DAY, *};
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Quarter(i64);

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Quarter {
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        s.parse().map_err(serde::de::Error::custom)
    }
}

#[cfg(feature = "serde")]
impl Serialize for Quarter {
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[derive(Clone, Copy, Debug)]
pub enum QuarterOfYear {
    Q1,
    Q2,
    Q3,
    Q4,
}

impl QuarterOfYear {
    const fn from_month(month_of_year: MonthOfYear) -> Self {
        match month_of_year {
            MonthOfYear::Jan | MonthOfYear::Feb | MonthOfYear::Mar => QuarterOfYear::Q1,
            MonthOfYear::Apr | MonthOfYear::May | MonthOfYear::Jun => QuarterOfYear::Q2,
            MonthOfYear::Jul | MonthOfYear::Aug | MonthOfYear::Sep => QuarterOfYear::Q3,
            MonthOfYear::Oct | MonthOfYear::Nov | MonthOfYear::Dec => QuarterOfYear::Q4,
        }
    }
    const fn number(self) -> u8 {
        match self {
            QuarterOfYear::Q1 => 1,
            QuarterOfYear::Q2 => 2,
            QuarterOfYear::Q3 => 3,
            QuarterOfYear::Q4 => 4,
        }
    }
    const fn start_month(self) -> MonthOfYear {
        match self {
            QuarterOfYear::Q1 => MonthOfYear::Jan,
            QuarterOfYear::Q2 => MonthOfYear::Apr,
            QuarterOfYear::Q3 => MonthOfYear::Jul,
            QuarterOfYear::Q4 => MonthOfYear::Oct,
        }
    }
    const fn end_month(self) -> MonthOfYear {
        match self {
            QuarterOfYear::Q1 => MonthOfYear::Mar,
            QuarterOfYear::Q2 => MonthOfYear::Jun,
            QuarterOfYear::Q3 => MonthOfYear::Sep,
            QuarterOfYear::Q4 => MonthOfYear::Dec,
        }
    }
    const fn offset(&self) -> i32 {
        match self {
            QuarterOfYear::Q1 => 0,
            QuarterOfYear::Q2 => 1,
            QuarterOfYear::Q3 => 2,
            QuarterOfYear::Q4 => 3,
        }
    }
}

const MIN: i64 = -7880;
const MAX: i64 = 32116; // TODO

impl Quarter {
       pub const MIN: Self = Self(MIN);
    pub const MAX: Self = Self(MAX);
    pub const fn new(year: Year, q: QuarterOfYear) -> Self {
        Self(year.to_monotonic() * 4 + q.number() as i64)
    }
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
        Minutes::<1>::from_monotonic(self.0 * MINUTES_PER_DAY).expect("")
    }

    pub const fn end_minute(self) -> Minute {
        Minutes::<1>::from_monotonic(self.0 * MINUTES_PER_DAY + MINUTES_PER_DAY).expect("")
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

    pub const fn quarter_of_year(self) -> QuarterOfYear {
        match self.0 % 4 + 1 {
            1 => QuarterOfYear::Q1,
            2 => QuarterOfYear::Q2,
            3 => QuarterOfYear::Q3,
            4 => QuarterOfYear::Q4,
            _ => panic!("Can't get here, (X % 4 + 1) is always within 1..=4"),
        }
    }
    pub const fn year(self) -> Year {
        Year::from_monotonic(self.0 / 4).expect("Always valid as year is longer")
    }

    // pub const fn year_num(self) -> i32 {
    //     self.0.div_euclid(4)
    // }
    // pub const fn quarter_num(self) -> u8 {
    //     (self.0.rem_euclid(4) + 1) as u8
    // }
    // pub const fn from_day(d: Day) -> Self {
    //     match d.month().month_of_year() {
    //         MonthOfYear::Jan | MonthOfYear::Feb | MonthOfYear::Mar => {
    //             Self::from_parts(d.year(), QuarterOfYear::Q1)
    //         }
    //         MonthOfYear::Apr | MonthOfYear::May | MonthOfYear::Jun => {
    //             Self::from_parts(d.year(), QuarterOfYear::Q2)
    //         }
    //         MonthOfYear::Jul | MonthOfYear::Aug | MonthOfYear::Sep => {
    //             Self::from_parts(d.year(), QuarterOfYear::Q3)
    //         }
    //         MonthOfYear::Oct | MonthOfYear::Nov | MonthOfYear::Dec => {
    //             Self::from_parts(d.year(), QuarterOfYear::Q4)
    //         }
    //     }
    // }
}

impl TimeResolution for Quarter {
    const NAME: &str = "Quarter";

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

impl DateResolution for Quarter {
    type Params = ();

    type FromDay = Self;

    fn params(self) -> Self::Params {
        ()
    }

    fn from_day(day: Day, _params: Self::Params) -> Self::FromDay {
        let date = day.date();
        Self::new(
            Year::from_monotonic(date.year().num() as i64).expect("TODO"),
            QuarterOfYear::from_month(date.month_of_year()),
        )
    }

    fn start_day(self) -> Day {
        Day::from_date(
            Date::first_on_month(
                date::Year::new(self.year().to_monotonic() as i32).unwrap(),
                self.quarter_of_year().start_month(),
            )
            .expect("Always valid"),
        )
    }

    fn end_day(self) -> Day {
        Day::from_date(
            Date::last_on_month(
                date::Year::new(self.year().to_monotonic() as i32).unwrap(),
                self.quarter_of_year().end_month(),
            )
            .expect("Always valid"),
        )
    }
}

impl Monotonic for Quarter {
    fn to_monotonic(self) -> i64 {
        self.to_monotonic()
    }

    fn between(self, other: Self) -> i64 {
        self.between(other)
    }
}

impl FromMonotonic for Quarter {
    fn from_monotonic(idx: i64) -> Option<Self> {
        Self::from_monotonic(idx)
    }
}

impl str::FromStr for QuarterOfYear {
    type Err = crate::Error;
    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        match s {
            "Q1" | "q1" => Ok(Self::Q1),
            "Q2" | "q2" => Ok(Self::Q2),
            "Q3" | "q3" => Ok(Self::Q3),
            "Q4" | "q4" => Ok(Self::Q4),
            _ => Err(crate::Error::ParseCustom {
                ty_name: "QuarterOfYear",
                input: s.to_string(),
            }),
        }
    }
}

impl fmt::Display for Quarter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-Q{}", self.year(), self.quarter_of_year().number())
    }
}

impl str::FromStr for Quarter {
    type Err = crate::Error;
    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        match s.split_once('-') {
            Some((year, quarter)) => {
                match (year.parse::<Year>(), quarter.parse::<QuarterOfYear>()) {
                    (Ok(year), Ok(quarter)) => Ok(Quarter::new(year, quarter)),
                    _ => Err(crate::Error::ParseCustom {
                        ty_name: "Quarter",
                        input: s.to_string(),
                    }),
                }
            }
            None => s.parse::<Day>().map(|d| Quarter::from_day(d, ())),
        }
    }
}

#[cfg(test)]
mod tests {
    use date::{DayOfMonth, MonthOfYear};

    use super::*;

    // #[test]
    // #[cfg(feature = "serde")]
    // fn test_roundtrip() {
    //     use crate::{DateResolution, DateResolutionExt};
    //     let dt = chrono::NaiveDate::from_ymd_opt(2021, 12, 6).unwrap();

    //     let wk = Quarter::from_day(Day::from_chrono_date(dt));
    //     assert!(wk.start_day().chrono_date() <= dt && wk.end_day().chrono_date() >= dt);

    //     assert_eq!(
    //         wk,
    //         serde_json::from_str(&serde_json::to_string(&wk).unwrap()).unwrap()
    //     )
    // }
    // #[test]
    // fn test_parse_quarter_syntax() {
    //     assert_eq!(
    //         "Q1-2021".parse::<Quarter>().unwrap().start(),
    //         Day::ymd(2021, MonthOfYear::Jan, DayOfMonth::D1),
    //     );
    //     assert_eq!(
    //         "Q1-2021".parse::<Quarter>().unwrap().succ().start(),
    //         Day::ymd(2021, MonthOfYear::Apr, DayOfMonth::D1),
    //     );
    //     assert_eq!(
    //         "Q1-2021".parse::<Quarter>().unwrap().succ().pred().start(),
    //         Day::ymd(2021, MonthOfYear::Jan, DayOfMonth::D1),
    //     );
    // }

    // #[test]
    // fn test_parse_date_syntax() {
    //     assert_eq!(
    //         "2021-01-01".parse::<Quarter>().unwrap().start(),
    //         Day::ymd(2021, MonthOfYear::Jan, DayOfMonth::D1),
    //     );
    //     assert_eq!(
    //         "2021-01-01".parse::<Quarter>().unwrap().succ().start(),
    //         Day::ymd(2021, MonthOfYear::Apr, DayOfMonth::D1),
    //     );
    //     assert_eq!(
    //         "2021-01-01"
    //             .parse::<Quarter>()
    //             .unwrap()
    //             .succ()
    //             .pred()
    //             .start(),
    //         Day::ymd(2021, MonthOfYear::Jan, DayOfMonth::D1),
    //     );
    // }

    // #[test]
    // fn test_start() {
    //     assert_eq!(
    //         Quarter(2).start(),
    //         Day::ymd(0, MonthOfYear::Jul, DayOfMonth::D1)
    //     );
    //     assert_eq!(
    //         Quarter(1).start(),
    //         Day::ymd(0, MonthOfYear::Apr, DayOfMonth::D1)
    //     );
    //     assert_eq!(
    //         Quarter(0).start(),
    //         Day::ymd(0, MonthOfYear::Jan, DayOfMonth::D1)
    //     );
    //     assert_eq!(
    //         Quarter(-1).start(),
    //         Day::ymd(-1, MonthOfYear::Dec, DayOfMonth::D1)
    //     );
    //     assert_eq!(
    //         Quarter(-2).start(),
    //         Day::ymd(-1, MonthOfYear::Jul, DayOfMonth::D1)
    //     );
    // }
}
