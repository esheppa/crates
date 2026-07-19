#![no_std]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

mod prelude {
    pub use alloc::{
        collections::{self},
        format,
        string::{String, ToString},
        vec::Vec,
    };
    #[cfg(feature = "chrono")]
    pub use chrono::NaiveDate;
    pub use core::{
        any, error, fmt, iter, mem,
        num::{self, ParseIntError},
        ops::*,
        str,
    };
    #[cfg(feature = "serde")]
    pub use serde::{Deserialize, Serialize};
}
use prelude::*;

// mod range;

use date::MonthOfYear;
mod range;
pub use range::{Cache, CacheResponse, TimeRange, TimeRangeComparison, TimeRangeIter};

mod minutes;
pub use minutes::DaySubdivison;
pub use minutes::{FiveMinute, HalfHour, Hour, Minute, Minutes};

mod day;
pub use day::Day;

mod month;
pub use month::Month;

mod quarter;
pub use quarter::Quarter;

mod year;
pub use year::Year;

mod financial_year;
pub use financial_year::FinancialYear;

// TODO
// mod iso_week;
// pub use iso_week::IsoWeek;

// #[cfg(feature = "chrono")]
// mod zoned;
// #[cfg(feature = "chrono")]
// pub use zoned::{FixedTimeZone, Zoned};

// TODO: log warnings for when close to edge of range - should likely never be used

pub trait Divides<T> {}

pub trait DividedBy<T> {}

impl<T, U> DividedBy<T> for U where U: Divides<T> {}

macro_rules! impl_minutes {
    // basic impl, takes the const and impls for the shorter version
    ($long:literal, $($short:literal),+ $(,)*) => {
        impl Divides<Minutes<$long>> for Minutes<$long> {}

        $(
            impl Divides<Minutes<$long>> for Minutes<$short> {}
        )+
    };
}

macro_rules! triangle_minutes {
    // recursive case, with a long and at least one properly dividing shorts
    // impl divides for all the shorts
    // and then call triangle without the head
    ($head:literal, $($tail:literal),+ $(,)*) => {
        impl_minutes!($head, $($tail,)+);
        triangle_minutes!($($tail),+);
    };
    // base case, do nothing!
    ($head:literal) => {};
}

macro_rules! impl_divides {
    // same as impl minutes, but for longer things
    ($long:ty, $($short:ty),+ $(,)*) => {
        impl Divides<$long> for $long {}

        $(
            impl Divides<$long> for $short {}
        )+
    };
}

triangle_minutes!(
    720, 480, 360, 240, 180, 120, 60, 30, 20, 15, 10, 6, 5, 4, 3, 2, 1
);

// impl_divides!(IsoWeek, Day);
impl_divides!(Month, Day);
impl_divides!(Quarter, Month, Day);
impl_divides!(FinancialYear, Quarter, Month, Day);
impl_divides!(Year, Quarter, Month, Day);

impl<const N: u16> Divides<Day> for Minutes<N> {}
// impl<const N: u16> Divides<IsoWeek> for Minutes<N> {}
impl<const N: u16> Divides<Month> for Minutes<N> {}
impl<const N: u16> Divides<Quarter> for Minutes<N> {}
impl<const N: u16> Divides<FinancialYear> for Minutes<N> {}
impl<const N: u16> Divides<Year> for Minutes<N> {}

// TODO, bring these in when needed
// impl<const N: u16, Tz> Divides<Zoned<Day, Tz>> for Zoned<Minutes<N>, Tz> {}
// impl<const N: u16, Tz> Divides<Zoned<IsoWeek, Tz>> for Zoned<Minutes<N>, Tz> {}
// impl<const N: u16, Tz> Divides<Zoned<Month, Tz>> for Zoned<Minutes<N>, Tz> {}
// impl<const N: u16, Tz> Divides<Zoned<Quarter, Tz>> for Zoned<Minutes<N>, Tz> {}
// impl<const N: u16, Tz> Divides<Zoned<FinancialYear, Tz>> for Zoned<Minutes<N>, Tz> {}
// impl<const N: u16, Tz> Divides<Zoned<Year, Tz>> for Zoned<Minutes<N>, Tz> {}

