use date::Date;

use crate::{minutes::MINUTES_PER_DAY, *};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Day(i64);

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Day {
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        s.parse().map_err(serde::de::Error::custom)
    }
}

#[cfg(feature = "serde")]
impl Serialize for Day {
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

const MIN: i64 = 0;
const MAX: i64 = 3652060; // TODO

impl Day {
    pub const fn date(self) -> Date {
        // TODO!!!!!!!!!!!!!!!!
        Date::new(self.0 as i32)
    }
    pub const fn from_date(date: Date) -> Day {
        Day(date.inner() as i64)
    }
    pub const fn from_monotonic(idx: i64) -> Option<Self> {
        // TODO: use MIN..=MAX here when it is const
        if idx >= MIN && idx <= MAX {
            Some(Day(idx))
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
        Day::from_monotonic(new)
    }
    pub const fn start_minute(self) -> Minute {
        Minutes::<1>::from_monotonic(self.0 * MINUTES_PER_DAY).expect("")
    }

    pub const fn end_minute(self) -> Minute {
        Minutes::<1>::from_monotonic(self.0 * MINUTES_PER_DAY + MINUTES_PER_DAY).expect("")
    }
}

impl TimeResolution for Day {
    const NAME: &str = "Day";

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

impl DateResolution for Day {
    type Params = ();

    type FromDay = Day;

    fn params(self) -> Self::Params {
        ()
    }

    fn from_day(day: Day, _params: Self::Params) -> Self::FromDay {
        day
    }

    fn start_day(self) -> Day {
        self
    }
    fn end_day(self) -> Day {
        self
    }
}

impl Monotonic for Day {
    fn to_monotonic(self) -> i64 {
        self.to_monotonic()
    }

    fn between(self, other: Self) -> i64 {
        self.between(other)
    }
}

impl FromMonotonic for Day {
    fn from_monotonic(idx: i64) -> Option<Self> {
        Self::from_monotonic(idx)
    }
}

impl str::FromStr for Day {
    type Err = Error;
    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        let mut parts = s.split('-');

        let Some(year) = parts
            .next()
            .and_then(|y| y.parse::<i32>().ok())
            .and_then(date::Year::new)
        else {
            return Err(Error::ParseCustom {
                ty_name: "date",
                input: s.to_string(),
            });
        };

        let Some(month) = parts
            .next()
            .and_then(|m| MonthOfYear::from_number(m.parse::<u8>().ok()?))
        else {
            return Err(Error::ParseCustom {
                ty_name: "date",
                input: s.to_string(),
            });
        };

        let Some(day) = parts.next().and_then(|y| y.parse::<u16>().ok()) else {
            return Err(Error::ParseCustom {
                ty_name: "date",
                input: s.to_string(),
            });
        };

        let Some(date) = Date::first_on_month(year, month)
            .and_then(|d| d.translate(day.saturating_sub(1) as i32))
        else {
            return Err(Error::ParseCustom {
                ty_name: "date",
                input: s.to_string(),
            });
        };

        Ok(Day::from_date(date))
    }
}

impl fmt::Display for Day {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (year, month, day) = self.date().to_ymd();
        write!(f, "{year:04}-{month:02}-{day:02}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use date::{DayOfMonth, MonthOfYear};

    #[cfg(feature = "serde")]
    #[test]
    fn test_roundtrip() {
        let dt = chrono::NaiveDate::from_ymd_opt(2021, 12, 6).unwrap();

        let wk = Day::from_date(Date::from_chrono_date(dt));

        assert_eq!(
            wk,
            serde_json::from_str(&serde_json::to_string(&wk).unwrap()).unwrap()
        )
    }

    #[test]
    fn min_max_year_roundtrip_ok() {
        assert!(Year::MIN.start_day().pred().is_none());
        assert_eq!(
            Year::MIN.start_day(),
            Day::from_date(Date::first_on_year(date::Year::new(0).unwrap()).unwrap())
        );

        assert_eq!(
            Year::MAX.end_day(),
            Day::from_date(Date::last_on_year(date::Year::new(9999).unwrap()).unwrap())
        );

        assert!(Year::MAX.end_day().succ().is_none());
    }

    #[test]

    fn test_parse_date_syntax() {
        let year = date::Year::new(2021).unwrap();
        assert_eq!(
            "2021-01-01".parse::<Day>().unwrap(),
            Day::from_date(Date::first_on_year(year).unwrap()),
        );
        assert_eq!(
            "2021-01-01".parse::<Day>().unwrap().succ().unwrap(),
            Day::from_date(Date::ymd(year, MonthOfYear::Jan, DayOfMonth::D2).unwrap()),
        );
        assert_eq!(
            "2021-01-01"
                .parse::<Day>()
                .unwrap()
                .succ()
                .unwrap()
                .pred()
                .unwrap(),
            Day::from_date(Date::first_on_year(year).unwrap()),
        );
    }

    #[test]
    fn test_start() {
        let year = date::Year::new(0).unwrap();

        assert_eq!(
            Day::from_monotonic(2),
            Some(Day::from_date(
                Date::ymd(year, MonthOfYear::Jan, DayOfMonth::D3).unwrap()
            ))
        );
        assert_eq!(
            Day::from_monotonic(1),
            Some(Day::from_date(
                Date::ymd(year, MonthOfYear::Jan, DayOfMonth::D2).unwrap()
            ))
        );
        assert_eq!(
            Day::from_monotonic(0),
            Some(Day::from_date(
                Date::ymd(year, MonthOfYear::Jan, DayOfMonth::D1).unwrap()
            ))
        );
        assert_eq!(Day::from_monotonic(-1), None);
        assert_eq!(Day::from_monotonic(-2), None,);
    }
}
