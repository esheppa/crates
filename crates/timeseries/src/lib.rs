#![no_std]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

use alloc::{collections::BTreeMap, fmt, string::String, string::ToString, vec::Vec};
use compressed::Compressed;
use core::{iter::FusedIterator, num::NonZeroU64};
use resolution::{TimeRange, TimeRangeIter, TimeResolution};
use rust_decimal::Decimal;

mod compressed;

// nullable? consider later?

// consider either forcing T to be a decimal, or having a trait that allows conversion
// alternatively, store it as a decimal and pass converters at runtime
// could later invesigate compression, etc, here.
#[derive(Clone)]
enum TimeseriesData {
    Plain(Vec<Decimal>),
    // RunLenghEncoded(RunLengthEncoded),
    Compressed(Compressed),
    // TBC:
    // map + u8 indexes
    // compressed + outliers
    // Shape + compressed
    // deltas + compressed
    // composition of compression layers
}

// #[derive(Clone)]
// struct RunLengthEncoded {
//     data: Vec<(NonZeroUsize, Decimal)>,
// }

impl TimeseriesData {
    // fn _into_inner(self) -> Vec<Decimal> {
    //     let TimeseriesData::Plain(s) = self;
    //     s
    // }

    fn get(&self, idx: usize) -> Option<Decimal> {
        match self {
            TimeseriesData::Plain(vec) => vec.get(idx).copied(),
            TimeseriesData::Compressed(compressed) => compressed.get(idx),
        }
    }
}

// consider:
// - what about integer fields?
// - what about non-numeric fields?

// just the raw data
#[derive(Clone)]
pub struct Timeseries<R, T>
where
    R: TimeResolution,
    T: Copy,
{
    range: TimeRange<R>,
    // include mapperfn to get from Decimal to T
    data: TimeseriesData,

    // render the data units to a string on serialization?
    // should this come via a Trait that T must implement instead?
    // disadvantage is that it means the trait will definitely not be
    // object safe...
    conv_out: fn(Decimal) -> T,
    conv_in: fn(T) -> Decimal,
}

impl<R> Timeseries<R, Decimal>
where
    R: TimeResolution + fmt::Display,
{
    pub fn new_decimal(iter: impl Iterator<Item = (R, Decimal)>) -> Result<Self> {
        Timeseries::new(iter, |i| i, |i| i)
    }
    pub fn from_parts_decimal(range: TimeRange<R>, data: Vec<Decimal>) -> Result<Self> {
        Timeseries::from_parts(range, data, |i| i, |i| i)
    }
}

pub struct TimeseriesIterator<'data, R, T>
where
    R: TimeResolution,
    T: Copy,
{
    inner: &'data Timeseries<R, T>,
    key_iter: TimeRangeIter<R>,
}

impl<'data, R, T> Iterator for TimeseriesIterator<'data, R, T>
where
    R: TimeResolution + fmt::Display,
    T: Copy,
{
    type Item = (R, T);

    fn next(&mut self) -> Option<Self::Item> {
        let key = self.key_iter.next()?;
        let value = self.inner.get(key)?;
        Some((key, value))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.key_iter.size_hint()
    }
}

impl<'data, R, T> FusedIterator for TimeseriesIterator<'data, R, T>
where
    R: TimeResolution + fmt::Display,
    T: Copy,
{
}

impl<'data, R, T> ExactSizeIterator for TimeseriesIterator<'data, R, T>
where
    R: TimeResolution + fmt::Display,
    T: Copy,
{
}

impl<'data, R, T> DoubleEndedIterator for TimeseriesIterator<'data, R, T>
where
    R: TimeResolution + fmt::Display,
    T: Copy,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let key = self.key_iter.next_back()?;
        let value = self.inner.get(key)?;
        Some((key, value))
    }
}

