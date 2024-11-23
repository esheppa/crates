use core::fmt::Debug;
use core::num::NonZeroU16;

use crate::{
    Convert, Day, Error, FromMonotonic, Minute, Monotonic, Month, SubDateResolution,
    TimeResolution, Year,
};
use alloc::{fmt, format, str, string::String};
#[cfg(feature = "chrono")]
use chrono::{DateTime, Datelike, Duration, NaiveDate, NaiveTime, Timelike, Utc};
use date_impl::Date;

// leap seconds are ignored here
const NUM_SECS: i32 = 60;

const MINUTES_PER_DAY: i32 = 24 * 60;

/// Note that for sensible behaviour, the N chosen should be a number that either:
/// 1. divides into an hour with no remainder (1, 2, 3, 4, 5, 6, 10, 12, 15, 20, 30, 60)
/// 2. is exactly a whole number of hours that divides into a day with no remainder (60, 120, 180, 240, 360, 480, 1800)
/// Any other choice will result in unexpected / unuseful behaviour (eg the `Minutes` not cleanly fitting into parts of a day)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(try_from = "Minutes_", into = "Minutes_"))]
pub struct Minutes<const N: u32> {
    index: i32,
}

impl<const N: u32> TryFrom<Minutes_> for Minutes<N> {
    type Error = String;
    fn try_from(value: Minutes_) -> Result<Self, Self::Error> {
        if value.length == N {
            Ok(Minutes { index: value.index })
        } else {
            Err(format!(
                "To create a Minutes[Length:{}], the length field should be {} but was instead {}",
                N, N, value.length
            ))
        }
    }
}

