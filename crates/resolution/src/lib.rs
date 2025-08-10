#![no_std]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

use core::{
    any, error, fmt,
    num::{self, ParseIntError},
    str,
};

// mod range;
use alloc::{format, string::String};
#[cfg(feature = "chrono")]
use chrono::{DateTime, NaiveDate, Utc};
use date::MonthOfYear;
mod range;
pub use range::{Cache, CacheResponse, TimeRange, TimeRangeComparison, TimeRangeIter};

mod minutes;
// pub use minutes::{DaySubdivison};
pub use minutes::{FiveMinute, HalfHour, Hour, Minute, Minutes};

mod day;
pub use day::Day;

// mod week;
// pub use week::{Friday, Monday, Saturday, StartDay, Sunday, Thursday, Tuesday, Wednesday, Week};

mod month;
pub use month::Month;

mod quarter;
pub use quarter::Quarter;

mod year;
pub use year::Year;

mod financial_year;
pub use financial_year::FinancialYear;

mod iso_week;
pub use iso_week::IsoWeek;

// #[cfg(feature = "chrono")]
// mod zoned;
// #[cfg(feature = "chrono")]
// pub use zoned::{FixedTimeZone, Zoned};

// TODO: log warnings for when close to edge of range - should likely never be used

pub trait LongerThan<T>: LongerThanOrEqual<T> {}

pub trait LongerThanOrEqual<T> {}

impl<T> LongerThanOrEqual<T> for T {}

pub trait ShorterThan<T>: ShorterThanOrEqual<T> {}

impl<Long, Short> ShorterThan<Long> for Short where
    Long: LongerThanOrEqual<Short> + LongerThan<Short>
{
}

pub trait ShorterThanOrEqual<T> {}

impl<Long, Short> ShorterThanOrEqual<Long> for Short where Long: LongerThan<Short> {}

// TODO: use macro for this

impl LongerThanOrEqual<Minute> for FiveMinute {}
impl LongerThanOrEqual<Minute> for HalfHour {}
impl LongerThanOrEqual<Minute> for Hour {}
impl LongerThanOrEqual<Minute> for Day {}
impl LongerThanOrEqual<Minute> for IsoWeek {}
impl LongerThanOrEqual<Minute> for Month {}
impl LongerThanOrEqual<Minute> for Quarter {}
impl LongerThanOrEqual<Minute> for Year {}

impl LongerThan<Minute> for FiveMinute {}
impl LongerThan<Minute> for HalfHour {}
impl LongerThan<Minute> for Hour {}
impl LongerThan<Minute> for Day {}
impl LongerThan<Minute> for IsoWeek {}
impl LongerThan<Minute> for Month {}
impl LongerThan<Minute> for Quarter {}
impl LongerThan<Minute> for Year {}

impl LongerThanOrEqual<FiveMinute> for HalfHour {}
impl LongerThanOrEqual<FiveMinute> for Hour {}
impl LongerThanOrEqual<FiveMinute> for Day {}
impl LongerThanOrEqual<FiveMinute> for IsoWeek {}
impl LongerThanOrEqual<FiveMinute> for Month {}
impl LongerThanOrEqual<FiveMinute> for Quarter {}
impl LongerThanOrEqual<FiveMinute> for Year {}

impl LongerThan<FiveMinute> for HalfHour {}
impl LongerThan<FiveMinute> for Hour {}
impl LongerThan<FiveMinute> for Day {}
impl LongerThan<FiveMinute> for IsoWeek {}
impl LongerThan<FiveMinute> for Month {}
impl LongerThan<FiveMinute> for Quarter {}
impl LongerThan<FiveMinute> for Year {}

impl LongerThanOrEqual<HalfHour> for Hour {}
impl LongerThanOrEqual<HalfHour> for Day {}
impl LongerThanOrEqual<HalfHour> for IsoWeek {}
impl LongerThanOrEqual<HalfHour> for Month {}
impl LongerThanOrEqual<HalfHour> for Quarter {}
impl LongerThanOrEqual<HalfHour> for Year {}

impl LongerThan<HalfHour> for Hour {}
impl LongerThan<HalfHour> for Day {}
impl LongerThan<HalfHour> for IsoWeek {}
impl LongerThan<HalfHour> for Month {}
impl LongerThan<HalfHour> for Quarter {}
impl LongerThan<HalfHour> for Year {}

impl LongerThanOrEqual<Hour> for Day {}
impl LongerThanOrEqual<Hour> for IsoWeek {}
impl LongerThanOrEqual<Hour> for Month {}
impl LongerThanOrEqual<Hour> for Quarter {}
impl LongerThanOrEqual<Hour> for Year {}

impl LongerThan<Hour> for Day {}
impl LongerThan<Hour> for IsoWeek {}
impl LongerThan<Hour> for Month {}
impl LongerThan<Hour> for Quarter {}
impl LongerThan<Hour> for Year {}

