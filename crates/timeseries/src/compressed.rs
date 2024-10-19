use num_traits::Num;
use resolution::{TimeRange, TimeResolution};
use rust_decimal::Decimal;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fmt,
    num::{NonZeroU64, NonZeroUsize},
    u16, u32, u64,
};

// simple compression that exploits the fact that many (most?) series of decimals
// will have a (normlalized) manitissa who's range fits within a much smaller integer type
// given an apprporiate offset.

// TODO:
// 1. have a unsigned representation for cases when the numbers are all postive or all negative
// 2. allow this to be combined with an outliers method
// 3. generally make sure the API allows multiple compression methods to be chained
#[derive(Clone)]
pub struct Compressed {
    offset: i128,
    scale: u32,
    values: ComprssedType,
}

#[derive(Clone)]
enum ComprssedType {
    I16(Vec<i16>),
    I32(Vec<i32>),
    I64(Vec<i64>),
    // slight optimization for when all values are 0.. or ..=0
    // U16 {
    //     values: Vec<u16>,
    //     all_positive: bool,
    // },
    // U32 {
    //     values: Vec<u32>,
    //     all_positive: bool,
    // },
    // U64 {
    //     values: Vec<u64>,
    //     all_positive: bool,
    // },
}

impl ComprssedType {
    fn iter(&self) -> Box<dyn Iterator<Item = i128> + '_> {
        match self {
            ComprssedType::I16(vec) => {
                Box::new(vec.iter().map(|v| (*v).into())) as Box<dyn Iterator<Item = i128> + '_>
            }
            ComprssedType::I32(vec) => Box::new(vec.iter().map(|v| (*v).into())),
            ComprssedType::I64(vec) => Box::new(vec.iter().map(|v| (*v).into())),
        }
    }
}

// const U16_MAX: i128 = u16::MAX as i128;
// const U32_MAX: i128 = u32::MAX as i128;
// const U64_MAX: i128 = u64::MAX as i128;

const I16_RANGE: i128 = (i16::MAX as i128) - (i16::MIN as i128);
const I32_RANGE: i128 = (i32::MAX as i128) - (i32::MIN as i128);
const I64_RANGE: i128 = (i64::MAX as i128) - (i64::MIN as i128);

impl Compressed {
    // if data can't be compressed, will return None...
    fn new(data: &[Decimal]) -> Option<Self> {
        // first normalize the scales -> find the worst case scale and use that
        let max_scale = data.iter().map(|d| d.scale()).max()?;

        // find min & max _mantissa_, having rescaled to the worst case
        let min_mantissa = data
            .iter()
            .map(|d| {
                let mut d = *d;
                d.rescale(max_scale);
                d.mantissa()
            })
            .min()?;

        let max_mantissa = data
            .iter()
            .map(|d| {
                let mut d = *d;
                d.rescale(max_scale);
                d.mantissa()
            })
            .max()?;

        // let u16_max = u16::MAX as i128;
        // let u32_max = u32::MAX as i128;
        // let u64_max = u64::MAX as i128;

        match max_mantissa.checked_sub(min_mantissa)? {
            range if range < I16_RANGE => {
                let offset = min_mantissa;
                Some(Compressed {
                    offset,
                    scale: max_scale,
                    values: ComprssedType::I16(
                        data.iter()
                            .map(|d| {
                                let mut d = *d;
                                d.rescale(max_scale);
                                let val = d.mantissa().checked_sub(offset)?;
                                i16::try_from(val).ok()
                            })
                            .collect::<Option<_>>()?,
                    ),
                })
            }
            range if range < I32_RANGE => {
                let offset = min_mantissa;
                Some(Compressed {
                    offset,
                    scale: max_scale,
                    values: ComprssedType::I32(
                        data.iter()
                            .map(|d| {
                                let mut d = *d;
                                d.rescale(max_scale);
                                let val = d.mantissa().checked_sub(offset)?;
                                i32::try_from(val).ok()
                            })
                            .collect::<Option<_>>()?,
                    ),
                })
            }
            range if range < I64_RANGE => {
                let offset = min_mantissa;
                Some(Compressed {
                    offset,
                    scale: max_scale,
                    values: ComprssedType::I64(
                        data.iter()
                            .map(|d| {
                                let mut d = *d;
                                d.rescale(max_scale);
                                let val = d.mantissa().checked_sub(offset)?;
                                i64::try_from(val).ok()
                            })
                            .collect::<Option<_>>()?,
                    ),
                })
            }
            _ => return None,
        }
    }

    fn iter(&self) -> impl Iterator<Item = Decimal>  + '_ {
        self.values.iter().map(|v| Decimal::from_i128_with_scale(v + self.offset, self.scale))
    }
}
