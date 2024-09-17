use crate::{accounting, input};
use once_cell::sync;
use resolution;
use seed::{app::orders, browser::web_storage, prelude::*, virtual_dom::node, *};
use serde::{Deserialize, Serialize};
use std::{collections, default};

const STORAGE_KEY: &str = "opening-balances";

// we don't allow expense/revenue accounts when used for opening balances.
static ALLOWED_ACCOUNTS: sync::Lazy<collections::BTreeSet<accounting::AccountKind>> =
    sync::Lazy::new(|| {
        let mut m = collections::BTreeSet::new();
        m.insert(accounting::AccountKind::Asset(
            accounting::AssetKind::Current,
        ));
        m.insert(accounting::AccountKind::Asset(
            accounting::AssetKind::NonCurrent,
        ));
        m.insert(accounting::AccountKind::Asset(accounting::AssetKind::Bank));
        m.insert(accounting::AccountKind::Liability(
            accounting::LiabilityKind::Current,
        ));
        m.insert(accounting::AccountKind::Liability(
            accounting::LiabilityKind::NonCurrent,
        ));
        m.insert(accounting::AccountKind::Equity);
        m
    });

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Expected {required} but instead tried to add a transaction with date {got}")]
    WrongDate {
        got: resolution::Date,
        required: resolution::Date,
    },
    #[error("Expected Asset, Liability or Equity but got {got:?}")]
    WrongAccountType { got: accounting::AccountKind },
    #[error("Ulid: {0}")]
    Ulid(#[from] ulid::MonotonicError),
    #[error("Chrono: {0}")]
    Chrono(#[from] chrono::ParseError),
    #[error("Decimal: {0}")]
    Decimal(#[from] rust_decimal::Error),
    #[error("Accounting parse error: {0}")]
    Accounting(#[from] accounting::ParseError),
    #[error("Web storage error: {0:?}")]
    WebStorage(web_storage::WebStorageError),
}

type Result<T> = std::result::Result<T, Error>;

pub enum Msg {
    AddTransaction,
    RemoveTransaction(ulid::Ulid),
    ClearAll,
    EditTransaction(ulid::Ulid),
    Input(InputMsg),
}

pub struct Model {
    opening_balances: OpeningBalances,
    current_id: Option<ulid::Ulid>,
    input: TransactionInput,
    recent_error: Result<()>,
}

impl default::Default for Model {
    fn default() -> Model {
        Model {
            opening_balances: OpeningBalances::new(chrono::NaiveDate::from_ymd(2020, 12, 6)),
            input: TransactionInput::default(),
            current_id: None,
            recent_error: Ok(()),
        }
    }
}

impl Model {
    pub fn view(&self) -> node::Node<Msg> {
        let add_text = if self.current_id.is_none() {
            "Add transaction"
        } else {
            "Save transaction"
        };
        div![
            C!["columns"],
            div![
                C!["column"],
                self.input.view().map_msg(Msg::Input),
                input::horizontal_group(
                    "",
                    &[
                        input::button(add_text, || Msg::AddTransaction),
                        input::button("Clear Transactions", || Msg::ClearAll),
                    ]
                ),
                p![if let Err(e) = &self.recent_error {
                    e.to_string()
                } else {
                    "".to_string()
                }],
            ],
            div![
                C!["column"],
                self.opening_balances.view()
            ],
        ]
    }
    pub fn init(_orders: &mut impl orders::Orders<Msg>) -> Model {
        if let Ok(opening_balances) = web_storage::LocalStorage::get(STORAGE_KEY) {
            Model {
                opening_balances,
                ..Model::default()
            }
        } else {
            Model::default()
        }
    }
    pub fn title(&self) -> String {
        "Opening Balance".to_string()
    }

    fn persist_to_storage(&mut self) {
        self.recent_error = web_storage::LocalStorage::insert(STORAGE_KEY, &self.opening_balances)
            .map_err(Error::WebStorage);
    }
    pub fn update(&mut self, msg: Msg, _orders: &mut impl orders::Orders<Msg>) {
        match msg {
            Msg::Input(input_msg) => self.input.update(input_msg),
            Msg::AddTransaction => {
                match self.input.get_transaction() {
                    Ok(tran) => {
                        log!("Adding transaction");
                        self.recent_error =
                            self.opening_balances.add_transaction(self.current_id, tran);
                    }
                    Err(e) => {
                        error!("Couldn't add transaction, {}", e);
                        self.recent_error = Err(e);
                    }
                }
                // persist to storage
                self.persist_to_storage();

                // reset inputs to defaults
                self.input = TransactionInput::default();
                self.current_id = None;
            }
            Msg::RemoveTransaction(id) => {
                self.opening_balances.transactions.remove(&id);
                // persist to storage
                self.persist_to_storage();
            }
            Msg::EditTransaction(id) => {
                if let Some(tran) = self.opening_balances.transactions.get(&id) {
                    self.input = TransactionInput {
                        date: tran.date.to_string(),
                        narration: tran.narration.to_string(),
                        cr_kind: tran.cr.classification.to_string(),
                        cr_name: tran.cr.name.to_string(),
                        dr_kind: tran.dr.classification.to_string(),
                        dr_name: tran.dr.name.to_string(),
                        amount: tran.amount.to_string(),
                    };
                    self.current_id = Some(id);
                }
            }
            Msg::ClearAll => {
                self.opening_balances.transactions = collections::BTreeMap::new();
                self.persist_to_storage();
            }
        }
    }
}

pub enum InputMsg {
    Date(String),
    Narration(String),
    CrKind(String),
    CrName(String),
    DrKind(String),
    DrName(String),
    Amount(String),
}

pub struct TransactionInput {
    date: String,
    narration: String,
    cr_kind: String,
    cr_name: String,
    dr_kind: String,
    dr_name: String,
    amount: String,
}

impl TransactionInput {
    fn get_transaction(&self) -> Result<accounting::Transaction> {
        Ok(accounting::Transaction {
            date: chrono::NaiveDate::parse_from_str(&self.date, "%Y-%m-%d")?.into(),
            cr: accounting::Account {
                name: self.cr_name.to_string(),
                classification: self.cr_kind.parse()?,
            },
            dr: accounting::Account {
                name: self.dr_name.to_string(),
                classification: self.dr_kind.parse()?,
            },
            narration: self.narration.to_string(),
            amount: self.amount.parse()?,
        })
    }
    pub fn view(&self) -> Vec<node::Node<InputMsg>> {
        let account_kinds = ALLOWED_ACCOUNTS.iter().map(ToString::to_string).collect();
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
            InputMsg::Date(input) => {
                log!("Got input: {}", input);
                self.date = input;
            }
            InputMsg::Narration(input) => {
                log!("Got input: {}", input);
                self.narration = input;
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
            InputMsg::Amount(input) => {
                log!("Got input: {}", input);
                self.amount = input;
            }
        }
    }
}

impl default::Default for TransactionInput {
    fn default() -> TransactionInput {
        TransactionInput {
            date: chrono::NaiveDate::from_ymd(2020, 12, 6)
                .format("%Y-%m-%d")
                .to_string(),
            narration: String::new(),
            cr_kind: accounting::AccountKind::Equity.to_string(),
            cr_name: String::new(),
            dr_kind: accounting::AccountKind::Asset(accounting::AssetKind::Current).to_string(),
            dr_name: String::new(),
            amount: String::from("0.0"),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct OpeningBalances {
    date: resolution::Date,
    #[serde(skip)]
    ulid_gen: ulid::Generator,
    transactions: collections::BTreeMap<ulid::Ulid, accounting::Transaction>,
}

impl OpeningBalances {
    pub fn new(date: chrono::NaiveDate) -> OpeningBalances {
        OpeningBalances {
            date: date.into(),
            transactions: collections::BTreeMap::new(),
            ulid_gen: ulid::Generator::new(),
        }
    }
    pub fn add_transaction(
        &mut self,
        current_id: Option<ulid::Ulid>,
        transaction: accounting::Transaction,
    ) -> Result<()> {
        let id = match current_id {
            Some(ulid) => ulid,
            None => self.ulid_gen.generate()?,
        };

        if !ALLOWED_ACCOUNTS.contains(&transaction.dr.classification) {
            return Err(Error::WrongAccountType {
                got: transaction.dr.classification,
            });
        }

        if !ALLOWED_ACCOUNTS.contains(&transaction.cr.classification) {
            return Err(Error::WrongAccountType {
                got: transaction.cr.classification,
            });
        }

        if transaction.date != self.date {
            return Err(Error::WrongDate {
                got: transaction.date,
                required: self.date,
            });
        }

        self.transactions.insert(id, transaction);
        Ok(())
    }
    pub fn view(&self) -> node::Node<Msg> {
        table![
            C!["table"],
            thead![tr![
                th![""],
                th!["Narration"],
                th!["Account Type"],
                th!["Account Name"],
                th!["DR"],
                th!["CR"],
            ],],
            tbody![self
                .transactions
                .iter()
                .map(|(id, tran)| transaction_to_rows(*id, tran))
                .flatten()
                .collect::<Vec<_>>()]
        ]
    }
}

fn transaction_to_rows(
    id: ulid::Ulid,
    tran: &accounting::Transaction,
) -> impl std::iter::Iterator<Item = Node<Msg>> {
    vec![
        // DR row
        tr![
            td![
                attrs! {At::RowSpan => 2},
                input::button("Edit", move || Msg::EditTransaction(id)),
                input::button("Remove", move || Msg::RemoveTransaction(id)),
            ],
            td![&tran.narration, attrs! {At::RowSpan => 2}],
            td![tran.dr.classification.to_string()],
            td![&tran.dr.name],
            td![tran.amount.to_string()],
            td![""],
        ],
        // CR row
        tr![
            td![tran.cr.classification.to_string()],
            td![&tran.cr.name],
            td![""],
            td![tran.amount.to_string()],
        ],
    ]
    .into_iter()
}
