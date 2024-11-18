use alloc::{fmt, str};
use alloc::{format, string::String};
use chrono::{DateTime, Datelike, NaiveDate, NaiveTime, Utc};
use core::marker;

use crate::{DateResolution, FromMonotonic};

mod private {
    pub trait Sealed {}
    impl Sealed for super::Monday {}
    impl Sealed for super::Tuesday {}
    impl Sealed for super::Wednesday {}
    impl Sealed for super::Thursday {}
    impl Sealed for super::Friday {}
    impl Sealed for super::Saturday {}
    impl Sealed for super::Sunday {}
}

pub trait StartDay:
    private::Sealed
    + Send
    + Sync
    + 'static
    + Copy
    + Clone
    + fmt::Debug
    + PartialEq
    + Eq
    + PartialOrd
    + Ord
{
    const NAME: &'static str;
    fn weekday() -> Weekday;
}

pub enum Weekday {
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
    Sun,
}

impl Weekday {
    fn name(&self) -> &'static str {
        match self {
            Weekday::Mon => "Monday",
            Weekday::Tue => "Tuesday",
            Weekday::Wed => "Wednesday",
            Weekday::Thu => "Thursday",
            Weekday::Fri => "Friday",
            Weekday::Sat => "Saturday",
            Weekday::Sun => "Sunday",
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Monday;
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tuesday;
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Wednesday;
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Thursday;
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Friday;
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Saturday;
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sunday;

impl StartDay for Monday {
    const NAME: &'static str = "Monday";
    fn weekday() -> Weekday {
        Weekday::Mon
    }
}
impl StartDay for Tuesday {
    const NAME: &'static str = "Tuesday";
    fn weekday() -> Weekday {
        Weekday::Tue
    }
}
impl StartDay for Wednesday {
    const NAME: &'static str = "Wednesday";
    fn weekday() -> Weekday {
        Weekday::Wed
    }
}
impl StartDay for Thursday {
    const NAME: &'static str = "Thursday";
    fn weekday() -> Weekday {
        Weekday::Thu
    }
}
impl StartDay for Friday {
    const NAME: &'static str = "Friday";
    fn weekday() -> Weekday {
        Weekday::Fri
    }
}
impl StartDay for Saturday {
    const NAME: &'static str = "Saturday";
    fn weekday() -> Weekday {
        Weekday::Sat
    }
}
impl StartDay for Sunday {
    const NAME: &'static str = "Sunday";
    fn weekday() -> Weekday {
        Weekday::Sun
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialOrd, PartialEq, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(try_from = "Week_", into = "Week_"))]
pub struct Week<D: StartDay> {
    n: i64,
    d: marker::PhantomData<D>,
}

#[cfg(feature = "serde")]
impl<D: StartDay> TryFrom<Week_> for Week<D> {
    type Error = String;
    fn try_from(value: Week_) -> Result<Self, Self::Error> {
        if value.start_day == D::NAME {
            Ok(Week::from_monotonic(value.n))
        } else {
            Err(format!(
                "To create a Week<{}>, the start_day field should be {} but was instead {}",
                D::NAME,
                D::NAME,
                value.start_day
            ))
        }
    }
}

#[cfg(feature = "serde")]
impl<D: StartDay> From<Week<D>> for Week_ {
    fn from(w: Week<D>) -> Self {
        use alloc::string::ToString;
        Week_ {
            n: w.n,
            start_day: D::NAME.to_string(),
        }
    }
}

#[cfg(feature = "serde")]
#[derive(serde::Deserialize, serde::Serialize)]
struct Week_ {
    n: i64,
    start_day: String,
}

impl<D: StartDay> fmt::Display for Week<D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Week starting {}", crate::DateResolution::start(self))
    }
}

fn base(wd: chrono::Weekday) -> chrono::NaiveDate {
    chrono::NaiveDate::from_ymd_opt(2021, 1, 4 + wd.num_days_from_monday()).expect("valid date")
}

impl<D: StartDay> Week<D> {
    pub fn new(date: NaiveDate) -> Self {
        date.into()
    }

    pub const fn succ(&self) -> Week<D> {
        self.succ_n(1)
    }
    pub const fn pred(&self) -> Week<D> {
        self.pred_n(1)
    }
}

impl<D: StartDay> From<NaiveDate> for Week<D> {
    fn from(value: NaiveDate) -> Week<D> {
        Week::<D>::from_day(value, ())
    }
}

impl<D: StartDay> str::FromStr for Week<D> {
    type Err = crate::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 24 {
            return Err(crate::Error::UnexpectedInputLength {
                actual: s.len(),
                required: 24,
                format: "Week starting %Y-%m-%d",
            });
        }
        let date = chrono::NaiveDate::parse_from_str(&s[14..24], "%Y-%m-%d")?;
        if date.weekday() != D::weekday() {
            return Err(crate::Error::UnexpectedStartDate {
                date,
                actual: date.weekday(),
                required: D::weekday(),
            });
        };

