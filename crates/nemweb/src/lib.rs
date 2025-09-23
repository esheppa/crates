#![no_std]
extern crate alloc;

use core::{
    fmt::{Debug, Display},
    str::FromStr,
};

use alloc::{
    borrow::Cow,
    format,
    string::{String, ToString},
    vec::Vec,
};
use chrono::{DateTime, Month, NaiveDate, Utc};
use url::Url;

mod folders;
pub use folders::*;

const NEMWEB: &str = "https://nemweb.com.au";
const ARCHIVE: &str = "Reports/Archive";
const CURRENT: &str = "Reports/Current";
const DVD: &str = "Data_Archive/Wholesale_Electricity/MMSDM";

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ParquetName {
    Daily {
        date: date::Date,
        table: ParquetTable,
        is_archive: bool,
    },
    Monthly {
        month: resolution::Month,
        table: ParquetTable,
    },
}

impl ParquetName {
    pub fn table(&self) -> ParquetTable {
        match self {
            ParquetName::Daily { table, .. } => *table,
            ParquetName::Monthly { table, .. } => *table,
        }
    }

    pub fn month(&self) -> resolution::Month {
        match self {
            ParquetName::Daily { date, .. } => resolution::Month::new(
                resolution::Year::from_monotonic(date.year().num().into()).unwrap(),
                date.month_of_year(),
            ),
            ParquetName::Monthly { month, .. } => *month,
        }
    }

    pub fn path(&self) -> String {
        format!(
            "{}/yearmonth={}{:02}/{}",
            self.table().dvd_table_name(),
            self.month().year().to_monotonic(),
            self.month().month_of_year().number(),
            self
        )
    }
}

impl Display for ParquetName {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ParquetName::Daily {
                is_archive: false,
                date,
                table,
            } => write!(
                f,
                "{}_current_{}.parquet",
                table.dvd_table_name(),
                date.chrono_date().format("%Y-%m-%d")
            ),
            ParquetName::Daily {
                is_archive: true,
                date,
                table,
            } => write!(
                f,
                "{}_archive_{}.parquet",
                table.dvd_table_name(),
                date.chrono_date().format("%Y-%m-%d")
            ),
            ParquetName::Monthly { month, table } => write!(
                f,
                "{}_{}_{:02}.parquet",
                table.dvd_table_name(),
                month.year().to_monotonic(),
                month.month_of_year().number(),
            ),
        }
    }
}

// add to this only the minimal number of tables that we are interested in
// for now this is intended for historical data only (eg, previous day at minimum, no forecasts/pasa)
// in future, if a live data / forecasts / pasa system is required, potentially this could be divided
// into two sections: Live and Historical.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ParquetTable {
    DispatchPrice,
    TradingPrice,
    DispatchRegionsum,
    RooftopActual,
    // MeterdataGenduid, // no DVD files???
    DispatchUnitsolution,
}

// Rules:

// one file per day, or one file per month. _ONLY_
//

// DispatchPrice + TradingPrice + DispatchRegionSum + RooftopActual
// 1. Process the relevant files once a day at ~ 6am for previous day (one pq per day)
// 2. override with the archive when available (+2 days for dispatch, +7 days for trading, +14 days for rooftop) (on pq per day)
// 3. override with monthly when available (one pq per month)
// 4. periodically check tradingprice/dispatchprice against aemo agg price+demand files

// DispUnitSolution
// 1. Process the relevant files once a day at ~ 6am for previous day (one pq per day)
// 2. override with monthly when available (one pq per month)

