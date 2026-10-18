use core::{fmt::Display, num::NonZeroU16};

use date::{
    Date,
    time_of_day::{LocalDateTime, LocalTimeOfDay},
};

use crate::*;

pub type Minute = Minutes<1>;
pub type FiveMinute = Minutes<5>;
pub type HalfHour = Minutes<30>;
pub type Hour = Minutes<60>;

const MIN: i64 = 0;
const MAX: i64 = 5_259_491_999; // TODO
// leap seconds are ignored here

/// Note that for sensible behaviour, the N chosen should be a number that either:
/// 1. divides into an hour with no remainder (1, 2, 3, 4, 5, 6, 10, 12, 15, 20, 30, 60)
/// 2. is exactly a whole number of hours that divides into a day with no remainder (60, 120, 180, 240, 360, 480, 720)
/// Any other choice will result in unexpected / unuseful behaviour (eg the `Minutes` not cleanly fitting into parts of a day)
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Minutes<const N: u16>(i64);
pub(crate) const MINUTES_PER_DAY: i64 = 24 * 60;

#[cfg(feature = "serde")]
impl<'de, const N: u16> Deserialize<'de> for Minutes<N> {
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        s.parse().map_err(serde::de::Error::custom)
    }
}

#[cfg(feature = "serde")]
impl<const N: u16> Serialize for Minutes<N> {
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<const N: u16> FromMinute for Minutes<N> {
    fn from_minute(minute: Minute) -> Self {
        Minutes(minute.0 / Self::PERIODS_PER_DAY)
    }
}

impl<const N: u16> Minutes<N> {
    pub const MIN: Self = Self(MIN);
    pub const MAX: Self = Self(MAX / N as i64);

    const PERIODS_PER_DAY: i64 = MINUTES_PER_DAY / N as i64;
    const SENSIBLE: () = {
        let sensible = [
            1, 2, 3, 4, 5, 6, 10, 15, 20, 30, 60, 120, 180, 240, 360, 480, 720,
        ];

        let mut idx = 0;

        loop {
            if idx >= sensible.len() {
                panic!(
                    "Please choose a minutes impl within 1, 2, 3, 4, 5, 6, 10, 15, 20, 30, 60, 120, 180, 240, 360, 480, 720"
                )
            }

            if N == sensible[idx] {
                break;
            }

            idx += 1;
        }
    };

