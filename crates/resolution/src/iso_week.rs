use date::Date;

use crate::{minutes::MINUTES_PER_DAY, *};
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IsoWeek(i64);

const MIN: i64 = -521722;
const MAX: i64 = 521722; // TODO

impl IsoWeek {
       pub const MIN: Self = Self(MIN);
    pub const MAX: Self = Self(MAX);
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
}

impl TimeResolution for IsoWeek {
    const NAME: &str = "IsoWeek";

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

impl DateResolution for IsoWeek {
    type Params = ();

    type FromDay = Option<Self>;

    fn params(self) -> Self::Params {
        ()
    }

    fn from_day(_day: Day, _params: Self::Params) -> Self::FromDay {
        todo!()
    }

    fn start_day(self) -> Day {
       todo!()
    }
    fn end_day(self) -> Day {
        todo!()
    }
}

impl Monotonic for IsoWeek {
    fn to_monotonic(self) -> i64 {
        self.to_monotonic()
    }

    fn between(self, other: Self) -> i64 {
        self.between(other)
    }
}

impl FromMonotonic for IsoWeek {
    fn from_monotonic(idx: i64) -> Option<Self> {
        Self::from_monotonic(idx)
    }
}


// #[cfg(test)]
// mod tests {
//     use crate::date_impl::{DayOfMonth, MonthOfYear};

//     use super::*;
//     use crate::DateResolution;

//     #[test]
//     #[cfg(feature = "serde")]
//     fn test_roundtrip() {
//         use crate::DateResolutionExt;

//         let dt = Day::ymd(2021, MonthOfYear::Dec, DayOfMonth::D6);

//         let wk = Week::<Monday>::from_day(dt);
//         assert!(wk.start_day() <= dt && wk.end_day() >= dt);

//         let wk = Week::<Tuesday>::from_day(dt);
//         assert!(wk.start_day() <= dt && wk.end_day() >= dt);

//         let wk = Week::<Wednesday>::from_day(dt);
//         assert!(wk.start_day() <= dt && wk.end_day() >= dt);

//         let wk = Week::<Thursday>::from_day(dt);
//         assert!(wk.start_day() <= dt && wk.end_day() >= dt);

//         let wk = Week::<Friday>::from_day(dt);
//         assert!(wk.start_day() <= dt && wk.end_day() >= dt);

//         let wk = Week::<Saturday>::from_day(dt);
//         assert!(wk.start_day() <= dt && wk.end_day() >= dt);

//         let wk = Week::<Sunday>::from_day(dt);
//         assert!(wk.start_day() <= dt && wk.end_day() >= dt);

//         assert_eq!(
//             wk,
//             serde_json::from_str(&serde_json::to_string(&wk).unwrap()).unwrap()
//         )
//     }
//     #[test]
//     fn test_parse() {
//         assert_eq!(
//             "Week starting 2021-12-06"
//                 .parse::<Week<Monday>>()
//                 .unwrap()
//                 .start(),
//             Day::ymd(2021, MonthOfYear::Dec, DayOfMonth::D6),
//         );
//         assert_eq!(
//             "Week starting 2021-12-06"
//                 .parse::<Week<Monday>>()
//                 .unwrap()
//                 .succ()
//                 .start(),
//             Day::ymd(2021, MonthOfYear::Dec, DayOfMonth::D13),
//         );
//         assert_eq!(
//             "Week starting 2021-12-06"
//                 .parse::<Week<Monday>>()
//                 .unwrap()
//                 .succ()
//                 .pred()
//                 .start(),
//             Day::ymd(2021, MonthOfYear::Dec, DayOfMonth::D6),
//         );

//         assert!("Week starting 2021-12-06".parse::<Week<Tuesday>>().is_err(),);
//         assert!("Week starting 2021-12-06"
//             .parse::<Week<Wednesday>>()
//             .is_err(),);
//         assert!("Week starting 2021-12-06"
//             .parse::<Week<Thursday>>()
//             .is_err(),);
//         assert!("Week starting 2021-12-06".parse::<Week<Friday>>().is_err(),);
//         assert!("Week starting 2021-12-06"
//             .parse::<Week<Saturday>>()
//             .is_err(),);
//         assert!("Week starting 2021-12-06".parse::<Week<Sunday>>().is_err(),);
//     }
// }
