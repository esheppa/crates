#![no_std]
use arrayvec::ArrayString;
/// https://www.aemo.com.au/-/media/files/electricity/nem/retail_and_metering/metering-procedures/nmi-allocation-list.pdf?rev=e4c92faff5614b20933b16a4ff5784be&sc_lang=en
/// https://www.aemo.com.au/-/media/files/electricity/nem/retail_and_metering/metering-procedures/2024/msats-national-metering-identifier-procedure-v73.pdf?rev=aefc0a9f2fcb406aa81df9ba77e9512a&sc_lang=en
use core::{error::Error, fmt::Display, ops::Mul, str::FromStr};

extern crate std;
use std::eprintln;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Nmi([NmiChar; 10]);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NmiChar {
    Alpha(NmiAlpha),
    Numeric(NmiNumeric),
}

impl NmiChar {
    pub const fn byte(self) -> u8 {
        match self {
            NmiChar::Alpha(nmi_alpha) => nmi_alpha.0,
            NmiChar::Numeric(nmi_numeric) => nmi_numeric.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
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
    pub const fn as_u128(self) -> u128 {
        let mut le_bytes = [0; 16];

        let mut i = 0;
        while i < 10 {
            le_bytes[i + 6] = self.0[i].byte();
            i += 1;
        }
        u128::from_le_bytes(le_bytes)
    }
    pub const fn from_bytes(bytes: &[u8]) -> Result<Self, NmiError> {
        let Ok(input_str) = str::from_utf8(bytes) else {
            return Err(NmiError {
                input: ArrayString::new_const(),
                kind: NmiErrorKind::NonAsciiCharacters,
            });
        };

        let input = input_from_str(input_str);

        if bytes.len() > 11 {
            return Err(NmiError {
                input,
                kind: NmiErrorKind::TooLong,
            });
        } else if bytes.len() < 10 {
            return Err(NmiError {
                input,
                kind: NmiErrorKind::TooShort,
            });
        }

        for x in bytes.iter() {
            if !x.is_ascii_alphanumeric() {
                return Err(NmiError {
                    input,
                    kind: NmiErrorKind::NonAsciiCharacters,
                });
            }
        }

        let mut nmi = [NmiChar::Numeric(NmiNumeric(0)); 10];

        let mut i = 0;
        while i < 10 {
            let b = bytes[i];
            match b {
                b'0'..=b'9' => {
                    nmi[i] = NmiChar::Numeric(NmiNumeric::new(b));
                }
                b'I' | b'O' => {
                    eprintln!("Danm I/O");
                    return Err(NmiError {
                        input,
                        kind: NmiErrorKind::DisallowedCharacters,
                    });
                }
                b'A'..=b'Z' => {
                    nmi[i] = NmiChar::Alpha(NmiAlpha::new(b));
                }
                _ => {
                    eprintln!("Danm {b}");
                    return Err(NmiError {
                        input,
                        kind: NmiErrorKind::DisallowedCharacters,
                    });
                }
            }
            i += 1;
        }

        let nmi = Nmi(nmi);

        if bytes.len() == 11 && nmi.checksum() != bytes[10] {
            return Err(NmiError {
                input,
                kind: NmiErrorKind::InvalidChecksum {
                    expected: nmi.checksum(),
                },
            });
        }

        Ok(nmi)
    }
    pub const fn from_u128(u: u128) -> Result<Self, NmiError> {
        let le_bytes = u.to_le_bytes();

        const fn last_n_bytes_of<const N: usize>(input: &[u8]) -> [u8; N] {
            if input.len() < N {
                panic!("Input too short")
            }
            let offset = input.len() - N;
            let mut sl = [0; N];
            let mut i = 0;
            while i < N {
                sl[i] = input[i + offset];
                i += 1;
            }
            sl
        }
        Self::from_bytes(&last_n_bytes_of::<10>(&le_bytes))
    }
    pub const fn is_numeric(self) -> bool {
        let mut i = 0;
        while i < 10 {
            if let NmiChar::Alpha(_) = self.0[i] {
                return false;
            }
            i += 1;
        }
        true
    }
    pub const fn checksum(self) -> u8 {
        // https://stripe.com/au/resources/more/how-to-use-the-luhn-algorithm-a-guide-in-applications-for-businesses
        // https://en.wikipedia.org/wiki/Luhn_algorithm
        let mut checksum_counter = 0_u32;
        let mut i = 0;
        while i < 10 {
            // doubled ascii values up to Z are all less than 200
            // so we only need 0th, 1st and 2nd digits.

            let ascii_val = self.0[i].byte() as u32;

            // instead of going backwards... we just double odd indexes
            if i % 2 != 0 {
                checksum_counter += sum_digits(2 * ascii_val);
            } else {
                checksum_counter += sum_digits(ascii_val);
            }
            i += 1;
        }

        let next_highest_multiple_10 = 10 * (checksum_counter / 10 + 1);
        let diff = next_highest_multiple_10 - checksum_counter;

        match nth_digit::<0>(diff) {
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
            _ => panic!("danm!"),
        }
    }
    pub const fn prefix_bytes<const N: usize>(self) -> [u8; N] {
        if N > 10 {
            panic!("Can't take a prefix longer than 10 from a NMI");
        }

        let mut buf = [0; N];
        let mut i = 0;
        while i < N {
            buf[i] = self.0[i].byte();
            i += 1;
        }
        buf
    }

    pub const fn gas_region(self) -> Option<GasRegion> {
        let Some(c) = self.classification() else {
            return None;
        };
        c.gas()
    }

    pub const fn electricity_region(self) -> Option<ElectricityRegion> {
        let Some(c) = self.classification() else {
            return None;
        };
        c.electricity()
    }

    /// This is a _little_ permissive - however it is deemed to be unlikely that similar / overlapping ranges will be allocated elsewhere
    pub const fn classification(self) -> Option<Classification> {
        let bytes = self.prefix_bytes::<10>();
        let prefix_4 = self.prefix_bytes::<4>();
        // match (prefix_4[0], prefix_4[1]) {
        //     (b'A', b'A'..b'Z') => (),
        //     _ => (),
        // };

        let c = match &prefix_4 {
            _ if bytes[0] == b'A' && bytes[4] == b'W' => {
                Classification::Electricity(ElectricityRegion::Act(ActNsp::EvoenergyTnsp))
            }
            _ if bytes[0] == b'Q' && bytes[4] == b'W' => Classification::Electricity(
                ElectricityRegion::Qld(QldNsp::QldElectricityTransmissionCorp),
            ),
            _ if bytes[0] == b'V' && bytes[4] == b'W' => {
                Classification::Electricity(ElectricityRegion::Vic(VicNsp::AusnetTnsp))
            }
            _ if bytes[0] == b'S' && bytes[4] == b'W' => {
                Classification::Electricity(ElectricityRegion::Sa(SaNsp::ElectraNet))
            }
            _ if bytes[0] == b'T' && bytes[4] == b'W' => {
                Classification::Electricity(ElectricityRegion::Tas(TasNsp::TasNetworksTnsp))
            }
            b"NTTT" | b"4608" => {
                Classification::Electricity(ElectricityRegion::Nsw(NswNsp::TransGrid))
            }
            b"6509" => Classification::Electricity(ElectricityRegion::Vic(VicNsp::AusnetTnsp)),
            b"2102" => Classification::Electricity(ElectricityRegion::Sa(SaNsp::ElectraNet)),
            b"3202" => Classification::Electricity(ElectricityRegion::Qld(
                QldNsp::QldElectricityTransmissionCorp,
            )),
            _ if bytes[4] != b'W' => return None,

            b"NGGG" | b"7001" => {
                Classification::Electricity(ElectricityRegion::Act(ActNsp::Evoenergy))
            }
            b"NAAA" | b"NBBB" | b"NDDD" | b"NFFF" | b"4001" | b"4508" | b"4204" | b"4407" => {
                Classification::Electricity(ElectricityRegion::Nsw(NswNsp::Essential))
            }
            b"NEEE" | b"4310" | b"4311" | b"4312" | b"4313" | b"4314" | b"4315" | b"4316"
            | b"4317" | b"4318" | b"4319" => {
                Classification::Electricity(ElectricityRegion::Nsw(NswNsp::Endeavour))
            }
            b"NCCC" | b"4102" | b"4103" | b"4104" => {
                Classification::Electricity(ElectricityRegion::Nsw(NswNsp::Ausgrid))
            }
            b"2500" | b"2501" | b"2502" => {
                Classification::Electricity(ElectricityRegion::Nt(NtNsp::PowerAndWaterCorp))
            }
            b"2503" | b"2504" | b"2505" | b"2506" | b"2507" | b"2508" | b"2509" => {
                Classification::Electricity(ElectricityRegion::Nt(NtNsp::PowerAndWaterCorp))
            }

            _ if bytes[0] == b'Q' && bytes[1] == b'B' => {
                Classification::Electricity(ElectricityRegion::Qld(QldNsp::Energex))
            }
            _ if bytes[0] == b'3' && bytes[1] == b'1' => {
                Classification::Electricity(ElectricityRegion::Qld(QldNsp::Energex))
            }
            b"QAAA" | b"QCCC" | b"QDDD" | b"QEEE" | b"QFFF" | b"QGGG" => {
                Classification::Electricity(ElectricityRegion::Qld(QldNsp::Ergon))
            }

            b"SAAA" | b"2001" | b"2002" | b"SASM" => {
                Classification::Electricity(ElectricityRegion::Sa(SaNsp::Sapn))
            }

            b"T000" | b"8000" | b"8590" => {
                Classification::Electricity(ElectricityRegion::Tas(TasNsp::TasNetworks))
            }

            b"VAAA" | b"6102" | b"6103" => {
                Classification::Electricity(ElectricityRegion::Vic(VicNsp::CitiPower))
            }
            b"VBBB" | b"6305" | b"6306" => {
                Classification::Electricity(ElectricityRegion::Vic(VicNsp::Ausnet))
            }
            b"VCCC" | b"6203" | b"6204" => {
                Classification::Electricity(ElectricityRegion::Vic(VicNsp::Powercor))
            }
            b"VDDD" | b"6001" => {
                Classification::Electricity(ElectricityRegion::Vic(VicNsp::Jemena))
            }
            b"VEEE" | b"6407" | b"6408" => {
                Classification::Electricity(ElectricityRegion::Vic(VicNsp::United))
            }

            b"8021" => Classification::Electricity(ElectricityRegion::Wa(WaNsp::HorizonPower)),
            b"WAAA" => Classification::Electricity(ElectricityRegion::Wa(WaNsp::WesternPower)),
            _ if bytes[0] == b'8'
                && bytes[1] == b'0'
                && (bytes[2] == 0 || bytes[2] == 1 || bytes[2] == 2)
                && self.is_numeric() =>
            {
                Classification::Electricity(ElectricityRegion::Wa(WaNsp::WesternPower))
            }

            b"NJJJ" => Classification::Misc(Misc::SydneyAirport),
            b"NKKK" | b"7102" => Classification::Misc(Misc::Exempt),
            b"7105" | b"7106" => Classification::Misc(Misc::Embedded),
            b"8801" | b"8802" | b"8803" | b"8804" | b"8805" => {
                Classification::Misc(Misc::ReservedBlock1)
            }
            _ if bytes[0] == b'9' && self.is_numeric() => {
                Classification::Misc(Misc::ReservedBlock2)
            }

            _ if bytes[0] == b'5' && bytes[1] == b'2' && self.is_numeric() => {
                Classification::Gas(GasRegion::Nsw)
            }
            _ if bytes[0] == b'5' && bytes[1] == b'3' && self.is_numeric() => {
                Classification::Gas(GasRegion::Vic)
            }
            _ if bytes[0] == b'5' && bytes[1] == b'4' && self.is_numeric() => {
                Classification::Gas(GasRegion::Qld)
            }
            _ if bytes[0] == b'5' && bytes[1] == b'5' && self.is_numeric() => {
                Classification::Gas(GasRegion::Sa)
            }
            _ if bytes[0] == b'5' && bytes[1] == b'6' && self.is_numeric() => {
                Classification::Gas(GasRegion::Wa)
            }
            _ if bytes[0] == b'5' && bytes[1] == b'7' && self.is_numeric() => {
                Classification::Gas(GasRegion::Tas)
            }
            _ => return None,
        };
        Some(c)
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
    DisallowedCharacters,
    InvalidChecksum { expected: u8 },
}

impl Display for NmiErrorKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            NmiErrorKind::NonAsciiCharacters => f.write_str("Non ascii characters"),
            NmiErrorKind::TooLong => f.write_str("Too long"),
            NmiErrorKind::TooShort => f.write_str("Too short"),
            NmiErrorKind::DisallowedCharacters => f.write_str("Disallowed characters"),
            NmiErrorKind::InvalidChecksum { expected } => {
                write!(f, "Invalid checksum, expected: {expected}")
            }
        }
    }
}

