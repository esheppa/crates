extern crate std;
use crate::Year;

use super::{Date, DayOfMonth, MonthOfYear, YearAndDays, first_on_year_internal, is_leap_year};
use calendrical_calculations::{iso::iso_from_fixed, rata_die::RataDie};
use chrono::{self, Datelike, Days, NaiveDate};

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

    for i in 0..2_000_000_i32 {
        let chrono_adj = chrono_base + Days::new(i as u64);
        let date = Date(i);
        assert_eq!(chrono_adj.year(), date.year().num());
        assert_eq!(chrono_adj.month() as u8, date.month_of_year().number());
        assert_eq!(chrono_adj.day() as u8, date.day_of_month());

        {
            assert_eq!(chrono_adj, date.chrono_date());
            assert_eq!(date, Date::from_chrono_date(chrono_adj));
        }
    }

    for i in 0..2_000_000_i32 {
        let (year, month, day) = iso_from_fixed(RataDie::new(i as i64)).unwrap();
        let date = Date(i);
        assert_eq!(year, date.year().num());
        assert_eq!(month, date.month_of_year().number());
        assert_eq!(day, date.day_of_month());

    }
}

#[test]
fn test_roundtrip_neg() {
    for i in (i32::MIN + 44387)..0 {
        let date = Date::new(i);
        let calculated = YearAndDays::calculate(date);

        let roundtrip = first_on_year_internal(calculated.year)
            .unwrap()
            .translate(calculated.days_through)
            .unwrap();
        assert_eq!(date, roundtrip);
    }

    for i in i32::MIN..(i32::MIN + 44387) {
        let date = Date::new(i);
        let calculated = YearAndDays::calculate(date);
        assert_eq!(first_on_year_internal(calculated.year), None);
    }
}

#[test]
fn test_roundtrip_pos() {
    for i in 0..(i32::MAX - 44021) {
        let date = Date::new(i);
        let calculated = YearAndDays::calculate(date);
        let roundtrip = first_on_year_internal(calculated.year)
            .unwrap()
            .translate(calculated.days_through)
            .unwrap();
        assert_eq!(date, roundtrip);
    }

    for i in (i32::MAX - 44021 + 1)..=i32::MAX {
        let date = Date::new(i);
        let calculated = YearAndDays::calculate(date);
        assert_eq!(first_on_year_internal(calculated.year), None);
    }
}

#[test]
fn test_new() {
    for year in 1840..9_999 {
        let chrono_start = NaiveDate::from_ymd_opt(year, 1, 1).unwrap();
        let date_start = Date::first_on_year(Year::new(year).unwrap()).unwrap();
        // dbg!(
        //     chrono_start,
        //     date_start.to_ymd(),
        //     date_start,
        //     NaiveDate::from_num_days_from_ce_opt(date_start.inner())
        // );

        assert_eq!(chrono_start.year(), date_start.year().num());
        assert_eq!(
            chrono_start.month() as u8,
            date_start.month_of_year().number()
        );
        assert_eq!(chrono_start.day() as u8, date_start.day_of_month());

        let chrono_end = NaiveDate::from_ymd_opt(year, 12, 31).unwrap();
        let date_end = Date::last_on_year(Year::new(year).unwrap()).unwrap();

        // dbg!(
        //     chrono_end,
        //     date_end.to_ymd(),
        //     year,
        //     date_end,
        //     date_end.pred_n(1).to_ymd(),
        //     date_end.succ_n(1).to_ymd()
        // );

        assert_eq!(chrono_end.year(), date_start.year().num());
        assert_eq!(chrono_end.month() as u8, date_end.month_of_year().number());
        // dbg!(
        //     year,
        //     chrono_start.to_string(),
        //     chrono_end.to_string(),
        //     date_start.to_ymd(),
        //     date_end.to_ymd()
        // );
        assert_eq!(chrono_end.day() as u8, date_end.day_of_month());

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
            let date_start = Date::first_on_month(Year::new(year).unwrap(), month);
            // dbg!(chrono_start, date_start.unwrap().to_ymd());

            assert_eq!(chrono_start.year(), date_start.unwrap().year().num());
            assert_eq!(
                chrono_start.month() as u8,
                date_start.unwrap().month_of_year().number()
            );
            assert_eq!(chrono_start.day() as u8, date_start.unwrap().day_of_month());

            let chrono_end = NaiveDate::from_ymd_opt(
                year,
                month.number() as u32,
                month.num_days(Year::new(year).unwrap()) as u32,
            )
            .unwrap();
            let date_end = Date::last_on_month(Year::new(year).unwrap(), month);
            // dbg!(chrono_end, date_end.to_ymd());

            assert_eq!(chrono_end.year(), date_start.unwrap().year().num());
            assert_eq!(
                chrono_end.month() as u8,
                date_end.unwrap().month_of_year().number()
            );
            assert_eq!(chrono_end.day() as u8, date_end.unwrap().day_of_month());

            assert_eq!(
                date_start,
                Date::ymd(Year::new(year).unwrap(), month, DayOfMonth::D1)
            );
            assert_eq!(
                date_end,
                Date::first_on_month(Year::new(year).unwrap(), month)
                    .unwrap()
                    .translate(month.num_days(Year::new(year).unwrap()) as i32 - 1)
            );
        }
    }
}