    pub const fn from_minute(minute: Minute) -> Self {
        Minutes(minute.0 / (N as i64))
    }
    // pub const fn occurs_on_day(self) -> Day {
    //     Day::new(self.index / Self::PERIODS_PER_DAY)
    // }
    pub const fn first_on_day(day: Day) -> Self {
        let Some(x) = day.to_monotonic().checked_mul(Self::PERIODS_PER_DAY) else {
            panic!("TODO");
        };
        Self::from_monotonic(x).expect("TODO")
    }
    pub const fn from_monotonic(idx: i64) -> Option<Self> {
        // TODO: use MIN..=MAX here when it is const
        if idx >= MIN && idx <= MAX {
            Some(Minutes(idx))
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
        Minutes::<1>(self.0 * (N as i64))
    }

    pub const fn end_minute(self) -> Minute {
        Minutes::<1>(self.0 * (N as i64) + (N as i64) - 1)
    }
    const NAME: &str = {
        match N {
            1 => "Minutes[Length:1]",
            2 => "Minutes[Length:2]",
            3 => "Minutes[Length:3]",
            4 => "Minutes[Length:4]",
            5 => "Minutes[Length:5]",
            6 => "Minutes[Length:6]",
            10 => "Minutes[Length:10]",
            15 => "Minutes[Length:15]",
            20 => "Minutes[Length:20]",
            30 => "Minutes[Length:30]",
            60 => "Minutes[Length:60]",
            120 => "Minutes[Length:120]",
            180 => "Minutes[Length:180]",
            240 => "Minutes[Length:240]",
            360 => "Minutes[Length:360]",
            480 => "Minutes[Length:480]",
            720 => "Minutes[Length:720]",
            _ => panic!(
                "Please choose a minutes impl within 1, 2, 3, 4, 5, 6, 10, 15, 20, 30, 60, 120, 180, 240, 360, 720"
            ),
        }
    };

    pub const fn relative(self) -> DaySubdivison<N> {
        let idx = Minutes::<N>::first_on_day(self.occurs_on_day()).between(self);

        const {
            _ = Self::SENSIBLE;
        }

        debug_assert!(idx >= 0 && idx <= 1440);

        DaySubdivison { index: idx as u16 }
    }

    pub const fn day(self) -> Day {
        self.occurs_on_day()
    }

    pub const fn occurs_on_day(self) -> Day {
        Day::from_monotonic(self.0 / Self::PERIODS_PER_DAY).expect("")
    }

    pub const fn from_local_time(local: LocalDateTime) -> Option<Self> {
        let Some(through_day) = (local.time().hour().number() as i64)
            .checked_add(60 * (local.time().minute().number() as i64))
        else {
            return None;
        };

        let Some(x) = Day::from_date(local.day()) else {
            return None;
        };

        let Some(x) = x.translate(through_day) else {
            return None;
        };

        Some(Self::from_minute(Minute::first_on_day(x)))
    }

    pub const fn local_time(self) -> Option<LocalDateTime> {
        // subtract N at the end to get the minutes at the _start_ of the period
        let total_minutes = self.relative().index().get() * N - N;

        let Some(x) = LocalTimeOfDay::from_total_minutes(total_minutes) else {
            return None;
        };

        Some(LocalDateTime::new(self.day().date(), x))
    }

    // // TODO...
    // const fn to_str(self) -> [u8; 20] {
    //     let base = [
    //         b'0', b'0', b'0', b'0', b'-', b'0', b'0', b'-', b'0', b'0', b'P', b'0', b'0', b'0',
    //         b'0', b'/', b'0', b'0', b'0', b'0',
    //     ];

    //     base
    // }
    const fn parse(s: &str) -> core::result::Result<Self, MinutesParseErrorKind> {
        if !s.is_ascii() {
            return Err(MinutesParseErrorKind::NonAscii);
        }

        let bytes = s.as_bytes();

        if bytes.len() > 20 {
            return Err(MinutesParseErrorKind::TooLong);
        }
        if bytes.len() < 20 {
            return Err(MinutesParseErrorKind::TooShort);
        }
        if bytes[10] != b'P' {
            return Err(MinutesParseErrorKind::MissingP);
        }
        if bytes[15] != b'/' {
            return Err(MinutesParseErrorKind::MissingSlash);
        }

        let mut idx = 0;

        loop {
            if idx >= bytes.len() {
                break;
            }

            if idx != 4 && idx != 7 && idx != 10 && idx != 15 {
                if ascii_char_to_numeral(bytes[idx]).is_none() {
                    return Err(MinutesParseErrorKind::InvalidCharacterAtIndex {
                        idx,
                        char: bytes[idx],
                    });
                }
            }

            idx += 1;
        }

        let year = {
            ascii_char_to_numeral(bytes[0]).unwrap() as i32 * 1000
                + ascii_char_to_numeral(bytes[1]).unwrap() as i32 * 100
                + ascii_char_to_numeral(bytes[2]).unwrap() as i32 * 10
                + ascii_char_to_numeral(bytes[3]).unwrap() as i32
        };

        let year = date::Year::new(year);

        let month = {
            ascii_char_to_numeral(bytes[5]).unwrap() as u8 * 10
                + ascii_char_to_numeral(bytes[6]).unwrap() as u8
        };

        let Some(month) = MonthOfYear::from_number(month) else {
            return Err(MinutesParseErrorKind::InvalidMonth);
        };

        let day = {
            ascii_char_to_numeral(bytes[8]).unwrap() as u16 * 10
                + ascii_char_to_numeral(bytes[9]).unwrap() as u16
        };

        if day == 0 || day > (month.num_days(year) as u16) {
            return Err(MinutesParseErrorKind::InvalidDay { year, month, day });
        }

        let current = {
            ascii_char_to_numeral(bytes[11]).unwrap() as u16 * 1000
                + ascii_char_to_numeral(bytes[12]).unwrap() as u16 * 100
                + ascii_char_to_numeral(bytes[13]).unwrap() as u16 * 10
                + ascii_char_to_numeral(bytes[14]).unwrap() as u16
        };

        let total = {
            ascii_char_to_numeral(bytes[16]).unwrap() as u16 * 1000
                + ascii_char_to_numeral(bytes[17]).unwrap() as u16 * 100
                + ascii_char_to_numeral(bytes[18]).unwrap() as u16 * 10
                + ascii_char_to_numeral(bytes[19]).unwrap() as u16
        };

        if total as i64 != (MINUTES_PER_DAY / N as i64) {
            return Err(MinutesParseErrorKind::WrongPeriodsPerDay {
                got: total as i64,
                expected: (MINUTES_PER_DAY / N as i64),
            });
        }
        let Some(current) = NonZeroU16::new(current) else {
            return Err(MinutesParseErrorKind::CurrentPeriodZero);
        };

        let Some(subdivision) = DaySubdivison::<N>::new(current) else {
            return Err(MinutesParseErrorKind::InvalidSubdivision {
                length: N,
                value: current,
            });
        };

        let Some(date) = Date::first_on_month(year, month) else {
            return Err(MinutesParseErrorKind::CantCreateFirstOnMonth { year, month });
        };

        let Some(date) = date.translate(day.saturating_sub(1) as i32) else {
            return Err(MinutesParseErrorKind::CantAddDays {
                date,
                days: day.saturating_sub(1),
            });
        };

        let Some(day) = Day::from_date(date) else {
            return Err(MinutesParseErrorKind::InvalidDate(date));
        };

        let Some(minutes) = subdivision.on_date(day) else {
            return Err(MinutesParseErrorKind::CantCreateOnDate {
                date,
                length: N,
                value: subdivision.index(),
            });
        };

        Ok(minutes)
    }
}

impl<const N: u16> TimeResolution for Minutes<N> {
    const NAME: &str = Self::NAME;
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

impl<const N: u16> Monotonic for Minutes<N> {
    fn to_monotonic(self) -> i64 {
        self.to_monotonic()
    }

