#![no_std]

use core::ops::{Add, AddAssign, Sub, SubAssign};

use alloc::{format, string::String};

extern crate alloc;

#[path = "tests.rs"]
#[cfg(test)]
mod tests;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
// days since 1900-01-01
pub struct Date(i32);

impl Date {
    pub const fn succ_n(self, n: u16) -> Date {
        Date(self.0 + (n as i32))
    }
    pub const fn pred_n(self, n: u16) -> Date {
        Date(self.0 - (n as i32))
    }
    pub const fn first_on_year(year: i32) -> Date {
        first_on_year(year)
    }
    pub const fn last_on_year(year: i32) -> Date {
        first_on_year(year + 1).pred_n(1)
    }

    pub const fn first_on_month(year: i32, month: MonthOfYear) -> Date {
        first_on_year(year).succ_n(month.cumulative_days(year))
    }

    pub const fn last_on_month(year: i32, month: MonthOfYear) -> Date {
        Self::first_on_month(year, month).succ_n(month.num_days(year) as u16 - 1)
    }
    // this is limited to only the 28th day
    pub const fn ymd(year: i32, month: MonthOfYear, day: DayOfMonth) -> Date {
        Self::first_on_month(year, month).succ_n(day.number() as u16 - 1)
    }
    pub const fn with_day(self, day: DayOfMonth) -> Date {
        let current_day = self.day();
        if current_day == day.number() {
            return self;
        }
        if current_day < day.number() {
            self.succ_n((day.number() - current_day) as u16)
        } else {
            self.pred_n((current_day - day.number()) as u16)
        }
    }
    pub const fn year(&self) -> i32 {
        self.through().year
    }
    pub const fn month(&self) -> u8 {
        self.through().month()
    }

    pub const fn day(&self) -> u8 {
        self.through().day()
    }
    pub const fn through(&self) -> YearAndDays {
        YearAndDays::calculate(*self)
    }
    pub const fn new(days: i32) -> Date {
        Date(days)
    }
    pub const fn inner(self) -> i32 {
        self.0
    }
    pub const fn to_ymd(self) -> (i32, u8, u8) {
        let through = self.through();
        (through.year, through.month(), through.day())
    }

    #[cfg(feature = "chrono")]
    pub const fn chrono_date(self) -> chrono::NaiveDate {
        match chrono::NaiveDate::from_num_days_from_ce_opt(self.0 - 365) {
            Some(d) => d,
            None => panic!("Invalid date"),
        }
    }
    #[cfg(feature = "chrono")]
    pub fn from_chrono_date(d: chrono::NaiveDate) -> Self {
        Self::new(chrono::Datelike::num_days_from_ce(&d) + 365)
    }
}

#[cfg(feature = "chrono")]
impl From<chrono::NaiveDate> for Date {
    fn from(value: chrono::NaiveDate) -> Self {
        Date::from_chrono_date(value)
    }
}

#[cfg(feature = "chrono")]
impl From<Date> for chrono::NaiveDate {
    fn from(value: Date) -> Self {
        value.chrono_date()
    }
}

impl Add<i32> for Date {
    type Output = Date;

    fn add(self, rhs: i32) -> Self::Output {
        Date(self.0 + rhs)
    }
}

impl Add<Date> for i32 {
    type Output = Date;

    fn add(self, rhs: Date) -> Self::Output {
        Date(self + rhs.0)
    }
}

impl AddAssign<i32> for Date {
    fn add_assign(&mut self, rhs: i32) {
        self.0 += rhs;
    }
}

impl SubAssign<i32> for Date {
    fn sub_assign(&mut self, rhs: i32) {
        self.0 -= rhs;
    }
}

impl Sub<i32> for Date {
    type Output = Date;

