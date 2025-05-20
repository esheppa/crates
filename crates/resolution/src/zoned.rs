use crate::DateResolution;
use crate::DateResolutionExt;
use crate::Day;
use crate::LongerThan;
use crate::LongerThanOrEqual;
use crate::Minute;
use crate::Monotonic;
use crate::SubDateResolution;
use crate::TimeResolution;
use crate::time_of_day::LocalDateTime;
use crate::time_of_day::LocalTimeOfDay;
use alloc::format;
use alloc::string::String;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::NaiveDate;
use chrono::NaiveTime;
use chrono::Offset;
use chrono::TimeDelta;
use chrono::TimeZone;
use chrono::Utc;
use core::any;
use core::fmt;
use core::result;

pub trait FixedTimeZone: TimeZone + Copy + fmt::Debug {
    fn new() -> Self;
}

impl FixedTimeZone for Utc {
    fn new() -> Self {
        Utc
    }
}

/// `Zoned` stores a `TimeResolution` representing the UTC time in the zone, plus the zone itself.
/// This is intended to allow assertion that a given resolution is in a certain
/// timezone and thus allow finding the start and end times of that resolution with their correct UTC offsets.
///
/// warning: this should not be used for `SubDateResolution`s larger than `Minutes<60>` or equivalent. (Ideally
/// this restriction will be removed later)
///
/// note: this works perfectly well with _fixed_ and _non-fixed_ timezones, but many implementations are only
/// available for fixed timezones.
///
/// in the case of dividing a day into, for example, half hours, but the day has a DST transition
/// the effect would be that the backwards transition would have 50 periods while the forwards transition would have 46
/// importantly, on the backwards transition, some local resolutions would occur _twice_, while on the
/// forwards transition, some would not occur at all
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Zoned<R, Z>
where
    R: TimeResolution,
    Z: TimeZone + Copy + fmt::Debug,
{
    // where the R is a Minutes<N>:
    // where the R is not a Minutes<N>: no difference between utc and local anyway
    utc_resolution: R,
    zone: Z,
}

impl<R, Z> Copy for Zoned<R, Z>
where
    R: TimeResolution,
    Z: TimeZone + Copy + fmt::Debug,
{
}

impl<R, Z> Clone for Zoned<R, Z>
where
    R: TimeResolution,
    Z: TimeZone + Copy + fmt::Debug,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<R, Z> Eq for Zoned<R, Z>
where
    R: TimeResolution,
    Z: TimeZone + Copy + fmt::Debug,
{
}

impl<R, Z> PartialEq for Zoned<R, Z>
where
    R: TimeResolution,
    Z: TimeZone + Copy + fmt::Debug,
{
    fn eq(&self, other: &Self) -> bool {
        self.utc_resolution == other.utc_resolution
    }
}

impl<R, Z> Ord for Zoned<R, Z>
where
    R: TimeResolution,
    Z: TimeZone + Copy + fmt::Debug,
{
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.utc_resolution.cmp(&other.utc_resolution)
    }
}