impl ParquetTable {
    pub fn current_folder(&self) -> impl Iterator<Item = CurrentFolder> {
        match self {
            ParquetTable::DispatchPrice => [
                CurrentFolder::DispatchISReports,
                CurrentFolder::AdjustedPricesReports,
            ]
            .as_slice()
            .iter()
            .cloned(),
            ParquetTable::DispatchRegionsum => [CurrentFolder::DispatchISReports]
                .as_slice()
                .iter()
                .cloned(),
            ParquetTable::RooftopActual => [].as_slice().iter().cloned(),
            // ParquetTable::MeterdataGenduid => Some(CurrentFolder::NextDayActualGen),
            ParquetTable::DispatchUnitsolution => {
                [CurrentFolder::NextDayDispatch].as_slice().iter().cloned()
            }

            // 5 minutely but have to load once a day
            ParquetTable::TradingPrice => [
                CurrentFolder::TradingISReports,
                CurrentFolder::AdjustedPricesReports,
            ]
            .as_slice()
            .iter()
            .cloned(),
        }
        .into_iter()
    }
    pub fn archive_folder(&self) -> impl Iterator<Item = ArchiveFolder> {
        match self {
            ParquetTable::DispatchPrice => [
                ArchiveFolder::DispatchISReports,
                ArchiveFolder::AdjustedPricesReports,
            ]
            .as_slice()
            .iter()
            .cloned(),
            ParquetTable::DispatchRegionsum => [ArchiveFolder::DispatchISReports]
                .as_slice()
                .iter()
                .cloned(),
            ParquetTable::RooftopActual => {
                [ArchiveFolder::ROOFTOPPVActual].as_slice().iter().cloned()
            }
            ParquetTable::TradingPrice => {
                [ArchiveFolder::TradingISReports].as_slice().iter().cloned()
            }
            ParquetTable::DispatchUnitsolution => [].as_slice().iter().cloned(),
        }
    }
    pub fn dvd_table_name(&self) -> &'static str {
        match self {
            ParquetTable::DispatchPrice => "DISPATCHPRICE",
            ParquetTable::TradingPrice => "TRADINGPRICE",
            ParquetTable::DispatchRegionsum => "DISPATCHREGIONSUM",
            ParquetTable::RooftopActual => "ROOFTOP_PV_ACTUAL",
            ParquetTable::DispatchUnitsolution => "DISPATCHLOAD",
        }
    }

    // list all the relevant archive files for the date range
    // considering the current time (which means some desired files just won't exist yet)
    pub fn archive_file_names(
        &self,
        _effective: NaiveDate,
        _now: DateTime<Utc>,
    ) -> impl Iterator<Item = ArchiveFilePath> {
        [].as_slice().iter().cloned()
    }
    pub fn dvd_path(&self, month: resolution::Month) -> DvdFilePath {
        DvdFilePath {
            year: month.year().to_monotonic().try_into().unwrap(),
            month: month.month_of_year().chrono_month(),
            name: *self,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum NemwebZipName {
    Standard(String),
    Dvd {
        year: i32,
        month: Month,
        table: ParquetTable,
    },
}

impl NemwebZipName {
    #[tracing::instrument]
    pub fn matches_date(&self, effective: NaiveDate) -> bool {
        match self {
            NemwebZipName::Standard(n) => {
                let Some(datestr) = n.rsplitn(3, '_').nth(1) else {
                    return false;
                };

                if datestr.len() < 8 {
                    return false;
                }

                let Ok(date) = NaiveDate::parse_from_str(&datestr[0..8], "%Y%m%d") else {
                    return false;
                };

                date == effective
            }
            NemwebZipName::Dvd { .. } => true,
        }
    }
}

impl Display for NemwebZipName {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            NemwebZipName::Standard(s) => write!(f, "{s}.zip"),
            NemwebZipName::Dvd { year, month, table } => write!(
                f,
                "PUBLIC_DVD_{dvd_table}_{year}{month_num:02}010000.zip",
                dvd_table = table.dvd_table_name(),
                month_num = month.number_from_month()
            ),
        }
    }
}

impl FromStr for NemwebZipName {
    type Err = String;

    #[tracing::instrument]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.is_ascii() {
            return Err(format!(
                "Invalid file name, expected all ascii characters but got `{s}`"
            ));
        }

        if s.contains("DVD") {
            return Err(format!(
                "Unable to parse DVD name `{s}`, these should be generated directly"
            ));
        }

        match s.split_once(".zip") {
            Some(vals) => Ok(NemwebZipName::Standard(vals.0.to_string())),
            None => Err(format!(
                "Invalid file extension, expected `.zip` but got `{s}`"
            )),
        }
    }
}