/// This function is useful for formatting types implementing `Monotonic` when they are stored
/// in their `i64` form instead of their `TimeResolution` form. Provided you have the `TypeId` handy
/// you can find out what they were intended to be. This function handeles all the cases implemented
/// in this library and users can handle others via the function in the `handle_unknown` parameter.
pub fn format_erased_resolution(
    handle_unknown: fn(any::TypeId, i64) -> Option<String>,
    tid: any::TypeId,
    val: i64,
) -> Option<String> {
    if tid == any::TypeId::of::<Minute>() {
        Some(format!("Minute:{}", Minute::from_monotonic(val)?))
    } else if tid == any::TypeId::of::<FiveMinute>() {
        Some(format!("FiveMinute:{}", FiveMinute::from_monotonic(val)?))
    } else if tid == any::TypeId::of::<HalfHour>() {
        Some(format!("HalfHour:{}", HalfHour::from_monotonic(val)?))
    } else if tid == any::TypeId::of::<Hour>() {
        Some(format!("Hour:{}", Hour::from_monotonic(val)?))
    } else if tid == any::TypeId::of::<Day>() {
        Some(format!("Day:{}", Day::from_monotonic(val)?))
    } else if tid == any::TypeId::of::<Month>() {
        Some(format!("Month:{}", Month::from_monotonic(val)?))
    } else if tid == any::TypeId::of::<Quarter>() {
        Some(format!("Quarter:{}", Quarter::from_monotonic(val)?))
    } else if tid == any::TypeId::of::<Year>() {
        Some(format!("Year:{}", Year::from_monotonic(val)?))
    } else {
        handle_unknown(tid, val)
    }
}

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
    DayFromDate(date::Date),
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
            DayFromDate(date) => write!(f, "Unable to create Day from {date}",),
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
        Out: TimeResolution + FromMinute,
    {
        Out::from_minute(self.start_minute())
    }

    // handy functions.... to avoid turbofishing when it's a pain
    // fn day(self) -> Day;
    // fn month(self) -> Month;
    // fn quarter(self) -> Month;
    // fn year(self) -> Year;
    // no week/finyear becuase they don't fill the period
}

pub trait FromMinute {
    fn from_minute(minute: Minute) -> Self;
}

// we may decide later to use i64 or even i128 instead
// however this would only be to increase detail below Minute, eg Second, MilliSecond, etc.
/// `Monotonic` is used to enable multiple different resolutions to be stored together
///
/// It is named monotonic as it is intended to provide a monotonic (order preserving) function
/// from a given implementor of `TimeResolution`, to allow converting backwards and forwards
/// between the values of the `TimeResolution` implementor and `i64`s
pub trait Monotonic: Eq + Ord {
    // we choose i64 rather than u32
    // as the behaviour on subtraction is nicer!
    fn to_monotonic(self) -> i64;
    fn between(self, other: Self) -> i64;
}

pub trait FromMonotonic: Monotonic + Sized {
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
        Self: DividedBy<Out>,
    {
        range::TimeRange::from_bounds(self.start_p(), self.end_p())
    }

    fn start_p<Out>(self) -> Out
    where
        Out: DateResolution<Params = Self::Params, FromDay = Out> + FromMonotonic,
        Self: DividedBy<Out>,
    {
        Out::from_day(self.start_day(), self.params())
    }
    fn end_p<Out>(self) -> Out
    where
        Out: DateResolution<Params = Self::Params, FromDay = Out> + FromMonotonic,
        Self: DividedBy<Out>,
    {
        Out::from_day(self.end_day(), self.params())
    }

    fn parent<Out>(self) -> Out
    where
        Out: DateResolution<Params = Self::Params, FromDay = Out> + FromMonotonic,
        Self: Divides<Out>,
    {
        Out::from_day(self.start_day(), self.params())
    }
}

impl<T> DateResolutionExt for T where T: DateResolution {}

#[cfg(kani)]
#[kani::proof]
pub fn verify() {}
