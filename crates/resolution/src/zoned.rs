use crate::DateResolution;
use crate::DateResolutionExt;
use crate::LongerThan;
use crate::LongerThanOrEqual;
use crate::Monotonic;
use crate::SubDateResolution;
use crate::TimeResolution;
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

/// `Zoned` stores a `TimeResolution` representing the local time in the zone, plus the relevant
/// offset and zone itself. This is intended to allow assertion that a given resolution is in a certain
/// timezone and thus allow finding the start and end times of that resolution with their correct UTC offsets.
///
/// warning: this should not be used for `SubDateResolution`s larger than `Minutes<60>` or equivalent. (Ideally
/// this restriction will be removed later)
///
/// note: this works perfectly well with _fixed_ and _non-fixed_ timezones, but many implementations are only
/// available for fixed timezones.
pub struct Zoned<R, Z>
where
    R: TimeResolution,
    Z: TimeZone + Copy + fmt::Debug,
{
    // we store local rather than utc here.
    // this is because we want start time validation (relevant for Minutes<N>) to be applied to the
    // local time, not the UTC time.
    // we could alternatively just store a valid DateTime<Utc> here that matches the UTC start time
    // of the local resolution
    local_resolution: R,
    // store the offset of the local_resolution so that we can reconstruct the local time infallibly
    current_offset: FixedOffset,
    zone: Z,
}

#[cfg(feature = "serde")]
impl<'de, R, Z> serde::de::Deserialize<'de> for Zoned<R, Z>
where
    R: SubDateResolution<Params = ()>,
    Z: FixedTimeZone,
{
    fn deserialize<D>(deserializer: D) -> result::Result<Zoned<R, Z>, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let local = chrono::NaiveDateTime::deserialize(deserializer)?;

        // unwrap here is fine because by the rules of `FixedTimeZone`
        // this operation must not fail
        let zoned = local.and_local_timezone(Z::new()).unwrap();

        Ok(zoned.into())
    }
}

#[cfg(feature = "serde")]
impl<'de, R, Z> serde::Serialize for Zoned<R, Z>
where
    R: SubDateResolution<Params = ()>,
    Z: FixedTimeZone,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.local_start_datetime()
            .naive_local()
            .serialize(serializer)
    }
}

impl<R, Z> TimeResolution for Zoned<R, Z>
where
    R: TimeResolution,
    Z: FixedTimeZone,
{
    fn succ_n(&self, n: u64) -> Self {
        Zoned {
            local_resolution: self.local_resolution.succ_n(n),
            ..*self
        }
    }
    fn pred_n(&self, n: u64) -> Self {
        Zoned {
            local_resolution: self.local_resolution.pred_n(n),
            ..*self
        }
    }
    fn start_datetime(&self) -> DateTime<Utc> {
        self.utc_start_datetime()
    }
    fn name(&self) -> String {
        format!("Zoned[{},{:?}]", self.local_resolution.name(), self.zone)
    }
}

impl<R, Z> Zoned<R, Z>
where
    R: TimeResolution,
    Z: FixedTimeZone,
{
    pub fn local_end_exclusive(&self) -> chrono::DateTime<Z> {
        self.succ().local_start_datetime()
    }
}