pub trait NemwebFilePath: Display + Debug {
    fn file_name<'a>(&'a self) -> Cow<'a, NemwebZipName>;

    fn to_url(&self) -> Url {
        Url::parse(&self.to_string()).expect("Should always be a valid URL")
    }

    fn file_level(&self) -> FileLevel;
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CurrentFilePath {
    pub folder: CurrentFolder,
    pub name: NemwebZipName,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ArchiveFilePath {
    folder: ArchiveFolder,
    name: NemwebZipName,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct DvdFilePath {
    year: i32,
    month: Month,
    name: ParquetTable,
}

impl NemwebFilePath for CurrentFilePath {
    fn file_name<'a>(&'a self) -> Cow<'a, NemwebZipName> {
        Cow::Borrowed(&self.name)
    }
    fn file_level(&self) -> FileLevel {
        FileLevel::Current
    }
}

impl NemwebFilePath for ArchiveFilePath {
    fn file_name<'a>(&'a self) -> Cow<'a, NemwebZipName> {
        Cow::Borrowed(&self.name)
    }
    fn file_level(&self) -> FileLevel {
        FileLevel::Archive
    }
}

impl NemwebFilePath for DvdFilePath {
    fn file_name<'a>(&'a self) -> Cow<'a, NemwebZipName> {
        Cow::Owned(NemwebZipName::Dvd {
            table: self.name,
            month: self.month,
            year: self.year,
        })
    }
    fn file_level(&self) -> FileLevel {
        FileLevel::Dvd
    }
}

impl Display for CurrentFilePath {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(NEMWEB)?;
        f.write_str("/")?;

        let CurrentFilePath { folder, name } = self;
        f.write_str(CURRENT)?;
        f.write_str("/")?;
        f.write_str(folder.folder_name())?;
        f.write_str("/")?;
        write!(f, "{name}")?;

        Ok(())
    }
}

impl Display for ArchiveFilePath {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(NEMWEB)?;
        f.write_str("/")?;

        let ArchiveFilePath { folder, name } = self;
        f.write_str(ARCHIVE)?;
        f.write_str("/")?;
        f.write_str(folder.folder_name())?;
        f.write_str("/")?;
        write!(f, "{name}")?;

        Ok(())
    }
}

impl Display for DvdFilePath {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(NEMWEB)?;
        f.write_str("/")?;

        let DvdFilePath { year, month, name } = self;
        let month_num = month.number_from_month();
        let zip_name = NemwebZipName::Dvd {
            year: *year,
            month: *month,
            table: *name,
        };
        write!(f, "{DVD}/{year}/MMSDM_{year}_{month_num:02}/MMSDM_Historical_Data_SQLLoader/DATA/{zip_name}")?;

        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum FileLevel {
    Current = 0,
    Archive = 1,
    Dvd = 2,
}

impl FileLevel {
    #[tracing::instrument]
    pub fn names(
        &self,
        table: ParquetTable,
        date: date::Date,
    ) -> impl Iterator<Item = ParquetName> {
        let mut items = Vec::with_capacity(3);

        if matches!(self, FileLevel::Current) {
            items.push(ParquetName::Daily {
                date,
                table,
                is_archive: false,
            });
        }

        if matches!(self, FileLevel::Current | FileLevel::Archive) {
            items.push(ParquetName::Daily {
                date,
                table,
                is_archive: false,
            });
        }

        if matches!(
            self,
            FileLevel::Current | FileLevel::Archive | FileLevel::Dvd
        ) {
            items.push(ParquetName::Monthly {
                month: resolution::Month::new(
                    resolution::Year::from_monotonic(date.year().num().into()).unwrap(),
                    date.month_of_year(),
                ),
                table,
            });
        }

        items.into_iter()

        // match self {
        //     // check if current, archive or dvd done
        //     FileLevel::Current => todo!(),
        //     // check if archive or dvd done
        //     FileLevel::Archive => todo!(),
        //     // check if dvd done
        //     FileLevel::Dvd => todo!(),
        // }
    }
}