    fn sub(self, rhs: i32) -> Self::Output {
        Date(self.0 - rhs)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]

pub enum DayOfMonth {
    D1,
    D2,
    D3,
    D4,
    D5,
    D6,
    D7,
    D8,
    D9,
    D10,
    D11,
    D12,
    D13,
    D14,
    D15,
    D16,
    D17,
    D18,
    D19,
    D20,
    D21,
    D22,
    D23,
    D24,
    D25,
    D26,
    D27,
    D28,
}

impl DayOfMonth {
    pub const fn number(&self) -> u8 {
        match self {
            DayOfMonth::D1 => 1,
            DayOfMonth::D2 => 2,
            DayOfMonth::D3 => 3,
            DayOfMonth::D4 => 4,
            DayOfMonth::D5 => 5,
            DayOfMonth::D6 => 6,
            DayOfMonth::D7 => 7,
            DayOfMonth::D8 => 8,
            DayOfMonth::D9 => 9,
            DayOfMonth::D10 => 10,
            DayOfMonth::D11 => 11,
            DayOfMonth::D12 => 12,
            DayOfMonth::D13 => 13,
            DayOfMonth::D14 => 14,
            DayOfMonth::D15 => 15,
            DayOfMonth::D16 => 16,
            DayOfMonth::D17 => 17,
            DayOfMonth::D18 => 18,
            DayOfMonth::D19 => 19,
            DayOfMonth::D20 => 20,
            DayOfMonth::D21 => 21,
            DayOfMonth::D22 => 22,
            DayOfMonth::D23 => 23,
            DayOfMonth::D24 => 24,
            DayOfMonth::D25 => 25,
            DayOfMonth::D26 => 26,
            DayOfMonth::D27 => 27,
            DayOfMonth::D28 => 28,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum MonthOfYear {
    Jan,
    Feb,
    Mar,
    Apr,
    May,
    Jun,
    Jul,
    Aug,
    Sep,
    Oct,
    Nov,
    Dec,
}

#[cfg(feature = "chrono")]
impl From<MonthOfYear> for chrono::Month {
    fn from(value: MonthOfYear) -> Self {
        value.chrono_month()
    }
}

#[cfg(feature = "chrono")]
impl From<chrono::Month> for MonthOfYear {
    fn from(value: chrono::Month) -> Self {
        Self::from_chrono_month(value)
    }
}

pub fn month_of_year_from_name(name: &str) -> Result<MonthOfYear, String> {
    match name {
        "Jan" => Ok(MonthOfYear::Jan),
        "Feb" => Ok(MonthOfYear::Feb),
        "Mar" => Ok(MonthOfYear::Mar),
        "Apr" => Ok(MonthOfYear::Apr),
        "May" => Ok(MonthOfYear::May),
        "Jun" => Ok(MonthOfYear::Jun),
        "Jul" => Ok(MonthOfYear::Jul),
        "Aug" => Ok(MonthOfYear::Aug),
        "Sep" => Ok(MonthOfYear::Sep),
        "Oct" => Ok(MonthOfYear::Oct),
        "Nov" => Ok(MonthOfYear::Nov),
        "Dec" => Ok(MonthOfYear::Dec),
        _ => Err(format!("Unexpected month name: `{name}`")),
    }
}

impl MonthOfYear {
    #[cfg(feature = "chrono")]
    pub const fn chrono_month(self) -> chrono::Month {
        match self {
            MonthOfYear::Jan => chrono::Month::January,
            MonthOfYear::Feb => chrono::Month::February,
            MonthOfYear::Mar => chrono::Month::March,
            MonthOfYear::Apr => chrono::Month::April,
            MonthOfYear::May => chrono::Month::May,
            MonthOfYear::Jun => chrono::Month::June,
            MonthOfYear::Jul => chrono::Month::July,
            MonthOfYear::Aug => chrono::Month::August,
            MonthOfYear::Sep => chrono::Month::September,
            MonthOfYear::Oct => chrono::Month::October,
            MonthOfYear::Nov => chrono::Month::November,
            MonthOfYear::Dec => chrono::Month::December,
        }
    }
    #[cfg(feature = "chrono")]
    pub const fn from_chrono_month(mth: chrono::Month) -> Self {
        match mth {
            chrono::Month::January => MonthOfYear::Jan,
            chrono::Month::February => MonthOfYear::Feb,
            chrono::Month::March => MonthOfYear::Mar,
            chrono::Month::April => MonthOfYear::Apr,
            chrono::Month::May => MonthOfYear::May,
            chrono::Month::June => MonthOfYear::Jun,
            chrono::Month::July => MonthOfYear::Jul,
            chrono::Month::August => MonthOfYear::Aug,
            chrono::Month::September => MonthOfYear::Sep,
            chrono::Month::October => MonthOfYear::Oct,
            chrono::Month::November => MonthOfYear::Nov,
            chrono::Month::December => MonthOfYear::Dec,
        }
    }
    pub const fn num_days(self, year: i32) -> u8 {
        match self {
            MonthOfYear::Jan => 31,
            MonthOfYear::Feb if is_leap_year(year) => 29,
            MonthOfYear::Feb => 28,
            MonthOfYear::Mar => 31,
            MonthOfYear::Apr => 30,
            MonthOfYear::May => 31,
            MonthOfYear::Jun => 30,
            MonthOfYear::Jul => 31,
            MonthOfYear::Aug => 31,
            MonthOfYear::Sep => 30,
            MonthOfYear::Oct => 31,
            MonthOfYear::Nov => 30,
            MonthOfYear::Dec => 31,
        }
    }

    pub const fn cumulative_days(self, year: i32) -> u16 {
        if is_leap_year(year) {
            match self {
                MonthOfYear::Jan => 0,
                MonthOfYear::Feb => 31,
                MonthOfYear::Mar => 60,
                MonthOfYear::Apr => 91,
                MonthOfYear::May => 121,
                MonthOfYear::Jun => 152,
                MonthOfYear::Jul => 182,
                MonthOfYear::Aug => 213,
                MonthOfYear::Sep => 244,
                MonthOfYear::Oct => 274,
                MonthOfYear::Nov => 305,
                MonthOfYear::Dec => 335,
            }
        } else {
            match self {
                MonthOfYear::Jan => 0,
                MonthOfYear::Feb => 31,
                MonthOfYear::Mar => 59,
                MonthOfYear::Apr => 90,
                MonthOfYear::May => 120,
                MonthOfYear::Jun => 151,
                MonthOfYear::Jul => 181,
                MonthOfYear::Aug => 212,
                MonthOfYear::Sep => 243,
                MonthOfYear::Oct => 273,
                MonthOfYear::Nov => 304,
                MonthOfYear::Dec => 334,
            }
        }
    }
    pub const fn name(self) -> &'static str {
        match self {
            MonthOfYear::Jan => "Jan",
            MonthOfYear::Feb => "Feb",
            MonthOfYear::Mar => "Mar",
            MonthOfYear::Apr => "Apr",
            MonthOfYear::May => "May",
            MonthOfYear::Jun => "Jun",
            MonthOfYear::Jul => "Jul",
            MonthOfYear::Aug => "Aug",
            MonthOfYear::Sep => "Sep",
            MonthOfYear::Oct => "Oct",
            MonthOfYear::Nov => "Nov",
            MonthOfYear::Dec => "Dec",
        }
    }
    pub const fn number(self) -> u8 {
        match self {
            MonthOfYear::Jan => 1,
            MonthOfYear::Feb => 2,
            MonthOfYear::Mar => 3,
            MonthOfYear::Apr => 4,
            MonthOfYear::May => 5,
            MonthOfYear::Jun => 6,
            MonthOfYear::Jul => 7,
            MonthOfYear::Aug => 8,
            MonthOfYear::Sep => 9,
            MonthOfYear::Oct => 10,
            MonthOfYear::Nov => 11,
            MonthOfYear::Dec => 12,
        }
    }

    pub const fn months_from_jan(self) -> u8 {
        match self {
            MonthOfYear::Jan => 0,
            MonthOfYear::Feb => 1,
            MonthOfYear::Mar => 2,
            MonthOfYear::Apr => 3,
            MonthOfYear::May => 4,
            MonthOfYear::Jun => 5,
            MonthOfYear::Jul => 6,
            MonthOfYear::Aug => 7,
            MonthOfYear::Sep => 8,
            MonthOfYear::Oct => 9,
            MonthOfYear::Nov => 10,
            MonthOfYear::Dec => 11,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct YearAndDays {
    year: i32,
    leap: bool,
    days_through: i32,
}

impl YearAndDays {
    pub const fn leap(self) -> bool {
        self.leap
    }
    pub const fn days_through(self) -> i32 {
        self.days_through
    }
    pub const fn month(&self) -> u8 {
        if self.leap {
            match self.days_through {
                0..31 => 1,
                31..60 => 2,
                60..91 => 3,
                91..121 => 4,
                121..152 => 5,
                152..182 => 6,
                182..213 => 7,
                213..244 => 8,
                244..274 => 9,
                274..305 => 10,
                305..335 => 11,
                335..366 => 12,
                _ => panic!("out of range"),
            }
        } else {
            match self.days_through {
                0..31 => 1,
                31..59 => 2,
                59..90 => 3,
                90..120 => 4,
                120..151 => 5,
                151..181 => 6,
                181..212 => 7,
                212..243 => 8,
                243..273 => 9,
                273..304 => 10,
                304..334 => 11,
                334..365 => 12,
                _ => panic!("out of range"),
            }
        }
    }
    pub const fn day(&self) -> u8 {
        let d = if self.leap {
            match self.days_through {
                0..31 => self.days_through + 1 - 0,
                31..60 => self.days_through + 1 - 31,
                60..91 => self.days_through + 1 - 60,
                91..121 => self.days_through + 1 - 91,
                121..152 => self.days_through + 1 - 121,
                152..182 => self.days_through + 1 - 152,
                182..213 => self.days_through + 1 - 182,
                213..244 => self.days_through + 1 - 213,
                244..274 => self.days_through + 1 - 244,
                274..305 => self.days_through + 1 - 274,
                305..335 => self.days_through + 1 - 305,
                335..366 => self.days_through + 1 - 335,
                _ => panic!("out of range"),
            }
        } else {
            match self.days_through {
                0..31 => self.days_through + 1 - 0,
                31..59 => self.days_through + 1 - 31,
                59..90 => self.days_through + 1 - 59,
                90..120 => self.days_through + 1 - 90,
                120..151 => self.days_through + 1 - 120,
                151..181 => self.days_through + 1 - 151,
                181..212 => self.days_through + 1 - 181,
                212..243 => self.days_through + 1 - 212,
                243..273 => self.days_through + 1 - 243,
                273..304 => self.days_through + 1 - 273,
                304..334 => self.days_through + 1 - 304,
                334..365 => self.days_through + 1 - 334,
                _ => panic!("out of range"),
            }
        };
        d as u8
    }
}

const DAYS_PER_400Y: i32 = 4 * DAYS_PER_MOST_100Y + 1;

const DAYS_PER_MOST_100Y: i32 = 25 * DAYS_PER_MOST_4Y - 1;

const DAYS_PER_MOST_4Y: i32 = 4 * 365 + 1;

pub const fn is_not_leap_year(year: i32) -> bool {
    year % 4 != 0 || (year % 100 == 0 && year % 400 != 0)
}
pub const fn is_leap_year(year: i32) -> bool {
    !is_not_leap_year(year)
}
pub const fn days_in_year(year: i32) -> i32 {
    if is_leap_year(year) {
        366
    } else {
        365
    }
}

#[derive(Debug)]
struct CycleSplit {
    year_diff: i32,
    days_thru_year: i32,
}

impl CycleSplit {
    const fn new(days: i32) -> CycleSplit {
        match days {
            0..366 => CycleSplit {
                year_diff: 0,
                days_thru_year: days,
            },
            366..731 => CycleSplit {
                year_diff: 1,
                days_thru_year: days - 366,
            },
            731..1096 => CycleSplit {
                year_diff: 2,
                days_thru_year: days - 731,
            },
            1096..1461 => CycleSplit {
                year_diff: 3,
                days_thru_year: days - 1096,
            },
            _ => panic!("out of range"),
        }
    }
}

const fn first_on_year(year: i32) -> Date {
    // if year < 1900 {
    //     panic!("Out of range")
    // }

    // let adj = year - 1900;

    // if adj == 0 {
    //     return Date(0);
    // }
    // how many 400ys - we subtract one because it is about how many of these that we have passed
    let long_cycles = (year - 1).div_euclid(400);

    // how many 100ys - we subtract one because it is about how many of these that we have passed
    let mid_cycles = (year - 1).div_euclid(100);

    // how many 4ys - we subtract one because it is about how many of these that we have passed
    let cycles = (year - 1).div_euclid(4);

    Date(year * 365 + cycles - mid_cycles + long_cycles + 1)
}

const B1: i32 = 1 * DAYS_PER_MOST_100Y + 1;
const B2: i32 = 2 * DAYS_PER_MOST_100Y + 1;
const B3: i32 = 3 * DAYS_PER_MOST_100Y + 1;
const B4: i32 = 4 * DAYS_PER_MOST_100Y + 1;

impl YearAndDays {
    const fn calculate(date: Date) -> YearAndDays {
        // figure out which 400y block we are in
        let block = date.0.div_euclid(DAYS_PER_400Y);
        let remainder = date.0.rem_euclid(DAYS_PER_400Y);
        #[cfg(kani)]
        assert!(remainder < DAYS_PER_400Y);

        let (ext_years, split) = match remainder {
            0..B1 => {
                // extra leap
                let days_thru_block = remainder;
                let cycles_through_block = days_thru_block / DAYS_PER_MOST_4Y;
                let days_through_cycle = days_thru_block % DAYS_PER_MOST_4Y;

                #[cfg(kani)]
                assert!(days_through_cycle <= 1460);

                let split = CycleSplit::new(days_through_cycle);
                (cycles_through_block * 4, split)
            }
            B1..B2 => {
                // regular

                // add extra day, will remove later...
                let days_thru_block = remainder - B1 + 1;
                let cycles_through_block = days_thru_block / DAYS_PER_MOST_4Y;
                let days_through_cycle = days_thru_block % DAYS_PER_MOST_4Y;

                #[cfg(kani)]
                assert!(days_through_cycle <= 1460);

                let mut split = CycleSplit::new(days_through_cycle);
                if split.year_diff == 0 && cycles_through_block == 0 {
                    split.days_thru_year -= 1;
                }
                (cycles_through_block * 4 + 100, split)
            }
            B2..B3 => {
                // regular
                // add extra day, will remove later...
                let days_thru_block = remainder - B2 + 1;
                let cycles_through_block = days_thru_block / DAYS_PER_MOST_4Y;
                let days_through_cycle = days_thru_block % DAYS_PER_MOST_4Y;

                #[cfg(kani)]
                assert!(days_through_cycle <= 1460);

                let mut split = CycleSplit::new(days_through_cycle);
                if split.year_diff == 0 && cycles_through_block == 0 {
                    split.days_thru_year -= 1;
                }
                (cycles_through_block * 4 + 200, split)
            }
            B3..B4 => {
                // regular
                // add extra day, will remove later...
                let days_thru_block = remainder - B3 + 1;
                let cycles_through_block = days_thru_block / DAYS_PER_MOST_4Y;
                let days_through_cycle = days_thru_block % DAYS_PER_MOST_4Y;

                #[cfg(kani)]
                assert!(days_through_cycle <= 1460);

                let mut split = CycleSplit::new(days_through_cycle);
                if split.year_diff == 0 && cycles_through_block == 0 {
                    split.days_thru_year -= 1;
                }
                (cycles_through_block * 4 + 300, split)
            }
            _ => {
                panic!("Out of range!")
            }
        };

        let proposed_year = 400 * block + ext_years + split.year_diff;

        if is_leap_year(proposed_year) {
            assert!(split.days_thru_year >= 0 && split.days_thru_year <= 366);
        } else {
            assert!(split.days_thru_year >= 0 && split.days_thru_year <= 365);
        }

        YearAndDays {
            year: proposed_year,
            leap: is_leap_year(proposed_year),
            days_through: split.days_thru_year,
        }
    }
}

#[cfg(kani)]
#[kani::proof]
pub fn verify_roundtrip() {
    let x: i32 = kani::any();
    let date = Date::new(x);
    let calculated = YearAndDays::calculate(date);
    let roundtrip = first_on_year(calculated.year).succ_n(calculated.days_through as u16);
    assert_eq!(date, roundtrip);
}
