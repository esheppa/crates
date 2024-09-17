#![no_std]
use core::{error::Error, fmt::Display, str::FromStr};

use arrayvec::ArrayString;

#[derive(Debug, Clone, Copy)]
pub struct Nmi([NmiChar; 10]);

#[derive(Debug, Clone, Copy)]
pub enum NmiChar {
    Alpha(NmiAlpha),
    Numeric(NmiNumeric),
}

impl NmiChar {
    fn byte(&self) -> u8 {
        match self {
            NmiChar::Alpha(nmi_alpha) => nmi_alpha.0,
            NmiChar::Numeric(nmi_numeric) => nmi_numeric.0,
        }
    }
}


#[derive(Debug, Clone, Copy)]
pub struct NmiAlpha(u8);

impl NmiAlpha {
    const fn new(c: u8) -> NmiAlpha {
        match c {
            b'I' | b'O' => {
                panic!("Not allowed 'o' or 'i'")
            }
            b'A'..=b'Z' => NmiAlpha(c),
            b'0'..=b'9' => {
                panic!("Not allowed number character in NMI alpha section")
            }
            b'a'..=b'z' => {
                panic!("Not allowed lowercase character in NMI")
            }
            _ => {
                panic!("Not allowed non-alphabetic character in NMI ")
            }
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub struct NmiNumeric(u8);

impl NmiNumeric {
    const fn new(c: u8) -> NmiNumeric {
        match c {
            b'0'..=b'9' => NmiNumeric(c),
            b'A'..=b'Z' | b'a'..=b'z' => {
                panic!("Not allowed alphabetical character in NMI")
            }
            _ => {
                panic!("Not allowed non-alphabetic character in NMI ")
            }
        }
    }
}

impl Nmi {
    fn checksum(&self) -> char {
        let mut checksum_counter = 0_u32;
        for (idx, char) in self.0.iter().enumerate() {
            if idx % 2 == 0 {
                checksum_counter += 2 * u32::from(char.byte());
            } else {
                checksum_counter += u32::from(char.byte());
            }
        }
        todo!()
    }
}

#[derive(Debug)]
pub struct NmiError {
    input: ArrayString<20>,
    kind: NmiErrorKind,
}

#[derive(Debug)]
pub enum NmiErrorKind {
    NonAsciiCharacters,
    TooLong,
    TooShort,
}

impl Display for NmiError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        todo!()
    }
}

impl Error for NmiError {}

fn input_from_str(s: &str) -> ArrayString<20> {
    let mut input = ArrayString::new();
    for i in s.chars().take(20) {
        input.push(i);
    }
    return input;
}

impl FromStr for Nmi {
    type Err = NmiError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.is_ascii() {
            return Err(NmiError {
                input: input_from_str(&s),
                kind: NmiErrorKind::NonAsciiCharacters,
            });
        }

        match s.len() {
            // no checksum
            10 => {
                let mut nmi = [NmiChar::Numeric(NmiNumeric(0)); 10];

                Ok(Nmi(nmi))
            }
            // has checksum - verify it
            11 => {
                todo!()
            }
            _ if s.len() < 10 => {
                return Err(NmiError {
                    input: input_from_str(&s),
                    kind: NmiErrorKind::TooShort,
                })
            }
            _ => {
                return Err(NmiError {
                    input: input_from_str(&s),
                    kind: NmiErrorKind::TooLong,
                })
            }
        }
    }
}

enum Region {
    Act,
    Nsw,
    Nt,
    Qld,
    Sa,
    Tas,
    Vic,
    Wa,
}

enum Classification {
    Electricity(Region),
    Gas(Region),
    Misc,
}

impl Nmi {}

#[cfg(test)]
mod tests {
    extern crate std;
}