impl<R, Z> PartialOrd for Zoned<R, Z>
where
    R: TimeResolution,
    Z: TimeZone + Copy + fmt::Debug,
{
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<R, Z> Monotonic for Zoned<R, Z>
where
    Z: TimeZone + Copy + fmt::Debug,
    R: TimeResolution,
{
    fn to_monotonic(self) -> i32 {
        self.utc_resolution.to_monotonic()
    }
    fn between(self, other: Self) -> i32 {
        other.to_monotonic() - self.to_monotonic()
    }
}

impl<R, Z> TimeResolution for Zoned<R, Z>
where
    R: TimeResolution,
    Z: TimeZone + Copy + fmt::Debug,
{
    fn succ_n(self, n: u16) -> Self {
        Zoned {
            utc_resolution: self.utc_resolution.succ_n(n),
            ..self
        }
    }
    fn pred_n(self, n: u16) -> Self {
        Zoned {
            utc_resolution: self.utc_resolution.pred_n(n),
            ..self
        }
    }

    const NAME: &str = const {
        // if any::type_name::<R>() == "Year" {

        // }

        // panic!("not allowed for ZonedLocal")

        "ZonedUtc"
    };

    fn start_minute(self) -> crate::Minute {
        self.local_resolution().start_minute()
    }

    fn day(self) -> crate::Day {
        self.local_resolution().day()
    }

    fn month(self) -> crate::Month {
        self.local_resolution().month()
    }

    fn year(self) -> crate::Year {
        self.local_resolution().year()
    }
}

impl<R, Z> Zoned<R, Z>
where
    R: TimeResolution,
    Z: TimeZone + Copy + fmt::Debug,
{
    pub fn name(self) -> String {
        format!("Zoned[{},{:?}]", R::NAME, self.zone)
    }
    pub fn local_end_exclusive(&self) -> chrono::DateTime<Z> {
        self.succ().local_start_datetime()
    }
    fn start_datetime(&self) -> DateTime<Utc> {
        self.utc_start_datetime()
    }
    pub fn local_start_datetime(&self) -> DateTime<Z> {
        self.utc_start_datetime().with_timezone(&self.zone)
    }

    pub fn utc_start_datetime(&self) -> DateTime<Utc> {
        self.utc_resolution
            .start_minute()
            .local_time()
            .chrono_datetime()
            .and_utc()
    }

    pub fn zone(&self) -> Z {
        self.zone
    }

    pub fn start_time(self) -> LocalDateTime {
        LocalDateTime::from_minutes(Minute::from_utc_datetime(
            self.local_start_datetime().naive_local().and_utc(),
        ))
    }
}

impl<R, Z> Zoned<R, Z>
where
    R: DateResolution<Params = ()>,
    Z: TimeZone + Copy + fmt::Debug,
{
    pub fn start_day(&self) -> Day {
        self.local_resolution().start_day()
    }
    pub fn end_day(&self) -> Day {
        self.local_resolution().end_day()
    }
    pub fn from_day(date: Day, zone: Z) -> Self {
        todo!()
        // Zoned {
        //     local_resolution: R::from_day(date, ()),
        //     // for DateResolution this is the offset of the start time
        //     current_offset: local_offset_at_start_of_date(date, zone),
        //     zone,
        // }
    }
}

// the ZonedUtc itself can't implement SubDateResolution
// but it can happily wrap R's that do...
impl<R, Z> Zoned<R, Z>
where
    R: SubDateResolution<Params = ()>,
    Z: TimeZone + Copy + fmt::Debug,
{
    pub fn local_resolution(&self) -> R {
        // get utc start time
        // convert to local time
        // convert to Minute
        // convert to local res
        R::from_minute(
            Minute::from_utc_datetime(self.local_start_datetime().naive_local().and_utc()),
            (),
        )
    }

    // pub fn first_on_day
    // pub fn last_on_day
}

impl<R, Z> Zoned<R, Z>
where
    R: SubDateResolution<Params = ()>,
    // this is critical
    // as otherwise from_local would need to be fallible
    Z: FixedTimeZone,
{
    // if you want to create for a non FixedTimeZone
    // then you'll need to start with an known valid local date time and offset
    pub fn from_local(value: R, zone: Z) -> Self {
        value
            .start_datetime()
            .naive_utc()
            .and_local_timezone(zone)
            .single()
            // unwrap will never panic becuase calling
            // `and_local_timezone` with a FixedTimeZone will
            // always reuturn a valid local time
            .unwrap()
            .into()
    }
}

impl<R, Z> fmt::Debug for Zoned<R, Z>
where
    R: TimeResolution + fmt::Debug,
    Z: TimeZone + fmt::Debug + Copy,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Zoned")
            .field("start_time_local", &self.local_start_datetime())
            .field("start_time_utc", &self.utc_start_datetime())
            .field("utc_resolution", &self.utc_resolution)
            .field("zone", &self.zone)
            .finish()
    }
}

// impl<R1, R2, Z> LongerThan<ZonedUtc<R2, Z>> for ZonedUtc<R1, Z>
// where
//     R1: TimeResolution,
//     R2: TimeResolution,
//     Z: TimeZone + Copy + fmt::Debug,
//     R1: LongerThan<R2>,
// {
// }

// impl<R1, R2, Z> LongerThanOrEqual<ZonedUtc<R2, Z>> for ZonedUtc<R1, Z>
// where
//     R1: TimeResolution,
//     R2: TimeResolution,
//     Z: TimeZone + Copy + fmt::Debug,
//     R1: LongerThanOrEqual<R2>,
// {
// }