impl LongerThanOrEqual<Day> for IsoWeek {}
impl LongerThanOrEqual<Day> for Month {}
impl LongerThanOrEqual<Day> for Quarter {}
impl LongerThanOrEqual<Day> for Year {}

impl LongerThan<Day> for IsoWeek {}
impl LongerThan<Day> for Month {}
impl LongerThan<Day> for Quarter {}
impl LongerThan<Day> for Year {}

impl LongerThanOrEqual<IsoWeek> for Quarter {}
impl LongerThanOrEqual<IsoWeek> for Month {}
impl LongerThanOrEqual<IsoWeek> for Year {}

impl LongerThan<IsoWeek> for Month {}
impl LongerThan<IsoWeek> for Quarter {}
impl LongerThan<IsoWeek> for Year {}

impl LongerThanOrEqual<Month> for Quarter {}
impl LongerThanOrEqual<Month> for Year {}

impl LongerThan<Month> for Quarter {}
impl LongerThan<Month> for Year {}

impl LongerThanOrEqual<Quarter> for Year {}

impl LongerThan<Quarter> for Year {}

// /// This function is useful for formatting types implementing `Monotonic` when they are stored
// /// in their `i64` form instead of their `TimeResolution` form. Provided you have the `TypeId` handy
// /// you can find out what they were intended to be. This function handeles all the cases implemented
// /// in this library and users can handle others via the function in the `handle_unknown` parameter.
// pub fn format_erased_resolution(
//     handle_unknown: fn(any::TypeId, i64) -> String,
//     tid: any::TypeId,
//     val: i64,
// ) -> String {
//     if tid == any::TypeId::of::<Minute>() {
//         format!("Minute:{}", Minute::from_monotonic(val))
//     } else if tid == any::TypeId::of::<FiveMinute>() {
//         format!("FiveMinute:{}", FiveMinute::from_monotonic(val))
//     } else if tid == any::TypeId::of::<HalfHour>() {
//         format!("HalfHour:{}", HalfHour::from_monotonic(val))
//     } else if tid == any::TypeId::of::<Hour>() {
//         format!("Hour:{}", Hour::from_monotonic(val))
//     } else if tid == any::TypeId::of::<Day>() {
//         format!("Day:{}", Day::from_monotonic(val))
//     } else if tid == any::TypeId::of::<Week<week::Monday>>() {
//         format!("Week:{}", Week::<week::Monday>::from_monotonic(val))
//     } else if tid == any::TypeId::of::<Week<week::Tuesday>>() {
//         format!("Week:{}", Week::<week::Tuesday>::from_monotonic(val))
//     } else if tid == any::TypeId::of::<Week<week::Wednesday>>() {
//         format!("Week:{}", Week::<week::Wednesday>::from_monotonic(val))
//     } else if tid == any::TypeId::of::<Week<week::Thursday>>() {
//         format!("Week:{}", Week::<week::Thursday>::from_monotonic(val))
//     } else if tid == any::TypeId::of::<Week<week::Friday>>() {
//         format!("Week:{}", Week::<week::Friday>::from_monotonic(val))
//     } else if tid == any::TypeId::of::<Week<week::Saturday>>() {
//         format!("Week:{}", Week::<week::Saturday>::from_monotonic(val))
//     } else if tid == any::TypeId::of::<Week<week::Sunday>>() {
//         format!("Week:{}", Week::<week::Sunday>::from_monotonic(val))
//     } else if tid == any::TypeId::of::<Month>() {
//         format!("Month:{}", Month::from_monotonic(val))
//     } else if tid == any::TypeId::of::<Quarter>() {
//         format!("Quarter:{}", Quarter::from_monotonic(val))
//     } else if tid == any::TypeId::of::<Year>() {
//         format!("Year:{}", Year::from_monotonic(val))
//     } else {
//         handle_unknown(tid, val)
//     }
// }

impl error::Error for Error {}
#[derive(Debug)]
pub enum Error {
    GotNonMatchingNewData {
        point: String,
        old: String,
        new: String,
    },
    ParseInt(num::ParseIntError),
    #[cfg(feature = "chrono")]
    ParseDate(chrono::ParseError),
    ParseCustom {
        ty_name: &'static str,
        input: String,
    },
    EmptyRange,
    #[cfg(feature = "chrono")]
    UnexpectedStartDate {
        date: chrono::NaiveDate,
        required: chrono::Weekday,
        actual: chrono::Weekday,
    },
    UnexpectedInputLength {
        required: usize,
        actual: usize,
        format: &'static str,
    },
    ParseIntDetailed(ParseIntError, String),
    ParseDateInternal {
        message: String,
        input: String,
        format: &'static str,
    },
}

