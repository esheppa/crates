use date::Date;

use crate::{Year, minutes::MINUTES_PER_DAY, *};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Month(i64);

const MIN: i64 = -23640;
const MAX: i64 = 96348; // TODO

impl Month {
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
        Some(Self(new))
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
        Self(year.to_monotonic() * 12 + month.number() as i64)
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
            Year::from_monotonic(date.year_num().into()).expect("TODO"),
            date.month_of_year(),
        )
    }

    fn start_day(self) -> Day {
        Day::from_date(
            Date::first_on_month(self.year().to_monotonic() as i32, self.month_of_year())
                .expect("Always valid"),
        )
    }
    fn end_day(self) -> Day {
        Day::from_date(
            Date::last_on_month(self.year().to_monotonic() as i32, self.month_of_year())
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