impl<const N: u32> From<Minutes<N>> for Minutes_ {
    fn from(w: Minutes<N>) -> Self {
        Minutes_ {
            index: w.index,
            length: N,
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub(crate) struct Minutes_ {
    index: i32,
    pub(crate) length: u32,
}

#[cfg(feature = "chrono")]
impl<const N: u32> From<DateTime<Utc>> for Minutes<N> {
    fn from(d: DateTime<Utc>) -> Self {
        Minutes {
            index: d.timestamp().div_euclid(60 * i32::from(N)),
        }
    }
}
// macro_rules! minutes_impl_change_resolution {
//     ($i:literal, $out:literal) => {
//         impl Minutes<$i> {
//    const fn change_resolution(self) -> Minutes<$out> {
//                 if N2 == $i {
//                     Minutes { index: self.index }
//                 } else if N2 > $i {
//                     // long day subdivions to short
//                     // clean scaling
//                     if N2 % $i == 0 {
//                         Minutes {
//                             index: self.index / Self::PERIODS_PER_DAY
//                                 * Minutes::<N2>::PERIODS_PER_DAY,
//                         }
//                     } else {
//                         // non matched scaling, default earlier
//                         todo!()
//                     }
//                 } else {
//                     // short day subdivision to long
//                     if N % N2 == 0 {
//                         Minutes {
//                             index: self.index / Minutes::<N2>::PERIODS_PER_DAY
//                                 * Self::PERIODS_PER_DAY,
//                         }
//                     } else {
//                         // non matched scaling, default earlier
//                         todo!()
//                     }
//                 }
//             }
//         }
//     };
// }

macro_rules! minutes_impl {
    ($i:literal) => {
        impl Minutes<$i> {
            const NAME: &str = concat!("Minutes[Length:", $i, "]");

            pub const fn relative(self) -> DaySubdivison<$i> {
                DaySubdivison {
                    index: Minutes::<$i>::first_on_day(self.occurs_on_day()).between(self),
                }
            }

            pub const fn day(self) -> Day {
                self.occurs_on_day()
            }

            pub const fn month(self) -> Month {
                self.occurs_on_day().month()
            }

            pub const fn year(self) -> Year {
                self.occurs_on_day().year()
            }

            const PERIODS_PER_DAY: i32 = MINUTES_PER_DAY / $i as i32;

            pub const fn succ_n(self, n: u16) -> Minutes<$i> {
                Minutes {
                    index: self.index + n as i32,
                }
            }
            pub const fn pred_n(self, n: u16) -> Minutes<$i> {
                Minutes {
                    index: self.index - n as i32,
                }
            }
            pub const fn succ(self) -> Minutes<$i> {
                self.succ_n(1)
            }
            pub const fn pred(self) -> Minutes<$i> {
                self.pred_n(1)
            }
            pub const fn to_monotonic(self) -> i32 {
                self.index
            }
            pub const fn between(self, other: Self) -> i32 {
                other.index - self.index
            }
            pub const fn from_monotonic(index: i32) -> Self {
                Minutes { index }
            }
            pub const fn occurs_on_day(self) -> Day {
                Day::new(Date::new(self.index / Self::PERIODS_PER_DAY))
            }
            pub const fn first_on_day(day: Day) -> Self {
                Self::from_monotonic(day.to_monotonic() * Self::PERIODS_PER_DAY)
            }

            #[cfg(feature = "chrono")]
            pub const fn from_utc_datetime(datetime: DateTime<Utc>) -> Self {
                datetime.into()
            }
            pub const fn start_minute(self) -> Minute {
                todo!()
            }
            pub const fn from_minute(minute: Minute) -> Self {
                minute.change_resolution()
            }
        }

        impl Monotonic for Minutes<$i> {
            fn to_monotonic(self) -> i32 {
                self.to_monotonic()
            }
            fn between(self, other: Self) -> i32 {
                self.between(other)
            }
        }

        impl FromMonotonic for Minutes<$i> {
            fn from_monotonic(index: i32) -> Self {
                Self::from_monotonic(index)
            }
        }

        impl<const N1: u32> Convert<Minutes<N1>> for Minutes<$i> {
            fn convert(self) -> Minutes<N1> {
                todo!()
            }
        }

        impl Convert<Day> for Minutes<$i> {
            fn convert(self) -> Day {
                todo!()
            }
        }
        impl Convert<Month> for Minutes<$i> {
            fn convert(self) -> Month {
                todo!()
            }
        }

        impl Convert<Year> for Minutes<$i> {
            fn convert(self) -> Year {
                todo!()
            }
        }

        impl TimeResolution for Minutes<$i> {
            fn succ_n(self, n: u16) -> Minutes<$i> {
                self.succ_n(n)
            }
            fn pred_n(self, n: u16) -> Minutes<$i> {
                self.pred_n(n)
            }
            #[cfg(feature = "chrono")]
            fn start_datetime(self) -> DateTime<Utc> {
                DateTime::<Utc>::from_timestamp(self.index * NUM_SECS * i64::from($i), 0)
                    .expect("valid timestamp")
            }

            const NAME: &str = Self::NAME;

            fn start_minute(self) -> Minute {
                self.start_minute()
            }

            fn succ(self) -> Self {
                self.succ_n(1)
            }

            fn pred(self) -> Self {
                self.pred_n(1)
            }

            fn convert<Out>(self) -> Out
            where
                Out: TimeResolution + From<crate::Minute>,
            {
                Out::from(self.start_minute())
            }

            fn day(self) -> Day {
                self.day()
            }

            fn month(self) -> Month {
                self.month()
            }

            fn year(self) -> Year {
                self.year()
            }
        }
        impl SubDateResolution for Minutes<$i> {
            fn occurs_on_day(self) -> Day {
                self.occurs_on_day()
            }
            fn first_on_day(day: Day, _params: Self::Params) -> Self {
                Self::first_on_day(day)
            }

            type Params = ();

            fn params(self) -> Self::Params {}

            #[cfg(feature = "chrono")]
            fn from_utc_datetime(datetime: DateTime<Utc>, _params: Self::Params) -> Self {
                self.from_utc_datetime(datetime)
            }

            fn from_minute(minute: Minute, _params: Self::Params) -> Self {
                Self::from_minute(minute)
            }
        }

        impl fmt::Display for Minutes<$i> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let day = self.day();
                let sub = self.relative().index();
                let periods = Self::PERIODS_PER_DAY;

                if Self::PERIODS_PER_DAY < 100 {
                    write!(f, "{day}P{sub:02}/{periods:02}")
                } else {
                    write!(f, "{day}P{sub:03}/{periods:03}")
                }
            }
        }
        impl str::FromStr for Minutes<$i> {
            type Err = Error;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                if !s.is_ascii() {
                    todo!()
                }
                let Some((day, periods)) = s.split_once('P') else {
                    todo!()
                };

                let Some((current, total)) = periods.split_once('/') else {
                    todo!();
                };

                match (day.parse(), current.parse(), total.parse::<i32>()) {
                    (Ok(day), Ok(current), Ok(total)) => {
                        if total != Self::PERIODS_PER_DAY {
                            todo!();
                        }
                        let Some(current) = NonZeroU16::new(current) else {
                            todo!();
                        };

                        let Some(subdivision) = DaySubdivison::<$i>::new(current) else {
                            todo!();
                        };

                        Ok(subdivision.on_date(day))
                    }
                    _ => todo!(),
                }
            }
        }
    };
}

