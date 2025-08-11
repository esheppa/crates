use date::Date;

use crate::{Year, minutes::MINUTES_PER_DAY, *};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Month(i64);

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Month {
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        s.parse().map_err(serde::de::Error::custom)
    }
}

#[cfg(feature = "serde")]
impl Serialize for Month {
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

const MIN: i64 = -23640;
const MAX: i64 = 96348; // TODO

impl Month {
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
        Minutes::<1>::from_monotonic(self.0 * MINUTES_PER_DAY).expect("")
    }

    pub const fn end_minute(self) -> Minute {
        Minutes::<1>::from_monotonic(self.0 * MINUTES_PER_DAY + MINUTES_PER_DAY).expect("")
    }

    pub const fn month_of_year(self) -> MonthOfYear {
        match self.0 % 12 + 1 {
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
            _ => panic!("Can't get here, (X % 12 + 1) is always within 1..=12"),
        }
    }
    pub const fn year(self) -> Year {
        Year::from_monotonic(self.0 / 12).expect("Always valid as year is longer")
    }
    pub const fn new(year: Year, month: MonthOfYear) -> Self {
        Self(year.to_monotonic() * 12 + month.months_from_jan() as i64)
    }
}

impl TimeResolution for Month {
    const NAME: &str = "Month";

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

impl DateResolution for Month {
    type Params = ();

    type FromDay = Self;

    fn params(self) -> Self::Params {
        ()
    }

    fn from_day(day: Day, _params: Self::Params) -> Self::FromDay {
        let date = day.date();
        Self::new(
            Year::from_monotonic(date.year().num() as i64).expect("TODO"),
            date.month_of_year(),
        )
    }

    fn start_day(self) -> Day {
        Day::from_date(
            Date::first_on_month(
                date::Year::new(self.year().to_monotonic() as i32).unwrap(),
                self.month_of_year(),
            )
            .expect("Always valid"),
        )
    }
    fn end_day(self) -> Day {
        Day::from_date(
            Date::last_on_month(
                date::Year::new(self.year().to_monotonic() as i32).unwrap(),
                self.month_of_year(),
            )
            .expect("Always valid"),
        )
    }
}

impl Monotonic for Month {
    fn to_monotonic(self) -> i64 {
        self.to_monotonic()
    }

    fn between(self, other: Self) -> i64 {
        self.between(other)
    }
}

impl FromMonotonic for Month {
    fn from_monotonic(idx: i64) -> Option<Self> {
        Self::from_monotonic(idx)
    }
}

impl str::FromStr for Month {
    type Err = Error;
    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        match s.split_once('-') {
            Some((year, month)) => match (
                year.parse::<Year>(),
                month.parse::<u8>().ok().and_then(MonthOfYear::from_number),
            ) {
                (Ok(year), Some(month)) => Ok(Month::new(year, month)),
                _ => Err(Error::ParseCustom {
                    ty_name: "Month",
                    input: s.to_string(),
                }),
            },
            None => s.parse::<Day>().map(|d| Month::from_day(d, ())),
        }
    }
}

impl fmt::Display for Month {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{:02}", self.year(), self.month_of_year().number())
    }
}

#[cfg(test)]
mod tests {
    use date::{DayOfMonth, MonthOfYear};

    use super::Month;
    use crate::{DateResolution, Day};

    // #[test]
    // #[cfg(feature = "serde")]
    // fn test_roundtrip() {
    //     use crate::DateResolutionExt;

    //     let dt = chrono::NaiveDate::from_ymd_opt(2021, 12, 6).unwrap();

    //     let m1 = Month::from_day(Day::from_chrono_date(dt));
    //     assert!(m1.start_day().chrono_date() <= dt && m1.end_day().chrono_date() >= dt);

    //     let dt = chrono::NaiveDate::from_ymd_opt(2019, 7, 1).unwrap();

    //     let m2 = Month::from_day(Day::from_chrono_date(dt));

    //     assert!(m2.start_day().chrono_date() == dt);

    //     assert_eq!(
    //         m1,
    //         serde_json::from_str(&serde_json::to_string(&m1).unwrap()).unwrap()
    //     )
    // }

    // #[test]
    // fn test_parse() {
    //     assert_eq!(
    //         "Jan-2021".parse::<Month>().unwrap().start(),
    //         Day::ymd(2021, MonthOfYear::Jan, DayOfMonth::D1)
    //     );
    //     assert_eq!(
    //         "Jan-2021".parse::<Month>().unwrap().succ().start(),
    //         Day::ymd(2021, MonthOfYear::Feb, DayOfMonth::D1)
    //     );
    //     assert_eq!(
    //         "Jan-2021".parse::<Month>().unwrap().succ().pred().start(),
    //         Day::ymd(2021, MonthOfYear::Jan, DayOfMonth::D1)
    //     );
    // }

    // #[test]
    // fn test_start() {
    //     assert_eq!(
    //         Month(24240).start(),
    //         Day::ymd(2020, MonthOfYear::Jan, DayOfMonth::D1)
    //     );
    //     assert_eq!(
    //         Month(24249).start(),
    //         Day::ymd(2020, MonthOfYear::Oct, DayOfMonth::D1)
    //     );
    //     assert_eq!(
    //         Month(15).start(),
    //         Day::ymd(1, MonthOfYear::Apr, DayOfMonth::D1)
    //     );
    //     assert_eq!(
    //         Month(2).start(),
    //         Day::ymd(0, MonthOfYear::Mar, DayOfMonth::D1)
    //     );
    //     assert_eq!(
    //         Month(1).start(),
    //         Day::ymd(0, MonthOfYear::Feb, DayOfMonth::D1)
    //     );
    //     assert_eq!(
    //         Month(0).start(),
    //         Day::ymd(0, MonthOfYear::Jan, DayOfMonth::D1)
    //     );
    //     assert_eq!(
    //         Month(-1).start(),
    //         Day::ymd(-1, MonthOfYear::Dec, DayOfMonth::D1)
    //     );
    //     assert_eq!(
    //         Month(-2).start(),
    //         Day::ymd(-1, MonthOfYear::Nov, DayOfMonth::D1)
    //     );
    //     assert_eq!(
    //         Month(-15).start(),
    //         Day::ymd(-2, MonthOfYear::Oct, DayOfMonth::D1)
    //     );
    // }
}
