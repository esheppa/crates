use core::marker::PhantomData;

use crate::*;
// #[cfg(feature = "chrono")]
// use crate::{FixedTimeZone, Zoned};Vec

#[cfg(feature = "chrono")]
use chrono::{DateTime, Utc};

use crate::prelude::*;
use iter::FusedIterator;
use num::NonZeroU64;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
struct RangeSerialize {
    // todo
}

impl<P> TryFrom<RangeSerialize> for TimeRange<P> {
    type Error = String;

    fn try_from(value: RangeSerialize) -> core::result::Result<Self, Self::Error> {
        todo!()
    }
}

impl<P> From<TimeRange<P>> for RangeSerialize {
    fn from(value: TimeRange<P>) -> Self {
        todo!()
    }
}
// #[cfg_attr(
//     feature = "serde",
//     serde(bound(deserialize = "P: de::DeserializeOwned"))
// )]

// the `Step` trait may be interesting later
// https://doc.rust-lang.org/std/iter/trait.Step.html
/// `TimeRange` stores a contigious sequence of underlying periods of a given `TimeResolution`.
///
/// This is useful to represent the time axis of a timeseries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(
    feature = "serde",
    serde(
        bound = "P: Clone",
        try_from = "RangeSerialize",
        into = "RangeSerialize"
    )
)]
pub struct TimeRange<P> {
    range: LocalRange,
    ty: PhantomData<P>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct LocalRange {
    start: i64,
    end: i64,
}

impl LocalRange {
    fn range(self) -> RangeInclusive<i64> {
        RangeInclusive::new(self.start, self.end)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeRangeComparison {
    Superset,
    Subset,
    Earlier,
    Later,
}

impl<P> TimeRange<P>
where
    P: TimeResolution + FromMonotonic,
{
    pub fn start(self) -> P {
        P::from_monotonic(self.range.start).unwrap()
    }
    pub fn end(self) -> P {
        P::from_monotonic(self.range.end).unwrap()
    }
}

impl<P: SubDateResolution> TimeRange<P> {}

impl<P> TimeRange<P> {
    pub fn to_sub_date_resolution<S>(&self) -> TimeRange<S>
    where
        S: SubDateResolution<Params = P::Params> + FromMonotonic,
        P: DateResolution<FromDay = P> + FromMonotonic,
    {
        // get first start
        let first_start = S::first_on_day(self.start().start_day(), self.start().params());
        // get last end
        let last_end = S::last_on_day(self.end().end_day(), self.end().params());
        // do from_start_end and expect it
        TimeRange::from_bounds(first_start, last_end)
    }
}

// impl<P: TimeResolution + FromMonotonic> TimeRange<P> {
//     pub fn from_map(map: collections::BTreeSet<i32>) -> Vec<TimeRange<P>> {
//         let mut ranges = Vec::new();
//         if map.is_empty() {
//             return ranges;
//         }

//         let mut iter = map.into_iter();

//         let mut prev = match iter.next() {
//             Some(n) => n,
//             None => return ranges,
//         };
//         let mut current_range = TimeRange {
//             start: P::from_monotonic(prev),
//             len: num::NonZeroU64::new(1).unwrap(),
//         };
//         for val in iter {
//             if val == prev + 1 {
//                 current_range.len =
//                     num::NonZeroU64::new(current_range.len.get().saturating_add(1)).unwrap();
//             } else {
//                 let mut old_range = TimeRange {
//                     start: P::from_monotonic(val),
//                     len: num::NonZeroU64::new(1).unwrap(),
//                 };
//                 mem::swap(&mut current_range, &mut old_range);
//                 if !ranges.contains(&old_range) {
//                     ranges.push(old_range);
//                 }
//             }

//             prev = val;
//         }

//         ranges
//     }
// }

impl<P: TimeResolution + Monotonic + FromMonotonic> TimeRange<P> {
    pub fn iter_indexes(&self) -> impl Iterator<Item = i64> {
        self.range.range().into_iter()
    }
    pub fn index_of(&self, point: P) -> Option<usize> {
        if point < self.start() || point > self.end() {
            None
        } else {
            Some(
                usize::try_from(self.start().between(point))
                    .expect("Point is earlier than end so this is always ok"),
            )
        }
    }
    pub fn from_bounds(a: P, b: P) -> TimeRange<P> {
        TimeRange {
            range: LocalRange {
                start: a.to_monotonic().min(b.to_monotonic()),
                end: a.to_monotonic().max(b.to_monotonic()),
            },
            ty: PhantomData,
        }
    }

    pub fn len(&self) -> NonZeroU64 {
        NonZeroU64::new(
            self.end()
                .to_monotonic()
                .sub(self.start().to_monotonic())
                .add(1)
                .try_into()
                .unwrap(),
        )
        .unwrap()
    }

    pub fn intersection(&self, other: &TimeRange<P>) -> Option<TimeRange<P>> {
        let max_start = self.range.start.max(other.range.start);
        let min_end = self.range.end.min(other.range.end);

        if max_start > min_end {
            return None;
        }

        Some(TimeRange {
            range: LocalRange {
                start: max_start,
                end: min_end,
            },
            ty: PhantomData,
        })
    }

    pub fn union(&self, other: &TimeRange<P>) -> Option<TimeRange<P>> {
        if self.intersection(other).is_none() {
            return None;
        }

        let min_start = self.range.start.min(other.range.start);
        let max_end = self.range.end.max(other.range.end);

        Some(TimeRange {
            range: LocalRange {
                start: min_start,
                end: max_end,
            },
            ty: PhantomData,
        })
    }

    // pub fn subtract(&self, other: &TimeRange<P>) -> (Option<TimeRange<P>>, Option<TimeRange<P>>) {
    //     (
    //         {

    //             Some(TimeRange::from_bounds(self.start(), other.start().pred().min(self.end())))
    //         },
    //         {
    //             Some(TimeRange::from_bounds(other.end().succ().max(self.start()), self.end()))
    //         },
    //     )
    // }

    // pub fn compare(&self, other: &TimeRange<P>) -> TimeRangeComparison {
    //     match self.subtract(other) {
    //         (Some(_), Some(_)) => TimeRangeComparison::Superset,
    //         (Some(_), None) => TimeRangeComparison::Earlier,
    //         (None, Some(_)) => TimeRangeComparison::Later,
    //         (None, None) => TimeRangeComparison::Subset,
    //     }
    // }

    pub fn contains<O>(&self, rhs: O) -> bool
    where
        O: TimeResolution,
        P: LongerThanOrEqual<O>,
    {
        self.start().start_minute() <= rhs.start_minute()
            && self.end().end_minute() >= rhs.end_minute()
    }
    pub fn set(&self) -> collections::BTreeSet<P> {
        self.iter().collect()
    }
    pub fn iter(&self) -> TimeRangeIter<P> {
        TimeRangeIter { iter: self.range.range(), ty: PhantomData }
    }

    pub fn rescale<Out>(&self) -> TimeRange<Out>
    where
        Out: TimeResolution + From<Minute> + FromMonotonic,
    {
        // get the exact start
        let start = Out::from(self.start().start_minute());

        // for the end, we can't use something like 23:59:59
        // so we instead get the next period then look back.
        let end = Out::from(self.end().end_minute());

        TimeRange::from_bounds(start, end)
    }
}

pub struct TimeRangeIter<P: TimeResolution> {
    iter: RangeInclusive<i64>,
    ty: PhantomData<P>,
}

impl<P: TimeResolution + FromMonotonic> Iterator for TimeRangeIter<P> {
    type Item = P;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().and_then(P::from_monotonic)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<P: TimeResolution + FromMonotonic> FusedIterator for TimeRangeIter<P> {}

impl<P: TimeResolution + FromMonotonic> ExactSizeIterator for TimeRangeIter<P> {}

impl<P: TimeResolution + FromMonotonic> DoubleEndedIterator for TimeRangeIter<P> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().and_then(P::from_monotonic)
    }
}

// #[cfg(feature = "chrono")]
// impl<P: TimeResolution + FromMonotonic, Z: FixedTimeZone> TimeRange<Zoned<P, Z>>
// where
//     Zoned<P, Z>: FromMonotonic,
// {
//     pub fn local(&self) -> TimeRange<P> {
//         TimeRange::new(self.start().local_resolution(), self.len)
//     }
// }

pub struct Cache<K: Ord + fmt::Debug + Copy, T: Send + fmt::Debug + Eq + Copy> {
    // The actual data in the cache
    data: collections::BTreeMap<K, T>,
    // The requests for data which has been cached
    requests: collections::BTreeSet<K>,
}

// merge a request into a set of requests, grouping contigious on the way
fn missing_pieces<K: Ord + fmt::Debug + Copy>(
    request: collections::BTreeSet<K>,
    requests: &collections::BTreeSet<K>,
) -> Vec<collections::BTreeSet<K>> {
    let mut to_request = Vec::new();
    let mut current_request = collections::BTreeSet::new();

    // there is a fundamental assumption that `request` is contigious
    // as long as `request` is contigious, each of the returned requests
    // will also be contigious
    // there is no need to worry about filling gaps to reduce the total number
    // of requests - the consumer will handle this
    for requested in request {
        if !requests.contains(&requested) {
            current_request.insert(requested);
        } else if !current_request.is_empty() {
            to_request.push(mem::take(&mut current_request));
        }
    }

    if !current_request.is_empty() {
        to_request.push(current_request);
    }

    to_request
}

// No concept of partial, becuse we will simply request the missing data, then ask the cache again.
pub enum CacheResponse<K: Ord + fmt::Debug + Copy, T: Send + fmt::Debug + Eq + Copy> {
    Hit(collections::BTreeMap<K, T>), // means the whole request as able to be replied, doesn't necessarily mean the whole range of data is filled
    Miss(Vec<collections::BTreeSet<K>>), // will be a minimal reasonable set of time ranges to request from the provider
}

impl<K: Ord + fmt::Debug + Copy, T: Send + fmt::Debug + Eq + Copy> Cache<K, T> {
    pub fn get(&self, request: collections::BTreeSet<K>) -> CacheResponse<K, T> {
        if request.is_empty() {
            CacheResponse::Hit(collections::BTreeMap::new())
        } else if self.requests.is_superset(&request) {
            CacheResponse::Hit(
                self.data
                    .iter()
                    // mustn't be empty othewise we would have returned out of the first arm of the `if`
                    .filter(|(k, _)| request.iter().next().unwrap() <= *k)
                    .filter(|(k, _)| request.iter().next_back().unwrap() >= *k)
                    .map(|(k, v)| (*k, *v))
                    .collect(),
            )
        } else {
            CacheResponse::Miss(missing_pieces(request, &self.requests))
        }
    }
    pub fn empty() -> Cache<K, T> {
        Cache {
            data: collections::BTreeMap::new(),
            requests: collections::BTreeSet::new(),
        }
    }
    // could also store versioned data, with a DateTIme<Utc> associated with each T at each P?
    // or allow overwriting, etc
    // but this default seems better for now
    pub fn add(
        &mut self,
        mut request_range: collections::BTreeSet<K>,
        data: collections::BTreeMap<K, T>,
    ) {
        self.requests.append(&mut request_range);
        for (point, datum) in data {
            // should we check if the data point already exists?
            // if it does exist, what should we do?
            // for now, ignoring, as otherwise
            // this function would need to be fallible
            self.data.insert(point, datum);
        }
    }
}
#[cfg(test)]
mod tests {

