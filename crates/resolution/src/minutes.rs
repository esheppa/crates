use crate::*;

pub type Minute = Minutes<1>;
pub type FiveMinute = Minutes<5>;
pub type HalfHour = Minutes<30>;
pub type Hour = Minutes<60>;

const MIN: i64 = -4_000_000_000;
const MAX: i64 = 4_000_000_000; // TODO

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Minutes<const N: u16>(i64);
pub(crate) const MINUTES_PER_DAY: i64 = 24 * 60;

impl<const N: u16> Minutes<N> {
    const PERIODS_PER_DAY: i64 = MINUTES_PER_DAY / N as i64;
    const SENSIBLE: () = {
        let sensible = [
            1, 2, 3, 4, 5, 6, 10, 15, 20, 30, 60, 120, 180, 240, 360, 720,
        ];

        let mut idx = 0;

        loop {
            if idx >= sensible.len() {
                panic!(
                    "Please choose a minutes impl within 1, 2, 3, 4, 5, 6, 10, 15, 20, 30, 60, 120, 180, 240, 360, 720"
                )
            }

            if N == sensible[idx] {
                break;
            }

            idx += 1;
        }
    };
    // TODO: test?
    const fn change_resolution<const N2: u16>(self) -> Minutes<N2> {
        // ensures that both N and N2 are sensible...
        const {
            _ = Self::SENSIBLE;
            _ = Minutes::<N2>::SENSIBLE;
        }

        if N2 == N {
            // can't just return self, because compiler doesn't know that N2 == N ...
            Minutes(self.0)
        } else if N2 > N {
            // long day subdivions to short
            // clean scaling
            if N2 % N == 0 {
                Minutes {
                    0: self.0 / Self::PERIODS_PER_DAY * (MINUTES_PER_DAY / N2 as i64),
                }
            } else {
                panic!("Incompatible minutes when changing resolution")
            }
        } else {
            // short day subdivision to long
            if N % N2 == 0 {
                Minutes {
                    0: self.0 / (MINUTES_PER_DAY / N2 as i64) * Self::PERIODS_PER_DAY,
                }
            } else {
                panic!("Incompatible minutes when changing resolution")
            }
        }
    }

    // pub const fn occurs_on_day(self) -> Day {
    //     Day::new(self.index / Self::PERIODS_PER_DAY)
    // }
    pub const fn first_on_day(day: Day) -> Option<Self> {
        let Some(x) = day.to_monotonic().checked_mul(Self::PERIODS_PER_DAY) else {
            return None;
        };
        Self::from_monotonic(x)
    }
    pub const fn from_monotonic(idx: i64) -> Option<Self> {
        // TODO: use MIN..=MAX here when it is const
        if idx >= MIN && idx <= MAX {
            Some(Minutes(idx))
        } else {
            None
        }
    }
    pub const fn to_monotonic(self) -> i64 {
        self.0
    }

    pub const fn between(self, other: Self) -> i64 {
        other.0 - self.0
    }

    pub const fn translate(self, n: i64) -> Option<Self> {
        let Some(new) = self.0.checked_add(n) else {
            return None;
        };
        Some(Minutes(new))
    }

    pub const fn start_minute(self) -> Minute {
        Minutes::<1>(self.0 * (N as i64))
    }

    pub const fn end_minute(self) -> Minute {
        Minutes::<1>(self.0 * (N as i64) + (N as i64))
    }
    const NAME: &str = {
        match N {
            1 => "Minutes[Length:1]",
            2 => "Minutes[Length:2]",
            3 => "Minutes[Length:3]",
            4 => "Minutes[Length:4]",
            5 => "Minutes[Length:5]",
            6 => "Minutes[Length:6]",
            10 => "Minutes[Length:10]",
            15 => "Minutes[Length:15]",
            20 => "Minutes[Length:20]",
            30 => "Minutes[Length:30]",
            60 => "Minutes[Length:60]",
            120 => "Minutes[Length:120]",
            180 => "Minutes[Length:180]",
            240 => "Minutes[Length:240]",
            360 => "Minutes[Length:360]",
            720 => "Minutes[Length:720]",
            _ => panic!(
                "Please choose a minutes impl within 1, 2, 3, 4, 5, 6, 10, 15, 20, 30, 60, 120, 180, 240, 360, 720"
            ),
        }
    };
}

impl<const N: u16> TimeResolution for Minutes<N> {
    const NAME: &str = Self::NAME;
    fn translate(self, n: i64) -> Option<Self> {
        self.translate(n)
    }

    fn start_minute(self) -> Minute {
        self.start_minute()
    }

    fn end_minute(self) -> Minute {
        self.end_minute()
    }
}

impl<const N: u16> Monotonic for Minutes<N> {
    fn to_monotonic(self) -> i64 {
        self.to_monotonic()
    }

    fn between(self, other: Self) -> i64 {
        self.between(other)
    }
}

impl<const N: u16> FromMonotonic for Minutes<N> {
    fn from_monotonic(idx: i64) -> Option<Self> {
        Self::from_monotonic(idx)
    }
}

impl<const N: u16> SubDateResolution for Minutes<N> {
    type Params = ();

    fn params(self) -> Self::Params {
        ()
    }

    // TODO: test
    fn occurs_on_day(self) -> Day {
        Day::from_monotonic(self.0 / Self::PERIODS_PER_DAY).expect("")
    }

    fn from_minute(minute: Minute, _params: Self::Params) -> Self {
        minute.change_resolution()
    }

    fn first_on_day(day: Day, _params: Self::Params) -> Option<Self> {
        Self::first_on_day(day)
    }
}
