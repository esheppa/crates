use alloc::{format, string::String};
use calendrical_calculations::{
    helpers::i64_to_i32,
    iso::{const_fixed_from_iso, is_leap_year, iso_from_fixed},
    rata_die::RataDie,
};
use core::ops::{Add, AddAssign, Sub, SubAssign};

use crate::time_of_day::{LocalDateTime, LocalTimeOfDay};

extern crate alloc;

pub mod time_of_day;

#[path = "tests.rs"]
#[cfg(test)]
mod tests;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Year(i32);

impl Year {
    pub const fn num(self) -> i32 {
        self.0
    }
    pub const fn new(y: i32) -> Self {
        Year(y)
    }
    pub const fn succ(self) -> Option<Self> {
        self.translate(1)
    }
    pub const fn pred(self) -> Option<Self> {
        self.translate(-1)
    }
    pub const fn translate(self, years: i32) -> Option<Self> {
        let Some(y) = self.0.checked_add(years) else {
            return None;
        };
        Some(Year::new(y))
    }
}

// NOTE: add MIN/MAX here??
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
// days since 1900-01-01
pub struct Date(i32);

impl core::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (y, m, d) = self.to_ymd();
        write!(f, "{y}-{m:02}-{d:02}")
    }
}

impl Date {
    pub const fn translate(self, i: i32) -> Option<Date> {
        let Some(d) = self.0.checked_add(i) else {
            return None;
        };
        Some(Date(d))
    }

    pub const fn succ(self) -> Option<Date> {
        self.translate(1)
    }
    pub const fn pred(self) -> Option<Date> {
        self.translate(-1)
    }
    pub const fn first_on_year(year: Year) -> Option<Date> {
        ymd_to_date(year.0, 1, 1)
    }
    pub const fn last_on_year(year: Year) -> Option<Date> {
        Self::last_on_month(year, MonthOfYear::Dec)
    }

    pub const fn first_on_month(year: Year, month: MonthOfYear) -> Option<Date> {
        ymd_to_date(year.0, month.number(), 1)
    }

    pub const fn last_on_month(year: Year, month: MonthOfYear) -> Option<Date> {
        ymd_to_date(year.0, month.number(), month.num_days(year))
    }

    // this is limited to only the 28th day
    pub const fn ymd(year: Year, month: MonthOfYear, day: DayOfMonth) -> Option<Date> {
        let Some(first) = Self::first_on_month(year, month) else {
            return None;
        };
        let Some(sub) = (day.number() as i32).checked_sub(1) else {
            return None;
        };

        first.translate(sub)
    }
    pub const fn with_day(self, day: DayOfMonth) -> Option<Date> {
        let current_day = self.day_of_month();
        if current_day == day.number() {
            return Some(self);
        }
        self.translate((day.number() - current_day) as i32)
    }
    pub const fn year(self) -> Year {
        Year::new(self.to_ymd().0)
    }
    pub const fn month_of_year(self) -> MonthOfYear {
        // NOTE: exhaustively tested
        MonthOfYear::from_number(self.to_ymd().1).unwrap()
    }
    pub const fn day_of_month(self) -> u8 {
        self.to_ymd().2
    }
    pub const fn new(days: i32) -> Date {
        Date(days)
    }
    pub const fn inner(self) -> i32 {
        self.0
    }
    pub const fn to_ymd(self) -> (i32, u8, u8) {
        date_to_ymd(self)
    }

    #[cfg(feature = "chrono")]
    pub const fn chrono_date(self) -> Option<chrono::NaiveDate> {
        chrono::NaiveDate::from_num_days_from_ce_opt(self.0)
    }
    #[cfg(feature = "chrono")]
    pub fn from_chrono_date(d: chrono::NaiveDate) -> Self {
        Self::new(chrono::Datelike::num_days_from_ce(&d))
    }

    pub const fn and_time(self, time: LocalTimeOfDay) -> LocalDateTime {
        LocalDateTime::new(self, time)
    }
    pub const fn start_time(self) -> LocalDateTime {
        self.and_time(LocalTimeOfDay::start())
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
    pub const fn num_days(self, year: Year) -> u8 {
        match self {
            MonthOfYear::Jan => 31,
            MonthOfYear::Feb if is_leap_year(year.0) => 29,
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

    pub const fn cumulative_days(self, year: Year) -> i32 {
        if is_leap_year(year.0) {
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

const fn ymd_to_date(year: i32, month: u8, day: u8) -> Option<Date> {
    let rd = const_fixed_from_iso(year, month, day);

    match i64_to_i32(rd.to_i64_date()) {
        Ok(d) => Some(Date::new(d)),
        Err(_) => None,
    }
}

const fn date_to_ymd(date: Date) -> (i32, u8, u8) {
    match iso_from_fixed(RataDie::new(date.0 as i64)) {
        Ok(x) => x,
        Err(_) => panic!("unable to convert date to YMD"),
    }
}

#[cfg(kani)]
#[kani::proof]
pub fn verify_roundtrip() {
    let x: i32 = kani::any();
    let date = Date::new(x);
    let calculated = YearAndDays::calculate(date);
    let roundtrip = first_on_year_internal(calculated.year).succ_n(calculated.days_through as u16);
    assert_eq!(date, roundtrip);
}