impl From<num::ParseIntError> for Error {
    fn from(e: num::ParseIntError) -> Error {
        Error::ParseInt(e)
    }
}
#[cfg(feature = "chrono")]
impl From<chrono::ParseError> for Error {
    fn from(e: chrono::ParseError) -> Error {
        Error::ParseDate(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;
        match self {
            GotNonMatchingNewData { point, old, new } => write!(
                f,
                "Got new data for {point}: {new} different from data already in the cache {old}"
            ),
            ParseInt(e) => write!(f, "Error parsing int: {e}"),
            #[cfg(feature = "chrono")]
            ParseDate(e) => write!(f, "Error parsing date/time: {e}"),
            ParseCustom { ty_name, input } => {
                write!(f, "Error parsing {ty_name} from input: {input}")
            }
            EmptyRange => write!(
                f,
                "Time range cannot be created from an empty set of periods"
            ),
            #[cfg(feature = "chrono")]
            UnexpectedStartDate {
                date,
                required,
                actual,
            } => write!(
                f,
                "Unexpected input length for date {date}, got {actual} but needed {required}"
            ),
            UnexpectedInputLength {
                required,
                actual,
                format,
            } => write!(
                f,
                "Unexpected input length for format {format}, got {actual} but needed {required}"
            ),
            ParseIntDetailed(e, detail) => {
                write!(f, "Error parsing {detail} as integer: {e}")
            }
            ParseDateInternal {
                message,
                input,
                format,
            } => {
                write!(
                    f,
                    "Error parsing {input} as date due to {message} using format {format}"
                )
            }
        }
    }
}

pub trait Convert<T> {
    fn convert(self) -> T;
}

#[cfg(feature = "std")]
pub type Result<T> = std::result::Result<T, Error>;

/// `TimeResolution` should be used for contigious series of periods in time
///
/// This makes sense for the time part of a discrete timeseries, with observations
/// occurring at regular times. Some examples are:
/// * A cash-flow report aggregated to days or months
/// * Dispatch periods in the Australian Electricity Market (and similar concepts in other energy markets)
pub trait TimeResolution: Monotonic + Copy {
    const NAME: &str;
    fn succ(self) -> Option<Self> {
        self.translate(1)
    }

    fn pred(self) -> Option<Self> {
        self.translate(-1)
    }

    fn translate(self, n: i64) -> Option<Self>;

    fn start_minute(self) -> Minute;
    fn end_minute(self) -> Minute;

    // #[cfg(feature = "chrono")]
    // fn start_datetime(self) -> DateTime<Utc>;

    fn convert<Out>(self) -> Out
    where
        Out: TimeResolution + From<Minute>,
    {
        Out::from(self.start_minute())
    }

    // handy functions.... to avoid turbofishing when it's a pain
    // fn day(self) -> Day;
    // fn month(self) -> Month;
    // fn quarter(self) -> Month;
    // fn year(self) -> Year;
    // no week/finyear becuase they don't fill the period
}

// we may decide later to use i64 or even i128 instead
// however this would only be to increase detail below Minute, eg Second, MilliSecond, etc.
/// `Monotonic` is used to enable multiple different resolutions to be stored together
///
/// It is named monotonic as it is intended to provide a monotonic (order preserving) function
/// from a given implementor of `TimeResolution`, to allow converting backwards and forwards
/// between the values of the `TimeResolution` implementor and `i64`s
pub trait Monotonic: Copy + Eq + Ord {
    // we choose i64 rather than u32
    // as the behaviour on subtraction is nicer!
    fn to_monotonic(self) -> i64;
    fn between(self, other: Self) -> i64;
}

pub trait FromMonotonic: Monotonic {
    fn from_monotonic(idx: i64) -> Option<Self>;
}

/// `SubDateResolution` should only be implemented for periods of strictly less than one day in length
pub trait SubDateResolution: TimeResolution {
    type Params: Copy;

    fn params(self) -> Self::Params;

    fn occurs_on_day(self) -> Day;

    // #[cfg(feature = "chrono")]
    // fn from_utc_datetime(datetime: DateTime<Utc>, params: Self::Params) -> Self;

    // #[cfg(feature = "std")]
    // fn from_systemtime(systime: std::time::SystemTime, params: Self::Params) -> Self;

    fn from_minute(minute: Minute, params: Self::Params) -> Self;

    // the first of the resolutions units that occurs on the day
    fn first_on_day(day: Day, params: Self::Params) -> Self;