macro_rules! day_subdivision_impl {
    ($i:literal) => {
        // 1
        impl Debug for DaySubdivison<$i> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_struct("DaySubdivison")
                    .field("index", &self.index())
                    .field("length_minutes", &$i)
                    .field("periods", &Self::PERIODS)
                    .finish()
            }
        }

        // 2
        impl DaySubdivison<$i> {
            pub const PERIODS: u16 = 1440 / $i;
            pub const fn on_date(self, date: Day) -> Minutes<$i> {
                Minutes::<$i>::from_monotonic(
                    self.index as i32 + Minutes::<$i>::first_on_day(date).to_monotonic(),
                )
            }
            pub const fn new(period_no: NonZeroU16) -> Option<DaySubdivison<$i>> {
                if period_no.get() > Self::PERIODS as u16 {
                    return None;
                }

                Some(DaySubdivison {
                    index: period_no.get() - 1,
                })
            }
            pub const fn index(self) -> NonZeroU16 {
                match NonZeroU16::new(self.index + 1) {
                    Some(n) => n,
                    None => panic!("Add one to index means it must be non-zero"),
                }
            }
        }
    };
}

minutes_impl!(1);
minutes_impl!(2);
minutes_impl!(3);
minutes_impl!(4);
minutes_impl!(5);
minutes_impl!(6);
minutes_impl!(10);
minutes_impl!(15);
minutes_impl!(20);
minutes_impl!(30);
minutes_impl!(60);
minutes_impl!(120);
minutes_impl!(180);
minutes_impl!(240);
minutes_impl!(360);
minutes_impl!(720);