impl<R, Z> SubDateResolution for Zoned<R, Z>
where
    R: SubDateResolution<Params = ()>,
    // this is critical
    // as otherwise from_local would need to be fallible
    Z: FixedTimeZone,
{
    type Params = Z;

    fn params(self) -> Self::Params {
        self.zone()
    }
    fn occurs_on_day(self) -> Day {
        self.local_start_datetime().date_naive()
    }

    fn first_on_day(day: Day, params: Self::Params) -> Self {
        // find the start time of the day in UTC!
        // unwrap: should be ok, becuase empirically no recent TZ offset transitions at midnight
        // however, these could theoretically happen.
        let start_time_of_day = day
            .and_time(NaiveTime::MIN)
            .and_local_timezone(params)
            .single()
            .unwrap()
            .to_utc();
        Self::from_utc_datetime(start_time_of_day, params)
    }

    fn from_minute(minute: crate::Minute, params: Self::Params) -> Self {
        todo!()
    }
}

impl<R, Z> DateResolution for Zoned<R, Z>
where
    R: DateResolution<Params = ()>,
    Z: TimeZone + fmt::Debug + Copy,
{
    type Params = Z;
    fn params(&self) -> Self::Params {
        self.zone()
    }
    fn start_day(&self) -> chrono::NaiveDate {
        self.local_resolution.start_day()
    }

    fn from_day(date: NaiveDate, params: Self::Params) -> Self {
        Zoned::from_date(date, params)
    }
}

fn local_offset_at_start_of_date<Z>(date: NaiveDate, tz: Z) -> FixedOffset
where
    Z: TimeZone + Copy,
{
    (0..=240) // balance of prevent DoS and finding a valid local timestamp.
        .filter_map(|minutes_offset| {
            let local_start =
                date.and_time(NaiveTime::MIN) + TimeDelta::try_minutes(minutes_offset)?;
            let with_tz = local_start.and_local_timezone(tz).single()?;
            Some(with_tz.offset().fix())
        })
        .next()
        // possible to panic, but _extremely_ unlikely
        .unwrap()
}

impl<Z, R> From<chrono::DateTime<Z>> for Zoned<R, Z>
where
    R: SubDateResolution<Params = ()>,
    Z: TimeZone + Copy + fmt::Debug,
{
    fn from(local_time: chrono::DateTime<Z>) -> Self {
        todo!()
        // ZonedUtc {
        //     // we swap out the tz for UTC without changing the actual hour/minute here
        //     // a bit sketchy but does the intended effect of producitng a local resolution
        //     local_resolution: R::from_utc_datetime(local_time.naive_local().and_utc(), ()),
        //     current_offset: local_time.offset().fix(),
        //     zone: local_time.timezone(),
        // }
    }
}

#[cfg(test)]
mod tests {
    use crate::DateResolution;
    use crate::Day;
    use crate::FixedTimeZone;
    use crate::Minutes;
    use crate::Zoned;
    use alloc::vec::Vec;
    use chrono::FixedOffset;

