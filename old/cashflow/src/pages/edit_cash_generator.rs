use crate::{accounting, input};
use resolution;
use std::{collections, default, result, num};
use seed::{*, prelude::*, virtual_dom::node, app::orders};
use serde::{Deserialize, Serialize};   

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Ulid: {0}")]
    Ulid(#[from] ulid::MonotonicError),
    #[error("Chrono: {0}")]
    Chrono(#[from] chrono::ParseError),
    #[error("Decimal: {0}")]
    Decimal(#[from] rust_decimal::Error),
    #[error("ParseIntError: {0}")]
    Int(#[from] num::ParseIntError),
    #[error("Accounting parse error: {0}")]
    Accounting(#[from] accounting::ParseError),
    #[error("Web storage error: {0:?}")]
    WebStorage(web_storage::WebStorageError),
}

type Result<T> = result::Result<T, Error>;

const STORAGE_KEY: &str = "cash-transfers";

pub struct Model {
    generators: CashTransfers,
    current_id: Option<ulid::Ulid>,
    input: CashGeneratorInput,
    recent_error: Result<()>,
}

impl default::Default for Model {
    fn default() -> Model {
        Model {
            current_id: None,
            recent_error: Ok(()),
            ..Default::default()
        }
    }
}

impl Model {
    pub fn view(&self) -> node::Node<Msg> {
        p!["Edit transactions"]
    }
    pub fn init(_orders: &mut impl orders::Orders<Msg>) -> Model {
        if let Ok(generators) = web_storage::LocalStorage::get(STORAGE_KEY) {
            Model {
                generators,
                ..Model::default()
            }
        } else {
            Model::default()
        }
    }
    pub fn title(&self) -> String {
        "Edit Cash Transfer".to_string()
    }
    pub fn update(&mut self, msg: Msg,  orders: &mut impl orders::Orders<Msg>) {
        match msg {
            Msg::AddGenerator => {

            }
            Msg::RemoveGenerator(id) => {

            }
            Msg::ClearAll => {

            }
            Msg::EditGenerator(id) => {

            }
            Input(msg) => {
                model.input.update(msg)
            }
        }
    }
}

pub enum Msg {
    AddGenerator,
    RemoveGenerator(ulid::Ulid),
    ClearAll,
    EditGenerator(ulid::Ulid),
    Input(InputMsg),
}

pub enum InputMsg {
    Amount(String),
    CrKind(String),
    CrName(String),
    DrKind(String),
    DrName(String),
    Gst(String),
    Narration(String),
    Resolution(String),
    StartDate(String),
    NumOfEvents(String),
    DaysAfterStart(String),
}

pub struct CashGeneratorInput {
    amount: String,
    cr_kind: String,
    cr_name: String,
    dr_kind: String,
    dr_name: String,
    gst: String,
    narration: String,
    resolution: String,
    start_date: String,
    num_of_events: String,
    days_after_start: String,

}

impl default::Default for CashGeneratorInput {
    fn default() -> CashGeneratorInput {
        CashGeneratorInput {
            amount: String::from("0.0"),
            cr_kind: accounting::AccountKind::Equity.to_string(),
            cr_name: String::new(),
            dr_kind: accounting::AccountKind::Asset(accounting::AssetKind::Current).to_string(),
            dr_name: String::new(),
            narration: String::new(),
            gst: accounting::GstStatus::Including.to_string(),
            resolution: String::from("date"),
            start_date: chrono::NaiveDate::from_ymd(2020, 12, 6)
                .format("%Y-%m-%d")
                .to_string(),
            num_of_events: "0".to_string(),
            days_after_start: "0".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct CashTransfers {
    daily: collections::BTreeMap<ulid::Ulid, accounting::CashTransfer<resolution::Date>>,
    monthly: collections::BTreeMap<ulid::Ulid, accounting::CashTransfer<resolution::Month>>,
    quarterly: collections::BTreeMap<ulid::Ulid, accounting::CashTransfer<resolution::Quarter>>,
    yearly: collections::BTreeMap<ulid::Ulid, accounting::CashTransfer<resolution::Year>>,
    #[serde(skip)]
    ulid_gen: ulid::Generator,
}

impl CashGeneratorInput {
    fn get_generator_date(&self) -> Result<Option<accounting::CashTransfer<resolution::Date>>> {
        if self.resolution == "date" {
            self.get_generator().map(Some)
        } else {
            Ok(None)
        }
    }
    fn get_generator_month(&self) -> Result<Option<accounting::CashTransfer<resolution::Month>>> {
        if self.resolution == "month" {
            self.get_generator().map(Some)
        } else {
            Ok(None)
        }
    }
    fn get_generator_quarter(&self) -> Result<Option<accounting::CashTransfer<resolution::Quarter>>> {
        if self.resolution == "quarter" {
            self.get_generator().map(Some)
        } else {
            Ok(None)
        }
    }
    fn get_generator_year(&self) -> Result<Option<accounting::CashTransfer<resolution::Year>>> {
        if self.resolution == "year" {
            self.get_generator().map(Some)
        } else {
            Ok(None)
        }
    }
    fn get_generator<R: resolution::DateResolution>(&self) -> Result<accounting::CashTransfer<R>> {
        Ok(accounting::CashTransfer {
            range: resolution::TimeRange::new(
                self.start_date.parse(),
                self.num_of_events.parse()?,
            ),
            days_after_start: self.days_after_start.parse()?,
            // date: chrono::NaiveDate::parse_from_str(&self.date, "%Y-%m-%d")?.into(),
            cr_account: accounting::Account {
                name: self.cr_name.to_string(),
                classification: self.cr_kind.parse()?,
            },
            dr_account: accounting::Account {
                name: self.dr_name.to_string(),
                classification: self.dr_kind.parse()?,
            },
            narration: self.narration.to_string(),
            amount: self.amount.parse()?,
            gst: self.gst.parse()?,
        })
    }
    pub fn view(&self) -> Vec<node::Node<InputMsg>> {
        vec![
            input::horizontal_group("Date", &[input::date(&self.date, InputMsg::Date)]),
            input::horizontal_group(
                "Narration",
                &[input::text(&self.narration, InputMsg::Narration)],
            ),
            input::horizontal_group(
                "DR Account",
                &[
                    input::select(&self.dr_kind, &account_kinds, InputMsg::DrKind),
                    input::text(&self.dr_name, InputMsg::DrName),
                ],
            ),
            input::horizontal_group(
                "CR Account",
                &[
                    input::select(&self.cr_kind, &account_kinds, InputMsg::CrKind),
                    input::text(&self.cr_name, InputMsg::CrName),
                ],
            ),
            input::horizontal_group("Amount", &[input::number(&self.amount, InputMsg::Amount)]),
        ]
    }
    pub fn update(&mut self, msg: InputMsg) {
        match msg {
            InputMsg::Amount(input) => {
                log!("Got input: {}", input);
                self.amount = input;
            }
            InputMsg::CrKind(input) => {
                log!("Got input: {}", input);
                self.cr_kind = input;
            }
            InputMsg::CrName(input) => {
                log!("Got input: {}", input);
                self.cr_name = input;
            }
            InputMsg::DrKind(input) => {
                log!("Got input: {}", input);
                self.dr_kind = input;
            }
            InputMsg::DrName(input) => {
                log!("Got input: {}", input);
                self.dr_name = input;
            }
            InputMsg::Gst(input) => {
                log!("Got input: {}", input);
                self.gst = input;
            }
            InputMsg::Narration(input) => {
                log!("Got input: {}", input);
                self.narration = input;
            }
            InputMsg::StartDate(input) => {
                log!("Got input: {}", input);
                self.start_date = input;
            }
            InputMsg::NumOfEvents(input) => {
                log!("Got input: {}", input);
                self.num_of_events = input;
            }
            InputMsg::DaysAfterStart(input) => {
                log!("Got input: {}", input);
                self.days_after_start = input;
            }
        }
    }
}