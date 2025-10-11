#![no_std]
use const_utils::FixedString;
/// https://www.aemo.com.au/-/media/files/electricity/nem/retail_and_metering/metering-procedures/nmi-allocation-list.pdf?rev=e4c92faff5614b20933b16a4ff5784be&sc_lang=en
/// https://www.aemo.com.au/-/media/files/electricity/nem/retail_and_metering/metering-procedures/2024/msats-national-metering-identifier-procedure-v73.pdf?rev=aefc0a9f2fcb406aa81df9ba77e9512a&sc_lang=en
use core::{error::Error, fmt::Display, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Nmi([u8; 10]);

impl Display for Nmi {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NmiChar(u8);

impl NmiChar {
    pub const fn try_new(c: u8) -> Result<NmiChar, NmiErrorKind> {
        match c {
            b'I' | b'O' => Err(NmiErrorKind::DisallowedCharacter(c)),
            b'A'..=b'Z' => Ok(NmiChar(c)),
            b'0'..=b'9' => Ok(NmiChar(c)),
            _ => Err(NmiErrorKind::DisallowedCharacter(c)),
        }
    }
    const fn new(c: u8) -> NmiChar {
        let Ok(x) = Self::try_new(c) else {
            panic!("attempted to create NmiChar from invalid character")
        };
        x
    }
    pub const fn byte(self) -> u8 {
        self.0
    }
    #[allow(
        non_contiguous_range_endpoints,
        reason = "We explicitly want to exclude 'O' and 'I'"
    )]
    const fn index(self) -> u64 {
        match self.0 {
            b'A'..=b'H' => (self.0 as u64) - b'A' as u64 + 10,
            b'J'..=b'N' => (self.0 as u64) - b'J' as u64 + 18,
            b'P'..=b'Z' => (self.0 as u64) - b'P' as u64 + 23,
            b'0'..=b'9' => (self.0 as u64) - b'0' as u64,
            __ => panic!("Input is pre validated"),
        }
    }
}

impl Nmi {
    pub const fn as_str(&self) -> &str {
        match str::from_utf8(self.as_bytes()) {
            Ok(s) => s,
            _ => panic!("Input is pre validated"),
        }
    }
    pub const fn as_bytes(&self) -> &[u8; 10] {
        &self.0
    }