impl<R, T> Timeseries<R, T>
where
    R: TimeResolution + fmt::Display,
    T: Copy,
{
    pub fn compress(&mut self) -> Result<()> {
        match &self.data {
            TimeseriesData::Plain(vec) => {
                // compress the data

                let compressed = Compressed::new(&vec).ok_or_else(|| Error::CompressionFailure)?;

                self.data = TimeseriesData::Compressed(compressed);

                Ok(())
            }
            // already compresed
            TimeseriesData::Compressed(_) => Ok(()),
        }
    }
    pub fn contains(&self, time: R) -> bool {
        self.range.contains(time)
    }
    pub fn start(&self) -> R {
        self.range.start()
    }
    pub fn end(&self) -> R {
        self.range.end()
    }
    pub fn len(&self) -> NonZeroU64 {
        self.range.len()
    }
    pub fn iter(&self) -> TimeseriesIterator<'_, R, T> {
        TimeseriesIterator {
            inner: self,
            key_iter: self.range.iter(),
        }
    }
    pub fn iter_range(&self, range: TimeRange<R>) -> Option<TimeseriesIterator<'_, R, T>> {
        Some(TimeseriesIterator {
            inner: self,
            key_iter: self.range.intersection(&range)?.iter(),
        })
    }
    pub fn to_map(&self) -> BTreeMap<R, T> {
        self.iter().collect()
    }
    pub fn get(&self, time: R) -> Option<T> {
        self.get_decimal(time).map(self.conv_out)
    }
    pub fn get_decimal(&self, time: R) -> Option<Decimal> {
        let idx = self.range.index_of(time)?;
        self.data.get(idx)
    }
    pub fn range(&self) -> TimeRange<R> {
        self.range
    }

    pub fn new(
        mut iter: impl Iterator<Item = (R, T)>,
        conv_out: fn(Decimal) -> T,
        conv_in: fn(T) -> Decimal,
    ) -> Result<Timeseries<R, T>> {
        let (lower, upper) = iter.size_hint();
        let mut data = Vec::with_capacity(upper.unwrap_or(lower));
        let (start, data_start) = iter.next().ok_or(Error::Empty)?;
        let mut len = NonZeroU64::MIN;
        data.push(conv_in(data_start));
        for (time, obs) in iter {
            if time == start.succ_n(len.get()) {
                len = len.checked_add(1).ok_or(Error::LengthOverflow)?;
                data.push(conv_in(obs));
            } else {
                return Err(Error::NonContigious {
                    prev: start.succ_n(len.get()).to_string(),
                    next: time.to_string(),
                });
            }
        }
        Ok(Timeseries {
            range: TimeRange::new(start, len),
            data: TimeseriesData::Plain(data),
            conv_in,
            conv_out,
        })
    }

    pub fn from_parts(
        range: TimeRange<R>,
        input_data: Vec<T>,
        conv_out: fn(Decimal) -> T,
        conv_in: fn(T) -> Decimal,
    ) -> Result<Timeseries<R, T>> {
        if u64::from(range.len().get()) != u64::try_from(input_data.len()).unwrap() {
            return Err(Error::NonMatchingLength {
                range: range.len(),
                data: input_data.len(),
            });
        }

        let mut data = Vec::with_capacity(input_data.len());
        data.extend(input_data.into_iter().map(conv_in));

        Ok(Timeseries {
            range,
            data: TimeseriesData::Plain(data),
            conv_in,
            conv_out,
        })
    }

    pub fn from_map(
        map: &BTreeMap<R, T>,
        conv_out: fn(Decimal) -> T,
        conv_in: fn(T) -> Decimal,
    ) -> Result<Timeseries<R, T>> {
        Timeseries::new(map.iter().map(|(a, b)| (*a, *b)), conv_out, conv_in)
    }

    pub fn merge(
        self,
        rhs: Timeseries<R, T>,
    ) -> core::result::Result<Timeseries<R, T>, MergeFailure<R, T>> {
        let mut differences = BTreeMap::new();
        let (intersection, merged) = match (
            self.range.intersection(&rhs.range),
            self.range.union(&rhs.range),
        ) {
            (Some(intersection), Some(merged)) => (intersection, merged),

            _ => return Err(MergeFailure::NoIntersection { lhs: self, rhs }),
        };

        for t in intersection.iter() {
            match (self.get_decimal(t), rhs.get_decimal(t)) {
                (Some(lhs), Some(rhs)) => {
                    if lhs != rhs {
                        differences.insert(t, ((self.conv_out)(lhs), ((self.conv_out)(rhs))));
                    }
                }
                _ => return Err(MergeFailure::NoIntersection { lhs: self, rhs }),
            }
        }

        if !differences.is_empty() {
            return Err(MergeFailure::DifferentData {
                lhs: self,
                rhs,
                differences,
            });
        }

        let mut new_data = Vec::with_capacity(usize::try_from(self.range.len().get()).unwrap());

        for t in merged.iter() {
            match (self.get(t), rhs.get(t)) {
                (Some(l), _) => {
                    new_data.push((self.conv_in)(l));
                }
                (_, Some(r)) => {
                    new_data.push((self.conv_in)(r));
                }
                _ => {
                    return Err(MergeFailure::NoIntersection { lhs: self, rhs });
                }
            }
        }

        Ok(Timeseries {
            range: merged,
            data: TimeseriesData::Plain(new_data),
            conv_in: self.conv_in,
            conv_out: self.conv_out,
        })
    }
}

pub enum MergeFailure<R, T>
where
    R: TimeResolution,
    T: Copy,
{
    NoIntersection {
        lhs: Timeseries<R, T>,
        rhs: Timeseries<R, T>,
    },
    DifferentData {
        lhs: Timeseries<R, T>,
        rhs: Timeseries<R, T>,
        differences: BTreeMap<R, (T, T)>,
    },
}

type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    NonMatchingLength { range: NonZeroU64, data: usize },
    Empty,
    LengthOverflow,
    NonContigious { prev: String, next: String },
    CompressionFailure,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NonMatchingLength { range, data }=> write!(f, "Range and data should match but got range length of {range} and data length of {data}"),
            Error::Empty => write!(f, "Cannot create a Timeseries from an empty iterator"),
            Error::NonContigious { prev, next } => write!(f, "Cannot create a Timeseries from non-contigious data, but had a gap from {prev} to {next}"),
            Error::CompressionFailure => write!(f, "Unable to compress timeseries"),
            Error::LengthOverflow => write!(f, "Timeseries was longer than {}", NonZeroU64::MAX),
        }
    }
}

impl core::error::Error for Error {}

// struct TimeseriesIter<R, T>
// where R: TimeResolution, T: Num
// {

// }

#[cfg(test)]
mod tests {
    use resolution::Year;
    use rust_decimal::Decimal;

    use super::*;

    #[test]
    fn test_timeseries_new() {
        let data = [
            (Year::new(2022), Decimal::new(2, 2)),
            (Year::new(2023), Decimal::new(456, 2)),
            (Year::new(2024), Decimal::new(7892, 3)),
        ];

        let series = Timeseries::new(data.into_iter(), |i| i, |i| i).unwrap();

        assert_eq!(series.iter().len(), 3);
    }

    #[test]
    fn test_timeseries_from_parts() {
        let data = Vec::from([
            Decimal::new(2, 2),
            Decimal::new(456, 2),
            Decimal::new(7892, 3),
        ]);

        let series = Timeseries::from_parts_decimal(
            TimeRange::from_bounds(Year::new(2022), Year::new(2024)),
            data,
        )
        .unwrap();

        assert_eq!(series.iter().len(), 3);
    }
}