impl<R, Z> Zoned<R, Z>
where
    R: SubDateResolution<Params = ()>,
    Z: FixedTimeZone,
{
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

impl<R, Z> Zoned<R, Z>
where
    R: TimeResolution,
    Z: TimeZone + Copy + fmt::Debug,
{
    pub fn local_start_datetime(&self) -> DateTime<Z> {
        self.local_resolution
            .start_datetime()
            .naive_utc()
            .and_local_timezone(self.current_offset)
            .single()
            // unwrap will never panic becuase calling
            // `and_local_timezone` with a fixed offset will
            // always reuturn a valid local time
            .unwrap()
            .with_timezone(&self.zone)
    }

    pub fn utc_start_datetime(&self) -> DateTime<Utc> {
        self.local_start_datetime().to_utc()
    }

    pub fn zone(&self) -> Z {
        self.zone
    }
    pub fn local_resolution(&self) -> R {
        self.local_resolution
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
            .field("local_resolution", &self.local_resolution)
            .field("zone", &self.zone)
            .finish()
    }
}

impl<R, Z> Monotonic for Zoned<R, Z>
where
    Z: TimeZone + Copy + fmt::Debug,
    R: TimeResolution,
{
    fn to_monotonic(&self) -> i64 {
        self.local_resolution.to_monotonic()
    }
    fn between(&self, other: Self) -> i64 {
        other.to_monotonic() - self.to_monotonic()
    }
}

// impl<R1, R2, Z> LongerThan<Zoned<R2, Z>> for Zoned<R1, Z>
// where
//     R1: TimeResolution,
//     R2: TimeResolution,
//     Z: TimeZone + Copy + fmt::Debug,
//     R1: LongerThan<R2>,
// {
// }

// impl<R1, R2, Z> LongerThanOrEqual<Zoned<R2, Z>> for Zoned<R1, Z>
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
    Z: FixedTimeZone,
{
    type Params = Z;
    fn params(&self) -> Self::Params {
        self.zone()
    }
    fn occurs_on_date(&self) -> chrono::NaiveDate {
        self.local_start_datetime().date_naive()
    }

    fn first_on_day(day: chrono::NaiveDate, params: Self::Params) -> Self {
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

    fn from_utc_datetime(datetime: DateTime<Utc>, params: Self::Params) -> Self {
        datetime.with_timezone(&params).into()
    }
}

impl<R, Z> DateResolution for Zoned<R, Z>
where
    R: DateResolution<Params = ()>,
    Z: FixedTimeZone,
{
    type Params = Z;
    fn params(&self) -> Self::Params {
        self.zone()
    }
    fn start(&self) -> chrono::NaiveDate {
        self.local_resolution.start()
    }

    fn from_date(date: NaiveDate, params: Self::Params) -> Self {
        Zoned::from_date(date, params)
    }
}

impl<R, Z> Zoned<R, Z>
where
    R: DateResolution<Params = ()>,
    Z: TimeZone + Copy + fmt::Debug,
{
    pub fn start(&self) -> NaiveDate {
        self.local_resolution.start()
    }
    pub fn end(&self) -> NaiveDate {
        self.local_resolution.end()
    }
    pub fn from_date(date: NaiveDate, zone: Z) -> Self {
        Zoned {
            local_resolution: R::from_date(date, ()),
            // for DateResolution this is the offset of the start time
            current_offset: local_offset_at_start_of_date(date, zone),
            zone,
        }
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
        Zoned {
            // we swap out the tz for UTC without changing the actual hour/minute here
            // a bit sketchy but does the intended effect of producitng a local resolution
            local_resolution: R::from_utc_datetime(local_time.naive_local().and_utc(), ()),
            current_offset: local_time.offset().fix(),
            zone: local_time.timezone(),
        }
    }
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
        self.local_start_datetime() == other.local_start_datetime()
    }
}

impl<R, Z> Ord for Zoned<R, Z>
where
    R: TimeResolution,
    Z: TimeZone + Copy + fmt::Debug,
{
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.local_start_datetime()
            .cmp(&other.local_start_datetime())
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

impl<R, Z> Zoned<R, Z>
where
    R: DateResolution,
    Z: TimeZone + Copy + fmt::Debug,
{
}

impl<R, Z> Zoned<R, Z>
where
    R: SubDateResolution,
    Z: TimeZone + Copy + fmt::Debug,
{
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
                assert_eq!(period.date_naive(), zoned.start());
                assert_eq!(period.date_naive(), zoned.local_resolution().start());

                let zoned2 = Zoned::<R, _>::from_date(period.date_naive(), tz);
                assert_eq!(period.date_naive(), zoned2.start());
                assert_eq!(period.date_naive(), zoned2.local_resolution().start());
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
