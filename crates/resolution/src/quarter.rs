use date::Date;

use crate::{Year, minutes::MINUTES_PER_DAY, *};
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Quarter(i64);

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
}

const MIN: i64 = -7880;
const MAX: i64 = 32116; // TODO

impl Quarter {
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
        Some(Self(new))
    }
    pub const fn start_minute(self) -> Minute {
        Minutes::<1>::from_monotonic(self.0 * MINUTES_PER_DAY).expect("")
    }

    pub const fn end_minute(self) -> Minute {
        Minutes::<1>::from_monotonic(self.0 * MINUTES_PER_DAY + MINUTES_PER_DAY).expect("")
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
            Year::from_monotonic(date.year_num().into()).expect("TODO"),
            QuarterOfYear::from_month(date.month_of_year()),
        )
    }

    fn start_day(self) -> Day {
        Day::from_date(
            Date::first_on_month(
                self.year().to_monotonic() as i32,
                self.quarter_of_year().start_month(),
            )
            .expect("Always valid"),
        )
    }

    fn end_day(self) -> Day {
        Day::from_date(
            Date::last_on_month(
                self.year().to_monotonic() as i32,
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