    fn between(self, other: Self) -> i64 {
        self.between(other)
    }
}

impl<const N: u16> FromMonotonic for Minutes<N> {
    fn from_monotonic(idx: i64) -> Option<Self> {
        Self::from_monotonic(idx)
    }
}

impl<const N: u16> SubDateResolution for Minutes<N> {
    type Params = ();

    fn params(self) -> Self::Params {
        ()
    }

    // TODO: test
    fn occurs_on_day(self) -> Day {
        self.occurs_on_day()
    }

    fn from_minute(minute: Minute, _params: Self::Params) -> Self {
        Self::from_minute(minute)
    }

    fn first_on_day(day: Day, _params: Self::Params) -> Self {
        Self::first_on_day(day)
    }
    fn last_on_day(day: Day, _params: Self::Params) -> Self {
        Self::first_on_day(day.succ().expect("TODO"))
            .pred()
            .expect("TODO")
    }
}

impl<const N: u16> fmt::Display for Minutes<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let day = self.occurs_on_day();
        let sub = self.relative().index();
        let periods = Self::PERIODS_PER_DAY;

        write!(f, "{day}P{sub:04}/{periods:04}")
    }
}

impl<const N: u16> str::FromStr for Minutes<N> {
    type Err = ParseError;
    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        match Self::parse(s) {
            Ok(m) => Ok(m),
            Err(e) => Err(ParseError {
                kind: e,
                raw_data: s.to_string(),
            }),
        }
    }
}

#[derive(Clone, Debug)]

pub struct ParseError {
    pub kind: MinutesParseErrorKind,
    pub raw_data: String,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Failed to parse Minute from input of `{}` due to: {:?}",
            self.raw_data, self.kind
        )
    }
}

impl error::Error for ParseError {}

