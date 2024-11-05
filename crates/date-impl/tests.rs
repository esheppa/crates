extern crate std;
use chrono::{self, Datelike, Days, NaiveDate};
use std::dbg;

use crate::{first_on_year, is_leap_year, Date, DayOfMonth, MonthOfYear, YearAndDays};

#[test]
fn test_helpers() {
    assert!(is_leap_year(1904));
    assert!(is_leap_year(1996));
    assert!(is_leap_year(2000));
    assert!(!is_leap_year(1900));
}

#[test]
fn test_date() {
    let chrono_base = NaiveDate::from_ymd_opt(0, 1, 1).unwrap();

    for i in -2_000_000..2_000_000_i32 {
        let chrono_adj = if i >= 0 {
            chrono_base + Days::new(i as u64)
        } else {
            chrono_base - Days::new(-i as u64)
        };
        let date = Date(i);
        assert_eq!(chrono_adj.year(), date.year());
        assert_eq!(chrono_adj.month() as u8, date.month());
        assert_eq!(chrono_adj.day() as u8, date.day());

        #[cfg(feature = "chrono")]
        {
            assert_eq!(chrono_adj, date.chrono_date());
            assert_eq!(date, Date::from_chrono_date(chrono_adj));
        }
    }
}

#[test]
fn test_roundtrip_neg() {
    for i in i32::MIN..0 {
        let date = Date::new(i);
        let calculated = YearAndDays::calculate(date);
        let roundtrip = first_on_year(calculated.year).succ_n(calculated.days_through as u16);
        assert_eq!(date, roundtrip);
    }
}

#[test]
fn test_roundtrip_pos() {
    for i in 0..i32::MAX {
        let date = Date::new(i);
        let calculated = YearAndDays::calculate(date);
        let roundtrip = first_on_year(calculated.year).succ_n(calculated.days_through as u16);
        assert_eq!(date, roundtrip);
    }
}

#[test]
fn test_new() {
    for year in 1840..10_000 {
        let chrono_start = NaiveDate::from_ymd_opt(year, 1, 1).unwrap();
        let date_start = Date::first_on_year(year);
        dbg!(
            chrono_start,
            date_start.to_ymd(),
            date_start,
            NaiveDate::from_num_days_from_ce_opt(date_start.inner())
        );

        assert_eq!(chrono_start.year(), date_start.year());
        assert_eq!(chrono_start.month() as u8, date_start.month());
        assert_eq!(chrono_start.day() as u8, date_start.day());

        let chrono_end = NaiveDate::from_ymd_opt(year, 12, 31).unwrap();
        let date_end = Date::last_on_year(year);

        dbg!(
            chrono_end,
            date_end.to_ymd(),
            year,
            date_end,
            date_end.pred_n(1).to_ymd(),
            date_end.succ_n(1).to_ymd()
        );

        assert_eq!(chrono_end.year(), date_start.year());
        assert_eq!(chrono_end.month() as u8, date_end.month());
        assert_eq!(chrono_end.day() as u8, date_end.day());

        for month in [
            MonthOfYear::Jan,
            MonthOfYear::Feb,
            MonthOfYear::Mar,
            MonthOfYear::Apr,
            MonthOfYear::May,
            MonthOfYear::Jun,
            MonthOfYear::Jul,
            MonthOfYear::Aug,
            MonthOfYear::Sep,
            MonthOfYear::Oct,
            MonthOfYear::Nov,
            MonthOfYear::Dec,
        ] {
            let chrono_start = NaiveDate::from_ymd_opt(year, month.number() as u32, 1).unwrap();
            let date_start = Date::first_on_month(year, month);
            dbg!(chrono_start, date_start.to_ymd());

            assert_eq!(chrono_start.year(), date_start.year());
            assert_eq!(chrono_start.month() as u8, date_start.month());
            assert_eq!(chrono_start.day() as u8, date_start.day());

            let chrono_end =
                NaiveDate::from_ymd_opt(year, month.number() as u32, month.num_days(year) as u32)
                    .unwrap();
            let date_end = Date::last_on_month(year, month);
            dbg!(chrono_end, date_end.to_ymd());

            assert_eq!(chrono_end.year(), date_start.year());
            assert_eq!(chrono_end.month() as u8, date_end.month());
            assert_eq!(chrono_end.day() as u8, date_end.day());

            assert_eq!(date_start, Date::ymd(year, month, DayOfMonth::D1));
            assert_eq!(
                date_end,
                Date::first_on_month(year, month).succ_n(month.num_days(year) as u16 - 1)
            );
        }
    }
}
