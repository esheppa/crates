use alloc::{boxed::Box, vec::Vec};
use core::u32;
use rust_decimal::Decimal;

// simple compression that exploits the fact that many (most?) series of decimals
// will have a (normlalized) manitissa who's range fits within a much smaller integer type
// given an apprporiate offset.

// TODO:
// 2. allow this to be combined with an outliers method
// 3. generally make sure the API allows multiple compression methods to be chained
#[derive(Clone)]
pub struct Compressed {
    offset: i128,
    scale: u32,
    values: ComprssedType,
}
#[derive(Clone)]

enum StorageType {
    I16,
    I32,
    I64,
}

#[derive(Clone)]
enum ComprssedType {
    I16(Vec<i16>),
    I32(Vec<i32>),
    I64(Vec<i64>),
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
    fn get(&self, idx: usize) -> Option<i128> {
        match self {
            ComprssedType::I16(vec) => vec.get(idx).copied().map(i128::from),
            ComprssedType::I32(vec) => vec.get(idx).copied().map(i128::from),
            ComprssedType::I64(vec) => vec.get(idx).copied().map(i128::from),
        }
    }
    fn storage_type(&self) -> StorageType {
        match self {
            ComprssedType::I16(_) => StorageType::I16,
            ComprssedType::I32(_) => StorageType::I32,
            ComprssedType::I64(_) => StorageType::I64,
        }
    }
}

const I16_RANGE: i128 = (i16::MAX as i128) - (i16::MIN as i128);
const I32_RANGE: i128 = (i32::MAX as i128) - (i32::MIN as i128);
const I64_RANGE: i128 = (i64::MAX as i128) - (i64::MIN as i128);

impl Compressed {
    // if data can't be compressed, will return None...
    pub fn new(data: &[Decimal]) -> Option<Self> {
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

    pub fn iter(&self) -> impl Iterator<Item = Decimal> + '_ {
        self.values
            .iter()
            .map(|v| Decimal::from_i128_with_scale(v + self.offset, self.scale))
    }
    pub fn get(&self, idx: usize) -> Option<Decimal> {
        self.values
            .get(idx)
            .map(|v| Decimal::from_i128_with_scale(v + self.offset, self.scale))
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal::Decimal;

    use crate::compressed::StorageType;

    use super::Compressed;

    #[test]
    fn test_compression() {
        let data = &[
            Decimal::new(2, 2),
            Decimal::new(456, 2),
            Decimal::new(7892, 3),
        ];

        let compressed = Compressed::new(data).unwrap();

        assert!(matches!(compressed.values.storage_type(), StorageType::I16));

        for (a, b) in data.iter().zip(compressed.iter()) {
            assert_eq!(*a, b);
        }

        let data = &[
            Decimal::new(1, 0),
            Decimal::new(680_000, 0),
            Decimal::new(690_000, 0),
        ];

        let compressed = Compressed::new(data).unwrap();

        assert!(matches!(compressed.values.storage_type(), StorageType::I32));

        for (a, b) in data.iter().zip(compressed.iter()) {
            assert_eq!(*a, b);
        }

        let data = &[
            Decimal::new(2, 4),
            Decimal::new(123_112_123_212, 4),
            Decimal::new(123_112_123_212, 5),
        ];

        let compressed = Compressed::new(data).unwrap();

        assert!(matches!(compressed.values.storage_type(), StorageType::I64));

        for (a, b) in data.iter().zip(compressed.iter()) {
            assert_eq!(*a, b);
        }
    }
}
