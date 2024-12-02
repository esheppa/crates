use crate::alloc::string::ToString;
use crate::date_impl::MonthOfYear;
use crate::{
    Convert, Day, Error, FromMonotonic, Minute, Monotonic, Month, SubDateResolution,
    TimeResolution, Year,
};
use alloc::{fmt, format, str, string::String};
#[cfg(feature = "chrono")]
use chrono::{DateTime, Datelike, Duration, NaiveDate, NaiveTime, Timelike, Utc};
use core::fmt::Debug;
use core::num::NonZeroU16;

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
pub struct Minutes<const N: u16> {
    index: i32,
}

impl<const N: u16> TryFrom<Minutes_> for Minutes<N> {
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

impl<const N: u16> From<Minutes<N>> for Minutes_ {
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
    pub(crate) length: u16,
}

#[cfg(feature = "chrono")]
impl<const N: u16> From<DateTime<Utc>> for Minutes<N> {
    fn from(d: DateTime<Utc>) -> Self {
        Minutes {
            index: d.timestamp().div_euclid(60 * i32::from(N)),
        }
    }
}
#[derive(Clone, Debug)]

pub struct ParseError {
    pub kind: MinutesParseErrorKind,
    pub raw_data: String,
}

#[derive(Clone, Copy, Debug)]
pub enum MinutesParseErrorKind {
    NonAscii,
    MissingP,
    MissingSlash,
    WrongPeriodsPerDay {
        expected: i32,
        got: i32,
    },
    CurrentPeriodZero,
    InvalidCharacterAtIndex {
        idx: usize,
        char: u8,
    },
    TooLong,
    TooShort,
    InvalidMonth,

    InvalidDay {
        year: i32,
        month: MonthOfYear,
        day: u8,
    },
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

impl<const N: u16> Monotonic for Minutes<N> {
    fn to_monotonic(self) -> i32 {
        self.to_monotonic()
    }
    fn between(self, other: Self) -> i32 {
        self.between(other)
    }
}

impl<const N: u16> FromMonotonic for Minutes<N> {
    fn from_monotonic(index: i32) -> Self {
        Self::from_monotonic(index)
    }
}

impl<const N: u16, const N1: u16> Convert<Minutes<N1>> for Minutes<N> {
    fn convert(self) -> Minutes<N1> {
        self.change_resolution()
    }
}

impl<const N: u16> Convert<Day> for Minutes<N> {
    fn convert(self) -> Day {
        todo!()
    }
}
impl<const N: u16> Convert<Month> for Minutes<N> {
    fn convert(self) -> Month {
        todo!()
    }
}

impl<const N: u16> Convert<Year> for Minutes<N> {
    fn convert(self) -> Year {
        todo!()
    }
}

impl<const N: u16> TimeResolution for Minutes<N> {
    fn succ_n(self, n: u16) -> Minutes<N> {
        self.succ_n(n)
    }
    fn pred_n(self, n: u16) -> Minutes<N> {
        self.pred_n(n)
    }
    #[cfg(feature = "chrono")]
    fn start_datetime(self) -> DateTime<Utc> {
        DateTime::<Utc>::from_timestamp(self.index * NUM_SECS * i64::from(N), 0)
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
impl<const N: u16> SubDateResolution for Minutes<N> {
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

impl<const N: u16> fmt::Display for Minutes<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let day = self.day();
        let sub = self.relative().index();
        let periods = Self::PERIODS_PER_DAY;

        write!(f, "{day}P{sub:04}/{periods:04}")
    }
}
impl<const N: u16> str::FromStr for Minutes<N> {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Self::parse(s) {
            Ok(m) => Ok(m),
            Err(e) => Err(ParseError {
                kind: e,
                raw_data: s.to_string(),
            }),
        }
    }
}

impl<const N: u16> Minutes<N> {
    pub const fn relative(self) -> DaySubdivison<N> {
        let idx = Minutes::<N>::first_on_day(self.occurs_on_day()).between(self);
        assert!(idx >= 0 && idx <= u16::MAX as i32);
        DaySubdivison { index: idx as u16 }
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

    const PERIODS_PER_DAY: i32 = MINUTES_PER_DAY / N as i32;

    pub const fn succ_n(self, n: u16) -> Minutes<N> {
        Minutes {
            index: self.index + n as i32,
        }
    }
    pub const fn pred_n(self, n: u16) -> Minutes<N> {
        Minutes {
            index: self.index - n as i32,
        }
    }
    pub const fn succ(self) -> Minutes<N> {
        self.succ_n(1)
    }
    pub const fn pred(self) -> Minutes<N> {
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
        Day::new(self.index / Self::PERIODS_PER_DAY)
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
            720 => "Minutes[Length:720]",
            _ => panic!("Please choose a minutes impl within 1, 2, 3, 4, 5, 6, 10, 15, 20, 30, 60, 120, 180, 240, 360, 720")
        }
    };