    use date::MonthOfYear;

    use crate::{Day, FiveMinute, Hour, Month, Year};

    use super::*;

    #[test]
    fn test_iter() {
        let mth = Month::new(Year::from_monotonic(2024).unwrap(), MonthOfYear::Jan);

        let day_range = mth.rescale::<Day>();

        extern crate std;
        use std::dbg;
        dbg!(
            mth.start_day().date().to_ymd(),
            mth.end_day().date().to_ymd(),
            day_range.start().date().to_ymd(),
            day_range.end().date().to_ymd()
        );

        let mut iter = day_range.iter();

        assert_eq!(iter.len(), 31);
        assert_eq!(iter.next(), Some(mth.start_day().into()));
        assert_eq!(iter.next_back(), Some(mth.end_day().into()));
        assert_eq!(iter.len(), 29);
        let mut iter = iter.skip(29);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_missing_pieces() {
        let pieces = missing_pieces(
            collections::BTreeSet::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]),
            &collections::BTreeSet::from([2, 3, 7, 8]),
        );
        assert_eq!(
            pieces,
            Vec::from([
                collections::BTreeSet::from([1]),
                collections::BTreeSet::from([4, 5, 6]),
                collections::BTreeSet::from([9, 10]),
            ])
        )
    }

    #[cfg(feature = "chrono")]
    #[test]
    fn test_contains() {
        extern crate std;
        use crate::Minutes;
        use alloc::string::ToString;
        use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
        use std::dbg;

        let mth = Month::new(Year::from_monotonic(2024).unwrap(), MonthOfYear::Jan);

        let day_range = mth.rescale::<Day>();

        dbg!(
            mth.to_string(),
            day_range.start().start_day(),
            day_range.end().start_day()
        );

        // assert!(
        //     day_range.contains(Minutes::<5>::from_utc_datetime(
        //         NaiveDateTime::new(
        //             NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
        //             NaiveTime::from_hms_opt(15, 15, 0).unwrap(),
        //         )
        //         .and_utc()
        //     ))
        // );

        let year = Year::from_monotonic(2024).unwrap();

        let month_range = year.rescale::<Month>();

        assert!(month_range.contains(mth))
    }

