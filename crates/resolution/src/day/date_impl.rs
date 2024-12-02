use core::ops::{Add, AddAssign, Sub, SubAssign};

use alloc::{format, string::String};

extern crate alloc;

#[path = "tests.rs"]
#[cfg(test)]
mod tests;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
// days since 1900-01-01
pub struct Day(pub(super) i32);

impl Day {
    pub const fn succ_n(self, n: u16) -> Day {
        Day(self.0 + (n as i32))
    }
    pub const fn pred_n(self, n: u16) -> Day {
        Day(self.0 - (n as i32))
    }
    pub const fn succ(self) -> Day {
        Day(self.0 + 1)
    }
    pub const fn pred(self) -> Day {
        Day(self.0 - 1)
    }
    pub const fn first_on_year(year: i32) -> Day {
        first_on_year(year)
    }
    pub const fn last_on_year(year: i32) -> Day {
        first_on_year(year + 1).pred_n(1)
    }

    pub const fn first_on_month(year: i32, month: MonthOfYear) -> Day {
        first_on_year(year).succ_n(month.cumulative_days(year))
    }

    pub const fn last_on_month(year: i32, month: MonthOfYear) -> Day {
        Self::first_on_month(year, month).succ_n(month.num_days(year) as u16 - 1)
    }
    // this is limited to only the 28th day
    pub const fn ymd(year: i32, month: MonthOfYear, day: DayOfMonth) -> Day {
        Self::first_on_month(year, month).succ_n(day.number() as u16 - 1)
    }
    pub const fn with_day(self, day: DayOfMonth) -> Day {
        let current_day = self.day_of_month();
        if current_day == day.number() {
            return self;
        }
        if current_day < day.number() {
            self.succ_n((day.number() - current_day) as u16)
        } else {
            self.pred_n((current_day - day.number()) as u16)
        }
    }
    pub const fn year_num(&self) -> i32 {
        self.through().year
    }
    pub const fn month_of_year(&self) -> MonthOfYear {
        self.through().month()
    }
    pub const fn day_of_month(&self) -> u8 {
        self.through().day()
    }
    pub const fn through(&self) -> YearAndDays {
        YearAndDays::calculate(*self)
    }
    pub const fn new(days: i32) -> Day {
        Day(days)
    }
    pub const fn inner(self) -> i32 {
        self.0
    }
    pub const fn to_ymd(self) -> (i32, u8, u8) {
        let through = self.through();
        (through.year, through.month().number(), through.day())
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
impl From<chrono::NaiveDate> for Day {
    fn from(value: chrono::NaiveDate) -> Self {
        Day::from_chrono_date(value)
    }
}

#[cfg(feature = "chrono")]
impl From<Day> for chrono::NaiveDate {
    fn from(value: Day) -> Self {
        value.chrono_date()
    }
}

impl Add<i32> for Day {
    type Output = Day;

    fn add(self, rhs: i32) -> Self::Output {
        Day(self.0 + rhs)
    }
}

impl Add<Day> for i32 {
    type Output = Day;

    fn add(self, rhs: Day) -> Self::Output {
        Day(self + rhs.0)
    }
}

impl AddAssign<i32> for Day {
    fn add_assign(&mut self, rhs: i32) {
        self.0 += rhs;
    }
}

impl SubAssign<i32> for Day {
    fn sub_assign(&mut self, rhs: i32) {
        self.0 -= rhs;
    }
}

impl Sub<i32> for Day {
    type Output = Day;

    fn sub(self, rhs: i32) -> Self::Output {
        Day(self.0 - rhs)
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
    pub const fn offset(&self) -> u8 {
        self.number() - 1
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
        // best case
        "Jan" | "jan" => Ok(MonthOfYear::Jan),
        "Feb" | "feb" => Ok(MonthOfYear::Feb),
        "Mar" | "mar" => Ok(MonthOfYear::Mar),
        "Apr" | "apr" => Ok(MonthOfYear::Apr),
        "May" | "may" => Ok(MonthOfYear::May),
        "Jun" | "jun" => Ok(MonthOfYear::Jun),
        "Jul" | "jul" => Ok(MonthOfYear::Jul),
        "Aug" | "aug" => Ok(MonthOfYear::Aug),
        "Sep" | "sep" => Ok(MonthOfYear::Sep),
        "Oct" | "oct" => Ok(MonthOfYear::Oct),
        "Nov" | "nov" => Ok(MonthOfYear::Nov),
        "Dec" | "dec" => Ok(MonthOfYear::Dec),
        // lest performant but more flexible
        m if m.starts_with("Jan") || m.starts_with("jan") => Ok(MonthOfYear::Jan),
        m if m.starts_with("Feb") || m.starts_with("feb") => Ok(MonthOfYear::Feb),
        m if m.starts_with("Mar") || m.starts_with("mar") => Ok(MonthOfYear::Mar),
        m if m.starts_with("Apr") || m.starts_with("apr") => Ok(MonthOfYear::Apr),
        m if m.starts_with("May") || m.starts_with("may") => Ok(MonthOfYear::May),
        m if m.starts_with("Jun") || m.starts_with("jun") => Ok(MonthOfYear::Jun),
        m if m.starts_with("Jul") || m.starts_with("jul") => Ok(MonthOfYear::Jul),
        m if m.starts_with("Aug") || m.starts_with("aug") => Ok(MonthOfYear::Aug),
        m if m.starts_with("Sep") || m.starts_with("sep") => Ok(MonthOfYear::Sep),
        m if m.starts_with("Oct") || m.starts_with("oct") => Ok(MonthOfYear::Oct),
        m if m.starts_with("Nov") || m.starts_with("nov") => Ok(MonthOfYear::Nov),
        m if m.starts_with("Dec") || m.starts_with("dec") => Ok(MonthOfYear::Dec),
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
    pub const fn from_number(m: u8) -> Option<MonthOfYear> {
        match m {
            1 => Some(MonthOfYear::Jan),
            2 => Some(MonthOfYear::Feb),
            3 => Some(MonthOfYear::Mar),
            4 => Some(MonthOfYear::Apr),
            5 => Some(MonthOfYear::May),
            6 => Some(MonthOfYear::Jun),
            7 => Some(MonthOfYear::Jul),
            8 => Some(MonthOfYear::Aug),
            9 => Some(MonthOfYear::Sep),
            10 => Some(MonthOfYear::Oct),
            11 => Some(MonthOfYear::Nov),
            12 => Some(MonthOfYear::Dec),
            _ => None,
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
        self.number() - 1
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
    pub const fn month(&self) -> MonthOfYear {
        if self.leap {
            match self.days_through {
                0..31 => MonthOfYear::Jan,
                31..60 => MonthOfYear::Feb,
                60..91 => MonthOfYear::Mar,
                91..121 => MonthOfYear::Apr,
                121..152 => MonthOfYear::May,
                152..182 => MonthOfYear::Jun,
                182..213 => MonthOfYear::Jul,
                213..244 => MonthOfYear::Aug,
                244..274 => MonthOfYear::Sep,
                274..305 => MonthOfYear::Oct,
                305..335 => MonthOfYear::Nov,
                335..366 => MonthOfYear::Dec,
                _ => panic!("out of range"),
            }
        } else {
            match self.days_through {
                0..31 => MonthOfYear::Jan,
                31..59 => MonthOfYear::Feb,
                59..90 => MonthOfYear::Mar,
                90..120 => MonthOfYear::Apr,
                120..151 => MonthOfYear::May,
                151..181 => MonthOfYear::Jun,
                181..212 => MonthOfYear::Jul,
                212..243 => MonthOfYear::Aug,
                243..273 => MonthOfYear::Sep,
                273..304 => MonthOfYear::Oct,
                304..334 => MonthOfYear::Nov,
                334..365 => MonthOfYear::Dec,
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

const fn first_on_year(year: i32) -> Day {
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

    Day(year * 365 + cycles - mid_cycles + long_cycles + 1)
}

const B1: i32 = 1 * DAYS_PER_MOST_100Y + 1;
const B2: i32 = 2 * DAYS_PER_MOST_100Y + 1;
const B3: i32 = 3 * DAYS_PER_MOST_100Y + 1;
const B4: i32 = 4 * DAYS_PER_MOST_100Y + 1;

impl YearAndDays {
    const fn calculate(date: Day) -> YearAndDays {
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
    let date = Day::new(x);
    let calculated = YearAndDays::calculate(date);
    let roundtrip = first_on_year(calculated.year).succ_n(calculated.days_through as u16);
    assert_eq!(date, roundtrip);
}