    pub const fn change_resolution<const N2: u16>(self) -> Minutes<N2> {
        // ensures that both N and N2 are sensible...
        const {
            _ = Self::SENSIBLE;
            _ = Minutes::<N2>::SENSIBLE;
        }

        if N2 == N {
            // can't just return self, because compiler doesn't know that N2 == N ...
            Minutes { index: self.index }
        } else if N2 > N {
            // long day subdivions to short
            // clean scaling
            if N2 % N == 0 {
                Minutes {
                    index: self.index / Self::PERIODS_PER_DAY * (MINUTES_PER_DAY / N2 as i32),
                }
            } else {
                panic!("Incompatible minutes when changing resolution")
            }
        } else {
            // short day subdivision to long
            if N % N2 == 0 {
                Minutes {
                    index: self.index / (MINUTES_PER_DAY / N2 as i32) * Self::PERIODS_PER_DAY,
                }
            } else {
                panic!("Incompatible minutes when changing resolution")
            }
        }
    }

    const SENSIBLE: () = {
        let sensible = [
            1, 2, 3, 4, 5, 6, 10, 15, 20, 30, 60, 120, 180, 240, 360, 720,
        ];

        let mut idx = 0;

        loop {
            if idx >= sensible.len() {
                panic!("Please choose a minutes impl within 1, 2, 3, 4, 5, 6, 10, 15, 20, 30, 60, 120, 180, 240, 360, 720")
            }

            if N == sensible[idx] {
                break;
            }

            idx += 1;
        }
    };

    const fn to_str(self) -> [u8; 20] {
        let base = [
            b'0', b'0', b'0', b'0', b'-', b'0', b'0', b'-', b'0', b'0', b'P', b'0', b'0', b'0',
            b'0', b'/', b'0', b'0', b'0', b'0',
        ];

        base
    }
    const fn parse(s: &str) -> Result<Self, MinutesParseErrorKind> {
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

            if idx != 10 && idx != 13 {
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

        let month = {
            ascii_char_to_numeral(bytes[5]).unwrap() as u8 * 10
                + ascii_char_to_numeral(bytes[6]).unwrap() as u8
        };

        let Some(month) = MonthOfYear::from_number(month) else {
            return Err(MinutesParseErrorKind::InvalidMonth);
        };

        let day = {
            ascii_char_to_numeral(bytes[8]).unwrap() as u8 * 10
                + ascii_char_to_numeral(bytes[9]).unwrap() as u8
        };

        if day == 0 || day > month.num_days(year) {
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

        if total as i32 != (MINUTES_PER_DAY / N as i32) {
            return Err(MinutesParseErrorKind::WrongPeriodsPerDay {
                got: total as i32,
                expected: (MINUTES_PER_DAY / N as i32),
            });
        }
        let Some(current) = NonZeroU16::new(current) else {
            return Err(MinutesParseErrorKind::CurrentPeriodZero);
        };

        let Some(subdivision) = DaySubdivison::<N>::new(current) else {
            todo!();
        };

        let day = Year::new(year)
            .with_month(month)
            .first_day()
            .succ_n(day as u16 - 1);

        Ok(subdivision.on_date(day))
    }
}

impl<const N: u16> Debug for DaySubdivison<N> {
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
    pub const fn on_date(self, date: Day) -> Minutes<N> {
        Minutes::<N>::from_monotonic(
            self.index as i32 + Minutes::<N>::first_on_day(date).to_monotonic(),
        )
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

    #[cfg(feature = "chrono")]
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