    // #[test]
    // fn test_rescale() {
    //     let start = Year::from_monotonic(2024).unwrap();
    //     let year = TimeRange::from_bounds(start, start);

    //     let fiveminute = year.rescale::<FiveMinute>();
    //     assert_eq!(fiveminute.len().get(), 366 * 288);
    //     assert_eq!(fiveminute.rescale::<Year>(), year);

    //     let hours = year.rescale::<Hour>();
    //     assert_eq!(hours.len().get(), 366 * 24);
    //     assert_eq!(hours.rescale::<Year>(), year);
    //     assert_eq!(fiveminute.rescale::<Hour>(), hours);

    //     let days = year.rescale::<Day>();
    //     assert_eq!(days.len().get(), 366);
    //     assert_eq!(days.rescale::<Year>(), year);
    //     assert_eq!(fiveminute.rescale::<Day>(), days);
    //     assert_eq!(hours.rescale::<Day>(), days);

    //     let months = year.rescale::<Month>();
    //     assert_eq!(months.len().get(), 12);
    //     assert_eq!(months.rescale::<Year>(), year);
    //     assert_eq!(fiveminute.rescale::<Month>(), months);
    //     assert_eq!(hours.rescale::<Month>(), months);
    //     assert_eq!(days.rescale::<Month>(), months);
    // }
}