        let week_num = (date - base(D::weekday())).num_days() / 7;

        Ok(Week::from_monotonic(week_num))
    }
}

impl<D: StartDay> DateResolution for Week<D> {
    fn start(&self) -> chrono::NaiveDate {
        base(D::weekday()) + chrono::Duration::days(self.n * 7)
    }
    type Params = ();

    fn params(&self) -> Self::Params {}

    fn from_day(date: NaiveDate, _params: Self::Params) -> Self {
        let week_num = (date - base(D::weekday())).num_days() / 7;

        Week::from_monotonic(week_num)
    }
}

impl<D: StartDay> crate::TimeResolution for Week<D> {
    fn succ_n(&self, n: u32) -> Week<D> {
        Week::from_monotonic(self.n + i64::try_from(n).unwrap())
    }
    fn pred_n(&self, n: u32) -> Week<D> {
        Week::from_monotonic(self.n - i64::try_from(n).unwrap())
    }
    fn start_datetime(&self) -> DateTime<Utc> {
        crate::DateResolution::start(self)
            .and_time(NaiveTime::MIN)
            .and_utc()
    }
    fn name(&self) -> String {
        format!("Week[StartDay:{}]", D::NAME)
    }
}

impl<D: StartDay> crate::Monotonic for Week<D> {
    fn to_monotonic(&self) -> i64 {
        self.n
    }
    fn between(&self, other: Self) -> i64 {
        other.n - self.n
    }
}

impl<D: StartDay> crate::FromMonotonic for Week<D> {
    fn from_monotonic(idx: i64) -> Self {
        Week {
            n: idx,
            d: marker::PhantomData,
        }
    }
}

impl<D: StartDay> From<DateTime<Utc>> for Week<D> {
    fn from(date: DateTime<Utc>) -> Self {
        date.date_naive().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DateResolution, TimeResolution};

    #[test]
    #[cfg(feature = "serde")]
    fn test_roundtrip() {
        use crate::DateResolutionExt;

        let dt = chrono::NaiveDate::from_ymd_opt(2021, 12, 6).expect("valid date");

        let wk = Week::<Monday>::from(dt);
        assert!(wk.start() <= dt && wk.end() >= dt);

        let wk = Week::<Tuesday>::from(dt);
        assert!(wk.start() <= dt && wk.end() >= dt);

        let wk = Week::<Wednesday>::from(dt);
        assert!(wk.start() <= dt && wk.end() >= dt);

        let wk = Week::<Thursday>::from(dt);
        assert!(wk.start() <= dt && wk.end() >= dt);

        let wk = Week::<Friday>::from(dt);
        assert!(wk.start() <= dt && wk.end() >= dt);

        let wk = Week::<Saturday>::from(dt);
        assert!(wk.start() <= dt && wk.end() >= dt);

        let wk = Week::<Sunday>::from(dt);
        assert!(wk.start() <= dt && wk.end() >= dt);

        assert_eq!(
            wk,
            serde_json::from_str(&serde_json::to_string(&wk).unwrap()).unwrap()
        )
    }
    #[test]
    fn test_parse() {
        assert_eq!(
            "Week starting 2021-12-06"
                .parse::<Week<Monday>>()
                .unwrap()
                .start(),
            chrono::NaiveDate::from_ymd_opt(2021, 12, 6).unwrap(),
        );
        assert_eq!(
            "Week starting 2021-12-06"
                .parse::<Week<Monday>>()
                .unwrap()
                .succ()
                .start(),
            chrono::NaiveDate::from_ymd_opt(2021, 12, 13).unwrap(),
        );
        assert_eq!(
            "Week starting 2021-12-06"
                .parse::<Week<Monday>>()
                .unwrap()
                .succ()
                .pred()
                .start(),
            chrono::NaiveDate::from_ymd_opt(2021, 12, 6).unwrap(),
        );

        assert!("Week starting 2021-12-06".parse::<Week<Tuesday>>().is_err(),);
        assert!("Week starting 2021-12-06"
            .parse::<Week<Wednesday>>()
            .is_err(),);
        assert!("Week starting 2021-12-06"
            .parse::<Week<Thursday>>()
            .is_err(),);
        assert!("Week starting 2021-12-06".parse::<Week<Friday>>().is_err(),);
        assert!("Week starting 2021-12-06"
            .parse::<Week<Saturday>>()
            .is_err(),);
        assert!("Week starting 2021-12-06".parse::<Week<Sunday>>().is_err(),);
    }
}