    pub const fn as_u128(self) -> u128 {
        let mut le_bytes = [0; 16];

        let mut i = 0;
        while i < 10 {
            le_bytes[i + 6] = self.0[i];
            i += 1;
        }
        u128::from_le_bytes(le_bytes)
    }
    pub const fn from_bytes(bytes: &[u8]) -> Result<Self, NmiError> {
        let Ok(input_str) = str::from_utf8(bytes) else {
            return Err(NmiError {
                input: FixedString::default(),
                kind: NmiErrorKind::NonAsciiCharacters,
            });
        };

        let input = input_from_str(input_str);

        if bytes.len() > 11 {
            return Err(NmiError {
                input,
                kind: NmiErrorKind::TooLong(bytes.len()),
            });
        } else if bytes.len() < 10 {
            return Err(NmiError {
                input,
                kind: NmiErrorKind::TooShort(bytes.len()),
            });
        }

        // we know length is reasonable here

        let mut i = 0;
        while i < 10 {
            if !bytes[i].is_ascii_alphanumeric() {
                return Err(NmiError {
                    input,
                    kind: NmiErrorKind::NonAsciiCharacters,
                });
            }
            i += 1;
        }

        let mut nmi = [b'0'; 10];

        let mut i = 0;
        while i < 10 {
            let b = bytes[i];

            nmi[i] = match NmiChar::try_new(b) {
                Ok(_) => b,
                Err(kind) => return Err(NmiError { input, kind }),
            };

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
            if self.0[i].is_ascii_alphabetic() {
                return false;
            }
            i += 1;
        }
        true
    }
    pub fn chars(self) -> impl Iterator<Item = NmiChar> {
        self.0.into_iter().map(NmiChar::new)
    }
    pub const fn checksum(self) -> u8 {
        // https://stripe.com/au/resources/more/how-to-use-the-luhn-algorithm-a-guide-in-applications-for-businesses
        // https://en.wikipedia.org/wiki/Luhn_algorithm
        let mut checksum_counter = 0_u32;
        let mut i = 0;
        while i < 10 {
            // doubled ascii values up to Z are all less than 200
            // so we only need 0th, 1st and 2nd digits.

            let ascii_val = self.0[i] as u32;

            // instead of going backwards... we just double odd indexes
            if i % 2 != 0 {
                checksum_counter += const_utils::sum_digits(4, 2 * ascii_val).unwrap();
            } else {
                checksum_counter += const_utils::sum_digits(4, ascii_val).unwrap();
            }
            i += 1;
        }

        let next_highest_multiple_10 = 10 * (checksum_counter / 10 + 1);
        let diff = next_highest_multiple_10 - checksum_counter;

        match const_utils::nth_digit(0, diff) {
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
            buf[i] = self.0[i];
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

        // note that the order here _must not_ be changed
        // as otherwise if ranges are matched in the wrong order, invalid regions will be returned
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
    input: FixedString<20>,
    kind: NmiErrorKind,
}

#[derive(Debug)]
pub enum NmiErrorKind {
    NonAsciiCharacters,
    TooLong(usize),
    TooShort(usize),
    DisallowedCharacter(u8),
    InvalidChecksum { expected: u8 },
}

impl Display for NmiErrorKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            NmiErrorKind::NonAsciiCharacters => f.write_str("Non ascii characters"),
            NmiErrorKind::TooLong(len) => write!(f, "Too long: got {len} but expected 10 or 11"),
            NmiErrorKind::TooShort(len) => write!(f, "Too short: got {len} but expected 10 or 11"),
            NmiErrorKind::DisallowedCharacter(c) => write!(
                f,
                "Disallowed character: {:?} from byte {c}",
                str::from_utf8(&[*c])
            ),
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

const fn input_from_str(s: &str) -> FixedString<20> {
    match FixedString::from_str(s) {
        Ok(x) => x,
        Err(_) => FixedString::default(),
    }
}

impl FromStr for Nmi {
    type Err = NmiError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes())
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

const IDX_TO_NMI_CHAR: [NmiChar; 34] = [
    NmiChar::new(b'0'),
    NmiChar::new(b'1'),
    NmiChar::new(b'2'),
    NmiChar::new(b'3'),
    NmiChar::new(b'4'),
    NmiChar::new(b'5'),
    NmiChar::new(b'6'),
    NmiChar::new(b'7'),
    NmiChar::new(b'8'),
    NmiChar::new(b'9'),
    NmiChar::new(b'A'),
    NmiChar::new(b'B'),
    NmiChar::new(b'C'),
    NmiChar::new(b'D'),
    NmiChar::new(b'E'),
    NmiChar::new(b'F'),
    NmiChar::new(b'G'),
    NmiChar::new(b'H'),
    NmiChar::new(b'J'),
    NmiChar::new(b'K'),
    NmiChar::new(b'L'),
    NmiChar::new(b'M'),
    NmiChar::new(b'N'),
    NmiChar::new(b'P'),
    NmiChar::new(b'Q'),
    NmiChar::new(b'R'),
    NmiChar::new(b'S'),
    NmiChar::new(b'T'),
    NmiChar::new(b'U'),
    NmiChar::new(b'V'),
    NmiChar::new(b'W'),
    NmiChar::new(b'X'),
    NmiChar::new(b'Y'),
    NmiChar::new(b'Z'),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PackedNmi(u64);

impl PackedNmi {
    pub const fn try_from_bytes(by: &[u8]) -> Option<PackedNmi> {
        let Ok(nmi) = Nmi::from_bytes(by) else {
            return None;
        };
        Some(PackedNmi::pack(nmi))
    }
    pub const fn try_new(num: u64) -> Option<PackedNmi> {
        let mut i = 0;
        while i < 10 {
            // we have packed using the lowest index as the highest bit, etc.
            let translation = 4 + (9 - i) * 6;

            let bits = (num >> translation) & 0b_111_111;

            if bits > 34 {
                return None;
            }

            i += 1;
        }
        Some(PackedNmi(num))
    }
    pub const fn get(self) -> u64 {
        self.0
    }
    pub const fn pack(nmi: Nmi) -> PackedNmi {
        let mut packed = 0;
        let mut i = 0;
        while i < 10 {
            // we want to pack the lowest index as the highest bit, etc.
            let translation = 4 + (9 - i) * 6;
            let bits = NmiChar::new(nmi.0[i]).index() << translation;
            packed |= bits;
            i += 1;
        }
        PackedNmi(packed)
    }
    pub const fn unpack(self) -> Nmi {
        let mut nmi = [b'0'; 10];

        let mut i = 0;
        while i < 10 {
            // we have packed using the lowest index as the highest bit, etc.
            let translation = 4 + (9 - i) * 6;

            let bits = (self.0 >> translation) & 0b_111_111;

            assert!(bits < 34);

            nmi[i] = IDX_TO_NMI_CHAR[bits as usize].byte();
            i += 1;
        }
        Nmi(nmi)
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;
    use std::string::ToString;

    #[test]
    fn test_nmis() {
        assert!(
            PackedNmi::try_from_bytes("4316854005".as_bytes())
                < PackedNmi::try_from_bytes("QAAAVZZZZZ".as_bytes())
        );

        let nmis = [
            ("1234C6789A", b'3'),
            ("2001985732", b'8'),
            ("QAAAVZZZZZ", b'3'),
            ("2001985733", b'6'),
            ("QCDWW00010", b'2'),
            ("3075621875", b'8'),
            ("SMVEW00085", b'8'),
            ("3075621876", b'6'),
            ("VAAA000065", b'7'),
            ("4316854005", b'9'),
            ("VAAA000066", b'5'),
            ("4316854006", b'7'),
            ("VAAA000067", b'2'),
            ("6305888444", b'6'),
            ("VAAASTY576", b'8'),
            ("6350888444", b'2'),
            ("VCCCX00009", b'1'),
            ("7001888333", b'8'),
            ("VEEEX00009", b'1'),
            ("7102000001", b'7'),
            ("VKTS786150", b'2'),
            ("NAAAMYS582", b'6'),
            ("VKTS867150", b'5'),
            ("NBBBX11110", b'0'),
            ("VKTS871650", b'7'),
            ("NBBBX11111", b'8'),
            ("VKTS876105", b'7'),
            ("NCCC519495", b'5'),
            ("VKTS876150", b'3'),
            ("NGGG000055", b'4'),
            ("VKTS876510", b'8'),
        ];

        for (nmi, checksum) in nmis {
            let parsed = Nmi::from_bytes(nmi.as_bytes()).unwrap();

            // roundtrip
            assert_eq!(parsed.as_str(), nmi);
            assert_eq!(parsed.to_string(), nmi);

            // checksum matches
            assert_eq!(parsed.checksum(), checksum);

            // packing roundtrip
            assert_eq!(parsed, PackedNmi::pack(parsed).unpack());
            assert_eq!(
                PackedNmi::pack(parsed),
                PackedNmi::try_new(PackedNmi::pack(parsed).get()).unwrap()
            );
        }
    }
}
