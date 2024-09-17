use num_traits::Num;
use resolution::{TimeRange, TimeResolution};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{collections::BTreeMap, fmt, num::NonZeroU64};

// consider either forcing T to be a decimal, or having a trait that allows conversion
// alternatively, store it as a decimal and pass converters at runtime
// could later invesigate compression, etc, here.
#[derive(Serialize, Deserialize, Clone)]
#[serde(bound(deserialize = "T: DeserializeOwned + Serialize"))]
enum TimeseriesData<T> {
    Plain(Vec<T>),
}

impl<T> TimeseriesData<T>
where
    T: Copy,
{
    fn _into_inner(self) -> Vec<T> {
        let TimeseriesData::Plain(s) = self;
        s
    }

    fn get(&self, idx: usize) -> Option<T> {
        match self {
            TimeseriesData::Plain(vec) => vec.get(idx).copied(),
        }
    }
}

// just the raw data
#[derive(Serialize, Deserialize, Clone)]
#[serde(bound(deserialize = "T: DeserializeOwned + Serialize, R: DeserializeOwned + Serialize"))]
pub struct Timeseries<R, T>
where
    R: TimeResolution,
    T: Num + Copy + DeserializeOwned,
{
    range: TimeRange<R>,
    data: TimeseriesData<T>,
}

impl<R, T> Timeseries<R, T>
where
    R: TimeResolution + fmt::Display,
    T: Num + Copy + DeserializeOwned,
{
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
    // consider TrustedLen here
    pub fn iter(&self) -> impl Iterator<Item = (R, T)> + '_ {
        self.range
            .iter()
            .enumerate()
            .filter_map(|(idx, t)| Some((t, self.data.get(idx)?)))
    }
    pub fn to_map(&self) -> BTreeMap<R, T> {
        self.iter().collect()
    }
    pub fn get(&self, time: R) -> Option<T> {
        let idx = self.range.index_of(time)?;
        self.data.get(idx)
    }
    pub fn range(&self) -> TimeRange<R> {
        self.range
    }

    pub fn new(mut iter: impl Iterator<Item = (R, T)>) -> Result<Timeseries<R, T>> {
        let mut data = Vec::with_capacity(iter.size_hint().0);
        let (start, data_start) = iter.next().ok_or(Error::Empty)?;
        let mut len = 1;
        data.push(data_start);
        for (time, obs) in iter {
            if time == start.succ_n(len).succ() {
                len += 1;
                data.push(obs);
            } else {
                return Err(Error::NonContigious {
                    prev: start.succ_n(len).to_string(),
                    next: time.to_string(),
                });
            }
        }
        Ok(Timeseries {
            range: TimeRange::new(start, NonZeroU64::new(len).ok_or(Error::Empty)?),
            data: TimeseriesData::Plain(data),
        })
    }

    pub fn from_parts(range: TimeRange<R>, data: Vec<T>) -> Result<Timeseries<R, T>> {
        if u64::from(range.len().get()) != u64::try_from(data.len()).unwrap() {
            return Err(Error::NonMatchingLength {
                range: range.len(),
                data: data.len(),
            });
        }

        Ok(Timeseries {
            range,
            data: TimeseriesData::Plain(data),
        })
    }

    pub fn from_map(map: &BTreeMap<R, T>) -> Result<Timeseries<R, T>> {
        Timeseries::new(map.iter().map(|(a, b)| (*a, *b)))
    }

    pub fn merge(
        self,
        rhs: Timeseries<R, T>,
    ) -> std::result::Result<Timeseries<R, T>, MergeFailure<R, T>> {
        let mut differences = BTreeMap::new();
        let (intersection, merged) = match (
            self.range.intersection(&rhs.range),
            self.range.union(&rhs.range),
        ) {
            (Some(intersection), Some(merged)) => (intersection, merged),

            _ => return Err(MergeFailure::NoIntersection { lhs: self, rhs }),
        };

        for t in intersection.iter() {
            match (self.get(t), rhs.get(t)) {
                (Some(lhs), Some(rhs)) => {
                    if lhs != rhs {
                        differences.insert(t, (lhs, rhs));
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
                    new_data.push(l);
                }
                (_, Some(r)) => {
                    new_data.push(r);
                }
                _ => {
                    return Err(MergeFailure::NoIntersection { lhs: self, rhs });
                }
            }
        }

        Ok(Timeseries {
            range: merged,
            data: TimeseriesData::Plain(new_data),
        })
    }
}

pub enum MergeFailure<R, T>
where
    R: TimeResolution,
    T: Num + Copy + DeserializeOwned,
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

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    NonMatchingLength { range: NonZeroU64, data: usize },
    Empty,
    NonContigious { prev: String, next: String },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NonMatchingLength { range, data }=> write!(f, "Range and data should match but got range length of {range} and data length of {data}"),
            Error::Empty => write!(f, "Cannot create a Timeseries from an empty iterator"),
            Error::NonContigious { prev, next } => write!(f, "Cannot create a Timeseries from non-contigious data, but had a gap from {prev} to {next}"),
        }
    }
}

impl std::error::Error for Error {}

// struct TimeseriesIter<R, T>
// where R: TimeResolution, T: Num
// {

// }
