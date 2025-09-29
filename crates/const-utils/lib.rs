#![no_std]

pub const fn nth_digit(d: u32, n: u32) -> u32 {
    n % 10u32.pow(d + 1) / 10u32.pow(d)
}

pub const fn sum_digits(d: u32, n: u32) -> Option<u32> {
    // can't sum excess digits
    if 10u32.pow(d) > n {
        return None;
    }

    let mut nth = Some(0);

    let mut sum = 0;

    while let Some(x) = nth {
        sum += nth_digit(x, n);

        if x == d {
            break;
        }

        nth = x.checked_add(1);
    }

    Some(sum)
}

pub const fn slice_to_array_exact<const N: usize>(sl: &[u8]) -> Option<[u8; N]> {
    if sl.len() != N {
        return None;
    }
    let mut data = [0; N];

    data.copy_from_slice(sl);

    Some(data)
}

const fn ascii_char_to_digit(c: u8) -> Option<u8> {
    Some(match c {
        b'0' => 0,
        b'1' => 1,
        b'2' => 2,
        b'3' => 3,
        b'4' => 4,
        b'5' => 5,
        b'6' => 6,
        b'7' => 7,
        b'8' => 8,
        b'9' => 9,
        _ => return None,
    })
}

const fn digit_to_ascii_char(d: u8) -> Option<u8> {
    Some(match d {
        0 => b'0',
        1 => b'1',
        2 => b'2',
        3 => b'3',
        4 => b'4',
        5 => b'5',
        6 => b'6',
        7 => b'7',
        8 => b'8',
        9 => b'9',
        _ => return None,
    })
}


    // let mut idx = 0;
    // while idx < bytes.len() {
    // }



// pub const fn parse_u32(input: &str) -> Result<u32, ()> {
//     let mut idx = 0u32;
//     let mut accum = 0;

//     while (idx as usize) < input.len() {
//         match ascii_char_to_digit(input.as_bytes()[input.len() - idx as usize]) {
//             Some(d) => {

//                 accum += (idx + 1) * (d as u32);
//                 idx += 1
//             }
//             None => {
//                 return Err(())
//             }
//         }
//     }
    
//   Ok(accum)
// }

// no push char allowed here...
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FixedString<const N: usize>([u8; N], usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FixedAsciiString<const N: usize>([u8; N], usize);

pub struct AsciiDigit(u8);
pub struct AsciiLetterLower(u8);
pub struct AsciiLetterUpper(u8);
pub struct AsciiAlpha(u8);

// stores the data in an array of size N
// note that this _always_ has a capacity of N
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FixedArray<const N: usize>([u8; N], usize);

impl<const N: usize> FixedArray<N> {
    pub const fn default() -> Self {
        Self([0; N], 0)
    }
    pub const fn len(&self) -> usize {
        self.1
    }
    pub const fn bytes(&self) -> &[u8] {
        self.0.split_at(self.1).0
    }
    pub const fn bytes_mut(&mut self) -> &mut [u8] {
        self.0.split_at_mut(self.1).0
    }

    pub const fn from_slice(sl: &[u8]) -> Result<Self, ()> {
        if sl.len() > N {
            return Err(());
        }
        let mut data = [0; N];

        let (lhs, _) = data.split_at_mut(sl.len());

        lhs.copy_from_slice(sl);

        Ok(Self(data, sl.len()))
    }

    pub const fn is_empty(&self) -> bool {
        self.1 == 0
    }
    pub const fn try_push(&mut self, item: u8) -> Result<(), ()> {
        if self.len() < N {
            self.0[self.len()] = item;
            Ok(())
        } else {
            Err(())
        }
    }
    pub const fn pop(&mut self) -> Option<u8> {
        match self.1.checked_sub(1) {
            Some(x) => {
                self.1 = x;
            }
            None => {
                return None;
            }
        }

        Some(self.0[self.1])
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;

    #[test]
    fn test_sum_digits() {
        assert_eq!(sum_digits(0, 5269), Some(9));
        assert_eq!(sum_digits(1, 5269), Some(15));
        assert_eq!(sum_digits(2, 5269), Some(17));
        assert_eq!(sum_digits(3, 5269), Some(22));
    }

    #[test]
    fn test_nth_digit() {
        assert_eq!(nth_digit(3, 5269), 5);
        assert_eq!(nth_digit(2, 5269), 2);
        assert_eq!(nth_digit(1, 5269), 6);
        assert_eq!(nth_digit(0, 5269), 9);
    }
}