    fn last_on_day(day: Day, params: Self::Params) -> Self;
}

/// `DateResolution` should only be implemented for periods of one or more days in length
pub trait DateResolution: TimeResolution {
    // for timezones
    type Params;
    // eg, Self, or Option<Self> ... other choices would be less useful...
    type FromDay;
    fn params(self) -> Self::Params;
    fn from_day(day: Day, params: Self::Params) -> Self::FromDay;
    fn start_day(self) -> Day;
    fn end_day(self) -> Day;
}

/// `DateResolutionExt` implements some convenience methods for types that implement `DateResolution`
// This is an extra trait to avoid the methods being overriden
pub trait DateResolutionExt: DateResolution {
    fn num_days(self) -> i64 {
        self.start_day().between(self.end_day())
    }

    fn to_sub_date_resolution<R>(self) -> range::TimeRange<R>
    where
        R: SubDateResolution<Params = Self::Params> + FromMonotonic,
    {
        range::TimeRange::from_bounds(
            R::first_on_day(self.start_day(), self.params()),
            R::last_on_day(self.end_day(), self.params()),
        )
    }

    fn rescale<Out>(self) -> range::TimeRange<Out>
    where
        Out: DateResolution<Params = Self::Params, FromDay = Out> + FromMonotonic,
        Self: LongerThan<Out>,
    {
        range::TimeRange::from_bounds(
            Out::from_day(self.start_day(), self.params()),
            Out::from_day(self.end_day(), self.params()),
        )
    }
}

impl<T> DateResolutionExt for T where T: DateResolution {}

trait DateResolutionBuilder {
    fn q1(self) -> Quarter;
    fn q2(self) -> Quarter;
    fn q3(self) -> Quarter;
    fn q4(self) -> Quarter;
    fn jan(self) -> Month;
    fn feb(self) -> Month;
    fn mar(self) -> Month;
    fn apr(self) -> Month;
    fn may(self) -> Month;
    fn jun(self) -> Month;
    fn jul(self) -> Month;
    fn aug(self) -> Month;
    fn sep(self) -> Month;
    fn oct(self) -> Month;
    fn nov(self) -> Month;
    fn dec(self) -> Month;
}
impl DateResolutionBuilder for i16 {
    fn q1(self) -> Quarter {
        Quarter::new(
            Year::from_monotonic(self as i64).unwrap(),
            quarter::QuarterOfYear::Q1,
        )
    }
    fn q2(self) -> Quarter {
        Quarter::new(
            Year::from_monotonic(self as i64).unwrap(),
            quarter::QuarterOfYear::Q2,
        )
    }
    fn q3(self) -> Quarter {
        Quarter::new(
            Year::from_monotonic(self as i64).unwrap(),
            quarter::QuarterOfYear::Q3,
        )
    }
    fn q4(self) -> Quarter {
        Quarter::new(
            Year::from_monotonic(self as i64).unwrap(),
            quarter::QuarterOfYear::Q4,
        )
    }
    fn jan(self) -> Month {
        Month::new(Year::from_monotonic(self as i64).unwrap(), MonthOfYear::Jan)
    }
    fn feb(self) -> Month {
        Month::new(Year::from_monotonic(self as i64).unwrap(), MonthOfYear::Feb)
    }
    fn mar(self) -> Month {
        Month::new(Year::from_monotonic(self as i64).unwrap(), MonthOfYear::Mar)
    }
    fn apr(self) -> Month {
        Month::new(Year::from_monotonic(self as i64).unwrap(), MonthOfYear::Apr)
    }
    fn may(self) -> Month {
        Month::new(Year::from_monotonic(self as i64).unwrap(), MonthOfYear::May)
    }
    fn jun(self) -> Month {
        Month::new(Year::from_monotonic(self as i64).unwrap(), MonthOfYear::Jun)
    }
    fn jul(self) -> Month {
        Month::new(Year::from_monotonic(self as i64).unwrap(), MonthOfYear::Jul)
    }
    fn aug(self) -> Month {
        Month::new(Year::from_monotonic(self as i64).unwrap(), MonthOfYear::Aug)
    }
    fn sep(self) -> Month {
        Month::new(Year::from_monotonic(self as i64).unwrap(), MonthOfYear::Sep)
    }
    fn oct(self) -> Month {
        Month::new(Year::from_monotonic(self as i64).unwrap(), MonthOfYear::Oct)
    }
    fn nov(self) -> Month {
        Month::new(Year::from_monotonic(self as i64).unwrap(), MonthOfYear::Nov)
    }
    fn dec(self) -> Month {
        Month::new(Year::from_monotonic(self as i64).unwrap(), MonthOfYear::Dec)
    }
}

#[cfg(test)]
mod tests {
    use quarter::QuarterOfYear;

    use super::*;

    #[test]
    fn test_builder() {
        assert_eq!(
            2024.q1(),
            Quarter::from_parts(Year::new(2024), QuarterOfYear::Q1)
        );
        assert_eq!(2024.q1(), Year::new(2024).first_quarter());
        assert_eq!(Year::new(2024).q1(), Year::new(2024).first_quarter());
    }
}