#[derive(Clone, Copy, Debug)]
pub enum MinutesParseErrorKind {
    InvalidSubdivision {
        length: u16,
        value: NonZeroU16,
    },
    NonAscii,
    MissingP,
    MissingSlash,
    WrongPeriodsPerDay {
        expected: i64,
        got: i64,
    },
    CurrentPeriodZero,
    InvalidCharacterAtIndex {
        idx: usize,
        char: u8,
    },
    CantCreateFirstOnMonth {
        year: date::Year,
        month: MonthOfYear,
    },
    CantCreateOnDate {
        date: Date,
        length: u16,
        value: NonZeroU16,
    },
    TooLong,
    TooShort,
    InvalidMonth,
    CantAddDays {
        date: Date,
        days: u16,
    },
    InvalidDay {
        year: date::Year,
        month: MonthOfYear,
        day: u16,
    },
    InvalidYear(i32),
    InvalidDate(Date),
}

const fn ascii_char_to_numeral(ch: u8) -> Option<u8> {
    match ch {
        b'0' => Some(0),
        b'1' => Some(1),
        b'2' => Some(2),
        b'3' => Some(3),
        b'4' => Some(4),
        b'5' => Some(5),
        b'6' => Some(6),
        b'7' => Some(7),
        b'8' => Some(8),
        b'9' => Some(9),
        _ => None,
    }
}

impl<const N: u16> fmt::Debug for DaySubdivison<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DaySubdivison")
            .field("index", &self.index())
            .field("length_minutes", &N)
            .field("periods", &Self::PERIODS)
            .finish()
    }
}