    #[test]
    fn test_subdate() {
        fn subdate<const N: u32>(tz: chrono_tz::Tz) {
            let start = chrono::NaiveDate::from_ymd_opt(2022, 1, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap()
                .and_local_timezone(tz)
                .unwrap();

            let start_timestamps = (0..((24 * 60 / N) * 365))
                .map(|i| start + chrono::Duration::minutes((i * N).into()));

            for start_timestamp in start_timestamps {
                assert_eq!(
                    start_timestamp,
                    Zoned::<Minutes<N>, _>::from(start_timestamp).local_start_datetime(),
                );
            }
        }

        for tz in [
            chrono_tz::Australia::Sydney,
            chrono_tz::Australia::Adelaide,
            chrono_tz::Asia::Kathmandu,
        ] {
            subdate::<1>(tz);
            subdate::<2>(tz);
            subdate::<5>(tz);
            subdate::<6>(tz);
            subdate::<10>(tz);
            subdate::<15>(tz);
            subdate::<30>(tz);
            subdate::<60>(tz);

            // // this is ... problematic ... with daylight savings
            // // zoned may not be possible for times larger than an hour and less than a day
            // subdate::<120>(tz);
            // subdate::<180>(tz);
            // subdate::<240>(tz);
        }

        fn subdate_fixed<const N: u32, Z: FixedTimeZone>(tz: Z) {
            let start = chrono::NaiveDate::from_ymd_opt(2022, 1, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap()
                .and_local_timezone(tz)
                .unwrap();

            let start_timestamps = (0..((24 * 60 / N) * 365))
                .map(|i| start.clone() + chrono::Duration::minutes((i * N).into()));

            for start_timestamp in start_timestamps {
                assert_eq!(
                    start_timestamp.clone(),
                    Zoned::<Minutes<N>, _>::from(start_timestamp.clone()).local_start_datetime(),
                );

                assert_eq!(
                    Zoned::from_local(
                        Zoned::<Minutes<N>, _>::from(start_timestamp.clone()).local_resolution(),
                        tz
                    ),
                    Zoned::<Minutes<N>, _>::from(start_timestamp.clone())
                );

                #[cfg(feature = "serde")]
                assert_eq!(
                    serde_json::from_str::<Zoned::<Minutes<N>, _>>(
                        &serde_json::to_string(&Zoned::<Minutes<N>, _>::from(
                            start_timestamp.clone()
                        ))
                        .unwrap()
                    )
                    .unwrap(),
                    Zoned::<Minutes<N>, _>::from(start_timestamp)
                );
            }
        }

        #[derive(Debug, Clone, Copy)]
        struct FixedEast<const N: i32>;

        impl<const N: i32> chrono::TimeZone for FixedEast<N> {
            type Offset = FixedOffset;

            fn from_offset(_: &Self::Offset) -> Self {
                Self
            }

            fn offset_from_local_date(
                &self,
                _: &chrono::prelude::NaiveDate,
            ) -> chrono::MappedLocalTime<Self::Offset> {
                unimplemented!()
            }

            fn offset_from_local_datetime(
                &self,
                _: &chrono::prelude::NaiveDateTime,
            ) -> chrono::MappedLocalTime<Self::Offset> {
                chrono::MappedLocalTime::Single(chrono::FixedOffset::east_opt(N).unwrap())
            }

            fn offset_from_utc_date(&self, _: &chrono::prelude::NaiveDate) -> Self::Offset {
                unimplemented!()
            }

            fn offset_from_utc_datetime(&self, _: &chrono::prelude::NaiveDateTime) -> Self::Offset {
                chrono::FixedOffset::east_opt(N).unwrap()
            }
        }

        impl<const N: i32> FixedTimeZone for FixedEast<N> {
            fn new() -> Self {
                FixedEast
            }
        }

        fn test_for_zone<F: FixedTimeZone>() {
            subdate_fixed::<1, _>(F::new());
            subdate_fixed::<2, _>(F::new());
            subdate_fixed::<5, _>(F::new());
            subdate_fixed::<6, _>(F::new());
            subdate_fixed::<10, _>(F::new());
            subdate_fixed::<15, _>(F::new());
            subdate_fixed::<30, _>(F::new());
            subdate_fixed::<60, _>(F::new());
            subdate_fixed::<120, _>(F::new());
            subdate_fixed::<180, _>(F::new());
            subdate_fixed::<240, _>(F::new());
        }

        test_for_zone::<chrono::Utc>();
        test_for_zone::<FixedEast<{ 60 * 60 * 3 }>>();
        test_for_zone::<FixedEast<{ 60 * 60 * -4 }>>();
    }

    #[test]
    fn test_date() {
        fn date<R: DateResolution<Params = ()>>(tz: chrono_tz::Tz) {
            let start = chrono::NaiveDate::from_ymd_opt(2022, 1, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap()
                .and_local_timezone(tz)
                .unwrap();

            let periods = (0..365)
                .map(|i| start + chrono::Days::new(i))
                .collect::<Vec<_>>();

            for period in periods {
                let zoned = Zoned::<R, _>::from_date(period.date_naive(), tz);
                assert_eq!(period.date_naive(), zoned.start_day());
                assert_eq!(period.date_naive(), zoned.local_resolution().start_day());

                let zoned2 = Zoned::<R, _>::from_date(period.date_naive(), tz);
                assert_eq!(period.date_naive(), zoned2.start_day());
                assert_eq!(period.date_naive(), zoned2.local_resolution().start_day());
            }
        }
        for tz in [
            chrono_tz::Australia::Sydney,
            chrono_tz::Australia::Adelaide,
            chrono_tz::Asia::Kathmandu,
        ] {
            date::<Day>(tz);
        }
    }
}
