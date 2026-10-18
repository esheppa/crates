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

const MIN: i64 = 0;
const MAX: i64 = 9999 * 12 + 11; // TODO

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
        self.start_day().start_minute()
    }

    pub const fn end_minute(self) -> Minute {
        self.end_day().end_minute()
    }

    pub const fn start_day(self) -> Day {
        Day::from_date(
            Date::first_on_month(
                date::Year::new(self.year().to_monotonic() as i32),
                self.month_of_year(),
            )
            .expect("Always valid"),
        )
        .expect("Always valid")
    }
    pub const fn end_day(self) -> Day {
        Day::from_date(
            Date::last_on_month(
                date::Year::new(self.year().to_monotonic() as i32),
                self.month_of_year(),
            )
            .expect("Always valid"),
        )
        .expect("Always valid")
    }
    pub const fn from_day(day: Day) -> Self {
        let date = day.date();
        Self::new(
            Year::from_monotonic(date.year().num() as i64).expect("TODO"),
            date.month_of_year(),
        )
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
        Month::from_day(day)
    }

    fn start_day(self) -> Day {
        self.start_day()
    }
    fn end_day(self) -> Day {
        self.end_day()
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
            None => s.parse::<Day>().map(|d| Month::from_day(d)),
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
    use date::MonthOfYear;

    use super::*;
    use crate::{DateResolutionExt, Day, TimeResolution, Year};

    #[test]
    fn min_max_year_roundtrip_ok() {
        assert!(Month::MIN.start_day().pred().is_none());
        assert!(Month::MAX.end_day().succ().is_none());
        assert!(Year::MIN.start_p::<Month>().pred().is_none());
        assert!(Year::MAX.end_p::<Month>().succ().is_none());
        assert_eq!(Month::MIN, Year::MIN.start_p(),);
        assert_eq!(Month::MAX, Year::MAX.end_p(),);
    }

    #[test]
    fn exhaustive() {
        for i in MIN..=MAX {
            let y = Month::from_monotonic(i).unwrap();
            assert_eq!(Month::MIN.translate(i).unwrap(), y);

            assert_eq!(Month::from_day(y.start_day()), y);
            assert_eq!(Month::from_day(y.end_day()), y);
            _ = y.start_minute();
            _ = y.start_day();
            _ = y.start_p::<Day>();
            _ = y.end_minute();
            _ = y.end_day();
            _ = y.end_p::<Day>();
        }
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_serde_roundtrip() {
        for i in MIN..=MAX {
            let y = Month::from_monotonic(i).unwrap();
            let ser = serde_json::to_string(&y).unwrap();
            assert_eq!(serde_json::from_str::<Month>(&ser).unwrap(), y);
        }
    }

    #[test]
    fn test_parse_fmt() {
        assert_eq!(
            Month::new(Year::from_monotonic(2025).unwrap(), MonthOfYear::Aug)
                .to_string()
                .as_str(),
            "2025-08"
        );

        for x in 0..=9999 {
            for q in [
                MonthOfYear::Jan,
                MonthOfYear::Feb,
                MonthOfYear::Mar,
                MonthOfYear::Apr,
                MonthOfYear::May,
                MonthOfYear::Jun,
                MonthOfYear::Jul,
                MonthOfYear::Aug,
                MonthOfYear::Sep,
                MonthOfYear::Oct,
                MonthOfYear::Nov,
                MonthOfYear::Dec,
            ] {
                let qt = Month::new(Year::from_monotonic(x).unwrap(), q);
                assert_eq!(
                    format!("{x:04}-{:02}", q.number())
                        .parse::<Month>()
                        .unwrap(),
                    qt,
                );

                assert_eq!(qt.to_string().parse::<Month>().unwrap(), qt);
            }
        }
    }
}
