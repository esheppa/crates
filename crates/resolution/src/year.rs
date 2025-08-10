use date::Date;

use crate::{minutes::MINUTES_PER_DAY, *};
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

const MIN: i64 = -1970;
const MAX: i64 = 8029; // TODO

impl Year {
    const fn date_year(self) -> date::Year {
        date::Year::new(self.to_monotonic() as i32 + 1970).unwrap()
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
        Minutes::<1>::from_monotonic(self.0 * MINUTES_PER_DAY).expect("")
    }

    pub const fn end_minute(self) -> Minute {
        Minutes::<1>::from_monotonic(self.0 * MINUTES_PER_DAY + MINUTES_PER_DAY).expect("")
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
        let date = day.date();
        Year::from_monotonic(date.year().num() as i64).expect("TODO")
    }

    fn start_day(self) -> Day {
        extern crate std;
        std::dbg!(self.to_monotonic());
        Day::from_date(Date::first_on_year(self.date_year()).expect("Always valid"))
    }

    fn end_day(self) -> Day {
        Day::from_date(Date::last_on_year(self.date_year()).expect("Always valid"))
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
    use quarter::QuarterOfYear;

    use super::*;

    #[test]
    fn test_builder() {
        assert_eq!(
            Year::from_monotonic(2024).unwrap().q1(),
            Quarter::new(Year::from_monotonic(2024).unwrap(), QuarterOfYear::Q1)
        );
    }
}
