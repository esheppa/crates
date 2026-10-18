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

const MIN: i64 = 0;
const MAX: i64 = 9999 * 4 + 3; // TODO

impl Quarter {
    pub const MIN: Self = Self(MIN);
    pub const MAX: Self = Self(MAX);
    pub const fn new(year: Year, q: QuarterOfYear) -> Self {
        Self(year.to_monotonic() * 4 + q.offset() as i64)
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
        self.start_day().start_minute()
    }

    pub const fn end_minute(self) -> Minute {
        self.end_day().end_minute()
    }

    pub const fn start_day(self) -> Day {
        Day::from_date(
            Date::first_on_month(
                date::Year::new(self.year().to_monotonic() as i32),
                self.quarter_of_year().start_month(),
            )
            .expect("Always valid"),
        )
        .expect("Always valid")
    }

    pub const fn end_day(self) -> Day {
        Day::from_date(
            Date::last_on_month(
                date::Year::new(self.year().to_monotonic() as i32),
                self.quarter_of_year().end_month(),
            )
            .expect("Always valid"),
        )
        .expect("Always valid")
    }

    pub const fn from_day(day: Day) -> Self {
        let date = day.date();
        Self::new(
            Year::from_monotonic(date.year().num() as i64).expect("TODO"),
            QuarterOfYear::from_month(date.month_of_year()),
        )
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
        Self::from_day(day)
    }

    fn start_day(self) -> Day {
        self.start_day()
    }

    fn end_day(self) -> Day {
        self.end_day()
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
        write!(f, "{:04}-Q{}", self.year(), self.quarter_of_year().number())
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
            None => Err(crate::Error::ParseCustom {
                ty_name: "Quarter",
                input: s.to_string(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_max_year_roundtrip_ok() {
        assert_eq!(Quarter::MIN.start_day().pred(), None);
        assert_eq!(Quarter::MAX.end_day().succ(), None);
        assert_eq!(Year::MIN.start_p::<Quarter>().pred(), None);
        assert_eq!(Year::MAX.end_p::<Quarter>().succ(), None);
        assert_eq!(Quarter::MIN, Year::MIN.start_p(),);
        assert_eq!(Quarter::MAX, Year::MAX.end_p(),);
    }

    #[test]
    fn exhaustive() {
        for i in MIN..=MAX {
            let y = Quarter::from_monotonic(i).unwrap();
            assert_eq!(Quarter::MIN.translate(i).unwrap(), y);
            assert_eq!(Quarter::from_day(y.start_day()), y);
            assert_eq!(Quarter::from_day(y.end_day()), y);
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
    fn test_serde_roundtrip() {
        for i in MIN..=MAX {
            let y = Quarter::from_monotonic(i).unwrap();
            let ser = serde_json::to_string(&y).unwrap();
            assert_eq!(serde_json::from_str::<Quarter>(&ser).unwrap(), y);
        }
    }

    #[test]
    fn test_parse_fmt() {
        assert_eq!(
            Quarter::new(Year::from_monotonic(2025).unwrap(), QuarterOfYear::Q3)
                .to_string()
                .as_str(),
            "2025-Q3"
        );

        for x in 0..=9999 {
            for q in [
                QuarterOfYear::Q1,
                QuarterOfYear::Q2,
                QuarterOfYear::Q3,
                QuarterOfYear::Q4,
            ] {
                let qt = Quarter::new(Year::from_monotonic(x).unwrap(), q);
                assert_eq!(
                    format!("{x:04}-Q{}", q.number())
                        .parse::<Quarter>()
                        .unwrap(),
                    qt,
                );

                assert_eq!(qt.to_string().parse::<Quarter>().unwrap(), qt);
            }
        }
    }
}
