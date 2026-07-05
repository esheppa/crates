use date::Date;

use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Year(i64);

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Year {
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        s.parse().map_err(serde::de::Error::custom)
    }
}

#[cfg(feature = "serde")]
impl Serialize for Year {
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

const MIN: i64 = 0;
const MAX: i64 = 9999; // TODO

impl Year {
    const fn date_year(self) -> date::Year {
        date::Year::new(self.to_monotonic() as i32)
    }
    pub const MIN: Year = Year(MIN);
    pub const MAX: Year = Year(MAX);
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

    pub const fn from_day(day: Day) -> Year {
        let date = day.date();
        Year::from_monotonic(date.year().num() as i64).expect("TODO")
    }

    pub const fn start_day(self) -> Day {
        let date = Date::first_on_year(self.date_year()).expect("Always valid");
        Day::from_date(date).expect("Always valid")
    }

    pub const fn end_day(self) -> Day {
        Day::from_date(Date::last_on_year(self.date_year()).expect("Always valid"))
            .expect("Always valid")
    }

    pub const fn q1(self) -> Quarter {
        Quarter::new(self, quarter::QuarterOfYear::Q1)
    }
    pub const fn q2(self) -> Quarter {
        Quarter::new(self, quarter::QuarterOfYear::Q2)
    }
    pub const fn q3(self) -> Quarter {
        Quarter::new(self, quarter::QuarterOfYear::Q3)
    }
    pub const fn q4(self) -> Quarter {
        Quarter::new(self, quarter::QuarterOfYear::Q4)
    }
    pub const fn jan(self) -> Month {
        Month::new(self, MonthOfYear::Jan)
    }
    pub const fn feb(self) -> Month {
        Month::new(self, MonthOfYear::Feb)
    }
    pub const fn mar(self) -> Month {
        Month::new(self, MonthOfYear::Mar)
    }
    pub const fn apr(self) -> Month {
        Month::new(self, MonthOfYear::Apr)
    }
    pub const fn may(self) -> Month {
        Month::new(self, MonthOfYear::May)
    }
    pub const fn jun(self) -> Month {
        Month::new(self, MonthOfYear::Jun)
    }
    pub const fn jul(self) -> Month {
        Month::new(self, MonthOfYear::Jul)
    }
    pub const fn aug(self) -> Month {
        Month::new(self, MonthOfYear::Aug)
    }
    pub const fn sep(self) -> Month {
        Month::new(self, MonthOfYear::Sep)
    }
    pub const fn oct(self) -> Month {
        Month::new(self, MonthOfYear::Oct)
    }
    pub const fn nov(self) -> Month {
        Month::new(self, MonthOfYear::Nov)
    }
    pub const fn dec(self) -> Month {
        Month::new(self, MonthOfYear::Dec)
    }
}

impl TimeResolution for Year {
    const NAME: &str = "Year";

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

impl DateResolution for Year {
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

impl Monotonic for Year {
    fn to_monotonic(self) -> i64 {
        self.to_monotonic()
    }

    fn between(self, other: Self) -> i64 {
        self.between(other)
    }
}

impl FromMonotonic for Year {
    fn from_monotonic(idx: i64) -> Option<Self> {
        Self::from_monotonic(idx)
    }
}

impl fmt::Display for Year {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04}", self.0)
    }
}

impl str::FromStr for Year {
    type Err = Error;
    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        match Year::from_monotonic(s.parse()?) {
            Some(y) => Ok(y),
            None => Err(Error::ParseCustom {
                ty_name: "Year",
                input: s.to_string(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate std;

    use quarter::QuarterOfYear;

    use super::*;

    #[test]
    fn test_builder() {
        assert_eq!(
            Year::from_monotonic(2024).unwrap().q1(),
            Quarter::new(Year::from_monotonic(2024).unwrap(), QuarterOfYear::Q1)
        );
    }

    #[test]
    fn min_max_year_roundtrip_ok() {
        assert!(Year::MIN.start_day().pred().is_none());
        assert_eq!(
            Year::MIN.start_day(),
            Day::from_date(Date::first_on_year(date::Year::new(0)).unwrap()).unwrap()
        );
        assert_eq!(
            Year::MAX.end_day(),
            Day::from_date(Date::last_on_year(date::Year::new(9999)).unwrap()).unwrap()
        );
        assert!(Year::MAX.end_day().succ().is_none());
    }

    #[test]
    fn exhaustive() {
        for i in MIN..=MAX {
            let y = Year::from_monotonic(i).unwrap();
            assert_eq!(Year::MIN.translate(i).unwrap(), y);

            assert_eq!(Year::from_day(y.start_day()), y);
            assert_eq!(Year::from_day(y.end_day()), y);
            _ = y.start_minute();
            _ = y.start_day();
            _ = y.start_p::<Quarter>();
            _ = y.start_p::<Month>();
            _ = y.start_p::<Day>();
            _ = y.end_minute();
            _ = y.end_day();
            _ = y.end_p::<Quarter>();
            _ = y.end_p::<Month>();
            _ = y.end_p::<Day>();
        }
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_roundtrip() {
        for i in MIN..=MAX {
            let y = Year::from_monotonic(i).unwrap();
            let ser = serde_json::to_string(&y).unwrap();
            assert_eq!(serde_json::from_str::<Year>(&ser).unwrap(), y);
        }
    }

    #[test]
    fn test_parse_fmt() {
        for x in MIN..=MAX {
            let y = Year::from_monotonic(x).unwrap();
            assert_eq!(format!("{x:04}").parse::<Year>().unwrap(), y,);

            assert_eq!(y.to_string().parse::<Year>().unwrap(), y);
        }
    }
}