day_subdivision_impl!(1);
day_subdivision_impl!(2);
day_subdivision_impl!(3);
day_subdivision_impl!(4);
day_subdivision_impl!(5);
day_subdivision_impl!(6);
day_subdivision_impl!(10);
day_subdivision_impl!(15);
day_subdivision_impl!(20);
day_subdivision_impl!(30);
day_subdivision_impl!(60);
day_subdivision_impl!(120);
day_subdivision_impl!(180);
day_subdivision_impl!(240);
day_subdivision_impl!(360);
day_subdivision_impl!(720);

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DaySubdivison<const N: u32> {
    index: u16,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TimeResolution;

    #[test]
    fn test_relative() {
        let base = "2021-01-01 00:00".parse::<Minutes<1>>().unwrap();

        for i in 0..1440 {
            assert_eq!(
                base.succ_n(i).relative(),
                DaySubdivison::<1>::new(NonZeroU16::new(i + 1).unwrap()).unwrap()
            );
            assert_eq!(base.succ_n(i * 1440).relative().index().get(), 1);
            assert_eq!(base.succ_n(i).relative().index().get(), i + 1,);
        }

        let base = "2021-01-01 00:00 => 2021-01-01 00:02"
            .parse::<Minutes<2>>()
            .unwrap();
        for i in 0..720 {
            assert_eq!(
                base.succ_n(i).relative(),
                DaySubdivison::<2>::new(NonZeroU16::new(i + 1).unwrap()).unwrap()
            );
            assert_eq!(base.succ_n(i * 720).relative().index().get(), 1);
            assert_eq!(base.succ_n(i).relative().index().get(), i + 1,);
        }

        let base = "2021-01-01 00:00 => 2021-01-01 00:05"
            .parse::<Minutes<5>>()
            .unwrap();
        for i in 0..288 {
            assert_eq!(
                base.succ_n(i).relative(),
                DaySubdivison::<5>::new(NonZeroU16::new(i + 1).unwrap()).unwrap()
            );
            assert_eq!(base.succ_n(i * 288).relative().index().get(), 1);
            assert_eq!(base.succ_n(i).relative().index().get(), i + 1,);
        }

        let base = "2021-01-01 00:00 => 2021-01-01 00:30"
            .parse::<Minutes<30>>()
            .unwrap();
        for i in 0..48 {
            assert_eq!(
                base.succ_n(i).relative(),
                DaySubdivison::<30>::new(NonZeroU16::new(i + 1).unwrap()).unwrap()
            );
            assert_eq!(base.succ_n(i * 48).relative().index().get(), 1);
            assert_eq!(base.succ_n(i).relative().index().get(), i + 1,);
        }

        let base = "2021-01-01 00:00 => 2021-01-01 01:00"
            .parse::<Minutes<60>>()
            .unwrap();
        for i in 0..24 {
            assert_eq!(
                base.succ_n(i).relative(),
                DaySubdivison::<60>::new(NonZeroU16::new(i + 1).unwrap()).unwrap()
            );
            assert_eq!(base.succ_n(i * 24).relative().index().get(), 1);
            assert_eq!(base.succ_n(i).relative().index().get(), i + 1,);
        }

        let base = "2021-01-01 00:00 => 2021-01-01 02:00"
            .parse::<Minutes<120>>()
            .unwrap();
        for i in 0..12 {
            assert_eq!(
                base.succ_n(i).relative(),
                DaySubdivison::<120>::new(NonZeroU16::new(i + 1).unwrap()).unwrap()
            );
            assert_eq!(base.succ_n(i * 12).relative().index().get(), 1);
            assert_eq!(base.succ_n(i).relative().index().get(), i + 1,);
        }
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_roundtrip() {
        use SubDateResolution;

        let dt = chrono::NaiveDate::from_ymd_opt(2021, 12, 6).unwrap();
        let tm = dt.and_time(NaiveTime::MIN).and_utc();

        let min = Minutes::<1>::from(tm);
        assert!(min.occurs_on_date() == dt);
        assert!(min.start_datetime() == tm);

        let min = Minutes::<2>::from(tm);
        assert!(min.occurs_on_date() == dt);
        assert!(min.start_datetime() == tm);

        let min = Minutes::<3>::from(tm);
        assert!(min.occurs_on_date() == dt);
        assert!(min.start_datetime() == tm);

        let min = Minutes::<4>::from(tm);
        assert!(min.occurs_on_date() == dt);
        assert!(min.start_datetime() == tm);

        let min = Minutes::<5>::from(tm);
        assert!(min.occurs_on_date() == dt);
        assert!(min.start_datetime() == tm);

        assert_eq!(
            min,
            serde_json::from_str(&serde_json::to_string(&min).unwrap()).unwrap()
        )
    }

    #[test]
    fn test_into() {
        assert_eq!(
            Minutes::<2>::from(
                chrono::NaiveDate::from_ymd_opt(2021, 1, 1)
                    .unwrap()
                    .and_hms_opt(10, 2, 0)
                    .unwrap()
                    .and_utc()
            ),
            Minutes::<2>::from(
                chrono::NaiveDate::from_ymd_opt(2021, 1, 1)
                    .unwrap()
                    .and_hms_opt(10, 3, 59)
                    .unwrap()
                    .and_utc()
            ),
        );
    }

    #[test]
    fn test_parse() {
        assert!("2021-01-01 10:05".parse::<Minutes<2>>().is_err());
        assert!("2021-01-01 10:05 => 2021-01-01 10:06"
            .parse::<Minutes<2>>()
            .is_err());
        assert!("2021-01-01 10:02 => 2021-01-01 10:04"
            .parse::<Minutes<2>>()
            .is_ok());

        assert_eq!(
            "2021-01-01 10:05".parse::<Minutes<1>>().unwrap(),
            chrono::NaiveDate::from_ymd_opt(2021, 1, 1)
                .unwrap()
                .and_hms_opt(10, 5, 0)
                .unwrap()
                .and_utc()
                .into(),
        );
        assert_eq!(
            "2021-01-01 10:05".parse::<Minutes<1>>().unwrap().succ(),
            chrono::NaiveDate::from_ymd_opt(2021, 1, 1)
                .unwrap()
                .and_hms_opt(10, 6, 0)
                .unwrap()
                .and_utc()
                .into(),
        );
        assert_eq!(
            "2021-01-01 10:05"
                .parse::<Minutes<1>>()
                .unwrap()
                .succ()
                .pred(),
            chrono::NaiveDate::from_ymd_opt(2021, 1, 1)
                .unwrap()
                .and_hms_opt(10, 5, 0)
                .unwrap()
                .and_utc()
                .into(),
        );

        assert_eq!(
            "2021-01-01 10:02 => 2021-01-01 10:04"
                .parse::<Minutes<2>>()
                .unwrap(),
            chrono::NaiveDate::from_ymd_opt(2021, 1, 1)
                .unwrap()
                .and_hms_opt(10, 2, 0)
                .unwrap()
                .and_utc()
                .into(),
        );

        assert_eq!(
            "2021-01-01 10:00 => 2021-01-01 10:05"
                .parse::<Minutes<5>>()
                .unwrap(),
            chrono::NaiveDate::from_ymd_opt(2021, 1, 1)
                .unwrap()
                .and_hms_opt(10, 0, 0)
                .unwrap()
                .and_utc()
                .into(),
        );
    }
}
