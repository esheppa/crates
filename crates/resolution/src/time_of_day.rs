// exists for formatting and convenience only, no operations can be done to this, has to be converted to a Minutes<N> or similar first

// also explicitly ignores leap seconds

// represents the local time of day

use crate::Day;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct LocalDateTime(Day, LocalTimeOfDay);

impl LocalDateTime {
    pub const fn new(day: Day, time: LocalTimeOfDay) -> Self {
        Self(day, time)
    }
    pub const fn day(self) -> Day {
        self.0
    }
    pub const fn time(self) -> LocalTimeOfDay {
        self.1
    }

    #[cfg(feature = "chrono")]
    pub const fn chrono_datetime(self) -> chrono::NaiveDateTime {
        self.0.chrono_date().and_time(self.1.chrono_time())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct LocalTimeOfDay(HourOfDay, MinuteOfHour);

impl LocalTimeOfDay {
    pub const fn hour(self) -> HourOfDay {
        self.0
    }
    pub const fn minute(self) -> MinuteOfHour {
        self.1
    }
    pub const fn start() -> LocalTimeOfDay {
        LocalTimeOfDay(HourOfDay::H0, MinuteOfHour::M0)
    }
    #[cfg(feature = "chrono")]
    pub const fn chrono_time(self) -> chrono::NaiveTime {
        chrono::NaiveTime::MIN
            .overflowing_add_signed(chrono::TimeDelta::minutes(
                (self.0.number() as i64 * 60) + self.1.number() as i64,
            ))
            .0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
#[rustfmt::skip]
pub enum MinuteOfHour {
    M0,  M1,  M2,  M3,  M4,  M5,  M6,  M7,  M8,  M9, 
    M10, M11, M12, M13, M14, M15, M16, M17, M18, M19,
    M20, M21, M22, M23, M24, M25, M26, M27, M28, M29,
    M30, M31, M32, M33, M34, M35, M36, M37, M38, M39,
    M40, M41, M42, M43, M44, M45, M46, M47, M48, M49,
    M50, M51, M52, M53, M54, M55, M56, M57, M58, M59,
}

impl MinuteOfHour {
    pub const fn number(&self) -> u8 {
        match self {
            MinuteOfHour::M0 => 0,
            MinuteOfHour::M1 => 1,
            MinuteOfHour::M2 => 2,
            MinuteOfHour::M3 => 3,
            MinuteOfHour::M4 => 4,
            MinuteOfHour::M5 => 5,
            MinuteOfHour::M6 => 6,
            MinuteOfHour::M7 => 7,
            MinuteOfHour::M8 => 8,
            MinuteOfHour::M9 => 9,
            MinuteOfHour::M10 => 10,
            MinuteOfHour::M11 => 11,
            MinuteOfHour::M12 => 12,
            MinuteOfHour::M13 => 13,
            MinuteOfHour::M14 => 14,
            MinuteOfHour::M15 => 15,
            MinuteOfHour::M16 => 16,
            MinuteOfHour::M17 => 17,
            MinuteOfHour::M18 => 18,
            MinuteOfHour::M19 => 19,
            MinuteOfHour::M20 => 20,
            MinuteOfHour::M21 => 21,
            MinuteOfHour::M22 => 22,
            MinuteOfHour::M23 => 23,
            MinuteOfHour::M24 => 24,
            MinuteOfHour::M25 => 25,
            MinuteOfHour::M26 => 26,
            MinuteOfHour::M27 => 27,
            MinuteOfHour::M28 => 28,
            MinuteOfHour::M29 => 29,
            MinuteOfHour::M30 => 30,
            MinuteOfHour::M31 => 31,
            MinuteOfHour::M32 => 32,
            MinuteOfHour::M33 => 33,
            MinuteOfHour::M34 => 34,
            MinuteOfHour::M35 => 35,
            MinuteOfHour::M36 => 36,
            MinuteOfHour::M37 => 37,
            MinuteOfHour::M38 => 38,
            MinuteOfHour::M39 => 39,
            MinuteOfHour::M40 => 40,
            MinuteOfHour::M41 => 41,
            MinuteOfHour::M42 => 42,
            MinuteOfHour::M43 => 43,
            MinuteOfHour::M44 => 44,
            MinuteOfHour::M45 => 45,
            MinuteOfHour::M46 => 46,
            MinuteOfHour::M47 => 47,
            MinuteOfHour::M48 => 48,
            MinuteOfHour::M49 => 49,
            MinuteOfHour::M50 => 50,
            MinuteOfHour::M51 => 51,
            MinuteOfHour::M52 => 52,
            MinuteOfHour::M53 => 53,
            MinuteOfHour::M54 => 54,
            MinuteOfHour::M55 => 55,
            MinuteOfHour::M56 => 56,
            MinuteOfHour::M57 => 57,
            MinuteOfHour::M58 => 58,
            MinuteOfHour::M59 => 59,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
#[rustfmt::skip]
pub enum HourOfDay {
    H0,  H1,  H2,  H3,  H4,  H5,  H6,  H7,  H8,  H9,
    H10, H11, H12, H13, H14, H15, H16, H17, H18, H19,
    H20, H21, H22, H23,
}

impl HourOfDay {
    pub const fn and_minute(self, minute: MinuteOfHour) -> LocalTimeOfDay {
        LocalTimeOfDay(self, minute)
    }
    pub const fn start_time(self) -> LocalTimeOfDay {
        LocalTimeOfDay(self, MinuteOfHour::M0)
    }
    pub const fn number(&self) -> u8 {
        match self {
            HourOfDay::H0 => 0,
            HourOfDay::H1 => 1,
            HourOfDay::H2 => 2,
            HourOfDay::H3 => 3,
            HourOfDay::H4 => 4,
            HourOfDay::H5 => 5,
            HourOfDay::H6 => 6,
            HourOfDay::H7 => 7,
            HourOfDay::H8 => 8,
            HourOfDay::H9 => 9,
            HourOfDay::H10 => 10,
            HourOfDay::H11 => 11,
            HourOfDay::H12 => 12,
            HourOfDay::H13 => 13,
            HourOfDay::H14 => 14,
            HourOfDay::H15 => 15,
            HourOfDay::H16 => 16,
            HourOfDay::H17 => 17,
            HourOfDay::H18 => 18,
            HourOfDay::H19 => 19,
            HourOfDay::H20 => 20,
            HourOfDay::H21 => 21,
            HourOfDay::H22 => 22,
            HourOfDay::H23 => 23,
        }
    }
}
