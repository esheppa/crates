pub mod providers;
pub mod reports;
pub mod units;

use std::{collections, error, fmt, marker};

pub type DynErr = Box<dyn error::Error + Send + Sync + 'static>; // add constraints similar to anyhow::Error here

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    message: String,
    cause: Box<dyn error::Error + Send + Sync>, // add constraints similar to anyhow::Error here
    error_kind: ErrorKind,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Error {}: {} ", self.error_kind, self.message)?;
        write!(f, "  Cause by: {} ", self.cause)
    }
}

impl error::Error for Error {}

#[derive(Debug)]
pub enum ErrorKind {
    Custom(String),
    Database,
    Simulation,
    NotSupported,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorKind::Database => write!(f, "Database"),
            ErrorKind::Simulation => write!(f, "Simulation"),
            ErrorKind::NotSupported => write!(f, "NotSupported"),
            ErrorKind::Custom(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Position {
    Long,
    Short,
}

// should this instead be a trait?
// no quantity here, some contracts will have dynamic quantity
pub struct OtcBase<P>
where
    P: resolution::TimeResolution, // this will often be filled in for specific implementations
{
    pub delivery: resolution::TimeRange<P>, // some contigious set of durations
    pub position: Position,
    pub entity: String,
    pub counterparty: String,
    pub effective: resolution::Day,
    pub settlement: resolution::Day,
    pub tags: collections::HashMap<String, String>,
}

// impl<P> OtcBase<P>
// where
//     P: resolution::DateResolution, // this will often be filled in for specific implementations
// {
//     pub fn settlement_price_request(
//         &self,
//     ) -> providers::PriceRequest<P, P> {
//         PriceRequest {
//             series: range
//                 .iter()
//                 .map(|x| PriceMetadata {
//                     effective: x,
//                     delivery: x,
//                 })
//                 .collect(),
//         }
//     }
// }

// should this instead be a trait?
pub struct FuturesBase<P>
where
    P: resolution::TimeResolution, // this will often be filled in for specific implementations
{
    pub delivery: P, // futures will always need a regular duration. use OTC if irregular
    pub position: Position,
    pub entity: String,
    pub exchange: String,
    pub effective: chrono::NaiveDate,
    pub settlement: chrono::NaiveDate,
    pub tags: collections::HashMap<String, String>,
}
