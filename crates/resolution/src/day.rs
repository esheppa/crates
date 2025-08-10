use date::Date;

use crate::{minutes::MINUTES_PER_DAY, *};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Day(i64);

const MIN: i64 = -3652060;
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
        Some(Day(new))
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