impl Display for NmiError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Unable to parse {} as NMI due to: {}",
            self.input, self.kind
        )
    }
}

impl Error for NmiError {}

const fn input_from_str(s: &str) -> ArrayString<20> {
    let mut input = ArrayString::new_const();
    for i in s.chars().take(20) {
        input.push(i);
    }
    return input;
}

impl FromStr for Nmi {
    type Err = NmiError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes())
    }
}

pub struct FixedAsciiStr {
    data: [u8; 20]
}

impl FixedAsciiStr {
    pub fn from_bytes(b: &[u8]) -> FixedAsciiStr {

    }
      pub fn from_bytes(b: &[u8]) -> FixedAsciiStr {
        
    }
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NswNsp {
    Essential,
    Ausgrid,
    Endeavour,
    TransGrid,
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActNsp {
    Evoenergy,
    EvoenergyTnsp,
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NtNsp {
    PowerAndWaterCorp,
    PowerAndWaterCorpTnsp,
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QldNsp {
    Ergon,
    Energex,
    QldElectricityTransmissionCorp,
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TasNsp {
    TasNetworks,
    TasNetworksTnsp,
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SaNsp {
    Sapn,
    ElectraNet,
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VicNsp {
    CitiPower,
    Ausnet,
    Powercor,
    Jemena,
    United,
    AusnetTnsp,
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WaNsp {
    WesternPower,
    HorizonPower,
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GasRegion {
    Nsw,
    Vic,
    Qld,
    Sa,
    Wa,
    Tas,
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Misc {
    SydneyAirport,
    Exempt,
    Embedded,
    ReservedBlock1,
    ReservedBlock2,
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ElectricityRegion {
    Act(ActNsp),
    Nsw(NswNsp),
    Nt(NtNsp),
    Qld(QldNsp),
    Sa(SaNsp),
    Tas(TasNsp),
    Vic(VicNsp),
    Wa(WaNsp),
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Classification {
    Electricity(ElectricityRegion),
    Gas(GasRegion),
    Misc(Misc),
}

impl Classification {
    pub const fn electricity(self) -> Option<ElectricityRegion> {
        match self {
            Classification::Electricity(x) => Some(x),
            _ => None,
        }
    }
    pub const fn gas(self) -> Option<GasRegion> {
        match self {
            Classification::Gas(x) => Some(x),
            _ => None,
        }
    }
    pub const fn misc(self) -> Option<Misc> {
        match self {
            Classification::Misc(x) => Some(x),
            _ => None,
        }
    }
}

const fn nth_digit<const N: u32>(n: u32) -> u32 {
    n % 10u32.pow(N + 1) / 10u32.pow(N)
}

const fn sum_digits(n: u32) -> u32 {
    if n > 999 {
        panic!("cannot sum digits for a number greater than 1000");
    }

    nth_digit::<0>(n) + nth_digit::<1>(n) + nth_digit::<2>(n)
}

impl Nmi {}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;

    #[test]
    fn test_nth_digit() {
        assert_eq!(nth_digit::<3>(5269), 5);
        assert_eq!(nth_digit::<2>(5269), 2);
        assert_eq!(nth_digit::<1>(5269), 6);
        assert_eq!(nth_digit::<0>(5269), 9);
    }
    #[test]
    fn test_checksum() {
        // assert_eq!(
        //     Nmi::from_bytes("1234C6789A".as_bytes()).unwrap().checksum(),
        //     b'3'
        // );
        assert_eq!(
            Nmi::from_bytes("2001985732".as_bytes()).unwrap().checksum(),
            b'8'
        );
        assert_eq!(
            Nmi::from_bytes("QAAAVZZZZZ".as_bytes()).unwrap().checksum(),
            b'3'
        );
        assert_eq!(
            Nmi::from_bytes("2001985733".as_bytes()).unwrap().checksum(),
            b'6'
        );
        assert_eq!(
            Nmi::from_bytes("QCDWW00010".as_bytes()).unwrap().checksum(),
            b'2'
        );
        assert_eq!(
            Nmi::from_bytes("3075621875".as_bytes()).unwrap().checksum(),
            b'8'
        );
        assert_eq!(
            Nmi::from_bytes("SMVEW00085".as_bytes()).unwrap().checksum(),
            b'8'
        );
        assert_eq!(
            Nmi::from_bytes("3075621876".as_bytes()).unwrap().checksum(),
            b'6'
        );
        assert_eq!(
            Nmi::from_bytes("VAAA000065".as_bytes()).unwrap().checksum(),
            b'7'
        );
        assert_eq!(
            Nmi::from_bytes("4316854005".as_bytes()).unwrap().checksum(),
            b'9'
        );
        assert_eq!(
            Nmi::from_bytes("VAAA000066".as_bytes()).unwrap().checksum(),
            b'5'
        );
        assert_eq!(
            Nmi::from_bytes("4316854006".as_bytes()).unwrap().checksum(),
            b'7'
        );
        assert_eq!(
            Nmi::from_bytes("VAAA000067".as_bytes()).unwrap().checksum(),
            b'2'
        );
        assert_eq!(
            Nmi::from_bytes("6305888444".as_bytes()).unwrap().checksum(),
            b'6'
        );
        assert_eq!(
            Nmi::from_bytes("VAAASTY576".as_bytes()).unwrap().checksum(),
            b'8'
        );
        assert_eq!(
            Nmi::from_bytes("6350888444".as_bytes()).unwrap().checksum(),
            b'2'
        );
        assert_eq!(
            Nmi::from_bytes("VCCCX00009".as_bytes()).unwrap().checksum(),
            b'1'
        );
        assert_eq!(
            Nmi::from_bytes("7001888333".as_bytes()).unwrap().checksum(),
            b'8'
        );
        assert_eq!(
            Nmi::from_bytes("VEEEX00009".as_bytes()).unwrap().checksum(),
            b'1'
        );
        assert_eq!(
            Nmi::from_bytes("7102000001".as_bytes()).unwrap().checksum(),
            b'7'
        );
        assert_eq!(
            Nmi::from_bytes("VKTS786150".as_bytes()).unwrap().checksum(),
            b'2'
        );
        assert_eq!(
            Nmi::from_bytes("NAAAMYS582".as_bytes()).unwrap().checksum(),
            b'6'
        );
        assert_eq!(
            Nmi::from_bytes("VKTS867150".as_bytes()).unwrap().checksum(),
            b'5'
        );
        assert_eq!(
            Nmi::from_bytes("NBBBX11110".as_bytes()).unwrap().checksum(),
            b'0'
        );
        assert_eq!(
            Nmi::from_bytes("VKTS871650".as_bytes()).unwrap().checksum(),
            b'7'
        );
        assert_eq!(
            Nmi::from_bytes("NBBBX11111".as_bytes()).unwrap().checksum(),
            b'8'
        );
        assert_eq!(
            Nmi::from_bytes("VKTS876105".as_bytes()).unwrap().checksum(),
            b'7'
        );
        assert_eq!(
            Nmi::from_bytes("NCCC519495".as_bytes()).unwrap().checksum(),
            b'5'
        );
        assert_eq!(
            Nmi::from_bytes("VKTS876150".as_bytes()).unwrap().checksum(),
            b'3'
        );
        assert_eq!(
            Nmi::from_bytes("NGGG000055".as_bytes()).unwrap().checksum(),
            b'4'
        );
        assert_eq!(
            Nmi::from_bytes("VKTS876510".as_bytes()).unwrap().checksum(),
            b'8'
        );
    }
}