impl<const N: u16> DaySubdivison<N> {
    pub const PERIODS: u16 = 1440 / N;
    pub const fn on_date(self, date: Day) -> Option<Minutes<N>> {
        let x = Minutes::<N>::first_on_day(date).to_monotonic();
        let Some(res) = (self.index as i64).checked_add(x) else {
            return None;
        };

        Minutes::<N>::from_monotonic(res)
    }
    pub const fn new(period_no: NonZeroU16) -> Option<DaySubdivison<N>> {
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

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DaySubdivison<const N: u16> {
    index: u16,
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate std;

    use rayon::iter::{IntoParallelIterator, ParallelIterator};

    #[test]
    fn test_relative() {
        let base = "2021-01-01P0001/1440".parse::<Minutes<1>>().unwrap();

        for i in 0..1440 {
            assert_eq!(
                base.translate(i).unwrap().relative(),
                DaySubdivison::<1>::new(NonZeroU16::new(u16::try_from(i).unwrap() + 1).unwrap())
                    .unwrap()
            );
            assert_eq!(
                base.translate(i * 1440).unwrap().relative().index().get(),
                1
            );
            assert_eq!(
                base.translate(i).unwrap().relative().index().get() as i64,
                i + 1,
            );
        }

        let base = "2021-01-01P0001/0720".parse::<Minutes<2>>().unwrap();
        for i in 0..720 {
            assert_eq!(
                base.translate(i).unwrap().relative(),
                DaySubdivison::<2>::new(NonZeroU16::new(u16::try_from(i).unwrap() + 1).unwrap())
                    .unwrap()
            );
            assert_eq!(base.translate(i * 720).unwrap().relative().index().get(), 1);
            assert_eq!(
                base.translate(i).unwrap().relative().index().get() as i64,
                i + 1,
            );
        }

        let base = "2021-01-01P0001/0288".parse::<Minutes<5>>().unwrap();
        for i in 0..288 {
            assert_eq!(
                base.translate(i).unwrap().relative(),
                DaySubdivison::<5>::new(NonZeroU16::new(u16::try_from(i).unwrap() + 1).unwrap())
                    .unwrap()
            );
            assert_eq!(base.translate(i * 288).unwrap().relative().index().get(), 1);
            assert_eq!(
                base.translate(i).unwrap().relative().index().get() as i64,
                i + 1,
            );
        }

        let base = "2021-01-01P0001/0048".parse::<Minutes<30>>().unwrap();
        for i in 0..48 {
            assert_eq!(
                base.translate(i).unwrap().relative(),
                DaySubdivison::<30>::new(NonZeroU16::new(u16::try_from(i).unwrap() + 1).unwrap())
                    .unwrap()
            );
            assert_eq!(base.translate(i * 48).unwrap().relative().index().get(), 1);
            assert_eq!(
                base.translate(i).unwrap().relative().index().get() as i64,
                i + 1,
            );
        }

        let base = "2021-01-01P0001/0024".parse::<Minutes<60>>().unwrap();
        for i in 0..24 {
            assert_eq!(
                base.translate(i).unwrap().relative(),
                DaySubdivison::<60>::new(NonZeroU16::new(u16::try_from(i).unwrap() + 1).unwrap())
                    .unwrap()
            );
            assert_eq!(base.translate(i * 24).unwrap().relative().index().get(), 1);
            assert_eq!(
                base.translate(i).unwrap().relative().index().get() as i64,
                i + 1,
            );
        }

        let base = "2021-01-01P0001/0012".parse::<Minutes<120>>().unwrap();
        for i in 0..12 {
            assert_eq!(
                base.translate(i).unwrap().relative(),
                DaySubdivison::<120>::new(NonZeroU16::new(u16::try_from(i).unwrap() + 1).unwrap())
                    .unwrap()
            );
            assert_eq!(base.translate(i * 12).unwrap().relative().index().get(), 1);
            assert_eq!(
                base.translate(i).unwrap().relative().index().get() as i64,
                i + 1,
            );
        }
    }

    #[test]
    fn min_max_year_roundtrip_ok_all() {
        min_max_year_roundtrip_ok::<1>();
        min_max_year_roundtrip_ok::<2>();
        min_max_year_roundtrip_ok::<3>();
        min_max_year_roundtrip_ok::<4>();
        min_max_year_roundtrip_ok::<5>();
        min_max_year_roundtrip_ok::<6>();
        min_max_year_roundtrip_ok::<10>();
        min_max_year_roundtrip_ok::<15>();
        min_max_year_roundtrip_ok::<20>();
        min_max_year_roundtrip_ok::<30>();
        min_max_year_roundtrip_ok::<60>();
        min_max_year_roundtrip_ok::<120>();
        min_max_year_roundtrip_ok::<180>();
        min_max_year_roundtrip_ok::<240>();
        min_max_year_roundtrip_ok::<360>();
        min_max_year_roundtrip_ok::<480>();
        min_max_year_roundtrip_ok::<720>();
    }

    fn min_max_year_roundtrip_ok<const N: u16>() {
        assert!(Year::MIN.start_minute().pred().is_none());
        assert_eq!(Year::MIN.start_minute(), Minute::MIN);
        assert_eq!(Year::MAX.end_minute(), Minute::MAX);
        assert!(Year::MAX.end_minute().succ().is_none());
    }

    #[test]
    fn exhaustive_all() {
        exhaustive::<1>();
        exhaustive::<2>();
        exhaustive::<3>();
        exhaustive::<4>();
        exhaustive::<5>();
        exhaustive::<6>();
        exhaustive::<10>();
        exhaustive::<15>();
        exhaustive::<20>();
        exhaustive::<30>();
        exhaustive::<60>();
        exhaustive::<120>();
        exhaustive::<180>();
        exhaustive::<240>();
        exhaustive::<360>();
        exhaustive::<480>();
        exhaustive::<720>();
    }
    fn exhaustive<const N: u16>() {
        (MIN..=MAX).into_par_iter().for_each(|i| {
            let y = Minute::from_monotonic(i).unwrap();
            assert_eq!(Minute::MIN.translate(i).unwrap(), y);
            assert_eq!(Minute::from_minute(y.start_minute(), ()), y);
            assert_eq!(Minute::from_minute(y.end_minute(), ()), y);
            _ = y.start_minute();

            _ = y.end_minute();
        });
    }

    // #[cfg(all(feature = "serde", feature = "chrono"))]
    // #[test]
    // fn test_roundtrip() {
    //     let dt = chrono::NaiveDate::from_ymd_opt(2021, 12, 6).unwrap();
    //     let tm = dt.and_time(chrono::NaiveTime::MIN).and_utc();

    //     let min = Minutes::<1>::from_utc_datetime(tm);
    //     assert!(min.occurs_on_day().chrono_date() == dt);
    //     assert!(min.start_datetime() == tm);

    //     let min = Minutes::<2>::from_utc_datetime(tm);
    //     assert!(min.occurs_on_day().chrono_date() == dt);
    //     assert!(min.start_datetime() == tm);

    //     let min = Minutes::<3>::from_utc_datetime(tm);
    //     assert!(min.occurs_on_day().chrono_date() == dt);
    //     assert!(min.start_datetime() == tm);

    //     let min = Minutes::<4>::from_utc_datetime(tm);
    //     assert!(min.occurs_on_day().chrono_date() == dt);
    //     assert!(min.start_datetime() == tm);

    //     let min = Minutes::<5>::from_utc_datetime(tm);
    //     assert!(min.occurs_on_day().chrono_date() == dt);
    //     assert!(min.start_datetime() == tm);

    //     assert_eq!(
    //         min,
    //         serde_json::from_str(&serde_json::to_string(&min).unwrap()).unwrap()
    //     )
    // }

    // #[cfg(feature = "chrono")]
    // #[test]
    // fn test_into() {
    //     assert_eq!(
    //         Minutes::<2>::from_utc_datetime(
    //             chrono::NaiveDate::from_ymd_opt(2021, 1, 1)
    //                 .unwrap()
    //                 .and_hms_opt(10, 2, 0)
    //                 .unwrap()
    //                 .and_utc()
    //         ),
    //         Minutes::<2>::from_utc_datetime(
    //             chrono::NaiveDate::from_ymd_opt(2021, 1, 1)
    //                 .unwrap()
    //                 .and_hms_opt(10, 3, 59)
    //                 .unwrap()
    //                 .and_utc()
    //         ),
    //     );
    // }

    // #[test]
    // #[cfg(feature = "chrono")]
    // fn test_parse() {
    //     assert!("2021-01-01 10:05".parse::<Minutes<2>>().is_err());
    //     assert!(
    //         "2021-01-01 10:05 => 2021-01-01 10:06"
    //             .parse::<Minutes<2>>()
    //             .is_err()
    //     );
    //     assert!(
    //         "2021-01-01 10:02 => 2021-01-01 10:04"
    //             .parse::<Minutes<2>>()
    //             .is_ok()
    //     );

    //     assert_eq!(
    //         "2021-01-01 10:05".parse::<Minutes<1>>().unwrap(),
    //         Minutes::<1>::from_utc_datetime(
    //             chrono::NaiveDate::from_ymd_opt(2021, 1, 1)
    //                 .unwrap()
    //                 .and_hms_opt(10, 5, 0)
    //                 .unwrap()
    //                 .and_utc()
    //         ),
    //     );
    //     assert_eq!(
    //         "2021-01-01 10:05".parse::<Minutes<1>>().unwrap().succ(),
    //         Minutes::<1>::from_utc_datetime(
    //             chrono::NaiveDate::from_ymd_opt(2021, 1, 1)
    //                 .unwrap()
    //                 .and_hms_opt(10, 6, 0)
    //                 .unwrap()
    //                 .and_utc()
    //         ),
    //     );
    //     assert_eq!(
    //         "2021-01-01 10:05"
    //             .parse::<Minutes<1>>()
    //             .unwrap()
    //             .succ()
    //             .pred(),
    //         Minutes::<1>::from_utc_datetime(
    //             chrono::NaiveDate::from_ymd_opt(2021, 1, 1)
    //                 .unwrap()
    //                 .and_hms_opt(10, 5, 0)
    //                 .unwrap()
    //                 .and_utc()
    //         ),
    //     );

    //     assert_eq!(
    //         "2021-01-01 10:02 => 2021-01-01 10:04"
    //             .parse::<Minutes<2>>()
    //             .unwrap(),
    //         Minutes::<2>::from_utc_datetime(
    //             chrono::NaiveDate::from_ymd_opt(2021, 1, 1)
    //                 .unwrap()
    //                 .and_hms_opt(10, 2, 0)
    //                 .unwrap()
    //                 .and_utc()
    //         ),
    //     );

    //     assert_eq!(
    //         "2021-01-01 10:00 => 2021-01-01 10:05"
    //             .parse::<Minutes<5>>()
    //             .unwrap(),
    //         Minutes::<5>::from_utc_datetime(
    //             chrono::NaiveDate::from_ymd_opt(2021, 1, 1)
    //                 .unwrap()
    //                 .and_hms_opt(10, 0, 0)
    //                 .unwrap()
    //                 .and_utc()
    //         ),
    //     );
    // }
}
