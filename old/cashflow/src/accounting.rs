use resolution::TimeResolution;
use serde::{de, Deserialize, Serialize};
use std::{collections, iter};

#[derive(Debug, Deserialize, Serialize)]
pub struct GeneralLedger {
    transactions: Vec<Transaction>,
}

impl GeneralLedger {
    pub fn cash_flow_statement<R>(&self, period: resolution::TimeRange<R>) -> CashFlowStatement<R>
    where
        R: resolution::DateResolution,
    {
        todo!()
    }
    pub fn profit_and_loss<R>(&self, period: resolution::TimeRange<R>) -> ProfitAndLoss<R>
    where
        R: resolution::DateResolution,
    {
        todo!()
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(bound(deserialize = "R: de::DeserializeOwned"))]
pub struct ProfitAndLoss<R: resolution::DateResolution> {
    periods: collections::BTreeMap<R, ProfitAndLossSegment>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProfitAndLossSegment {
    // would be nice if only Exp/Rev accounts ca be used here
    data: collections::BTreeMap<Account, rust_decimal::Decimal>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(bound(deserialize = "R: de::DeserializeOwned"))]
pub struct CashFlowStatement<R: resolution::DateResolution> {
    periods: collections::BTreeMap<R, ProfitAndLossSegment>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CashFlowSegment {
    // would be nice if only Bank accounts ca be used here
    data: collections::BTreeMap<Account, rust_decimal::Decimal>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Transaction {
    pub date: resolution::Date,
    pub narration: String,
    pub cr: Account,
    pub dr: Account,
    pub amount: rust_decimal::Decimal,
}

impl Transaction {}

#[derive(Debug, Deserialize, Serialize, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Account {
    pub name: String,
    pub classification: AccountKind,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum AccountKind {
    Asset(AssetKind),
    Liability(LiabilityKind),
    Equity,
    Expense,
    Revenue,
}

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Unexpected account kind: {0}")]
    AccountKind(String),
    #[error("Unexpected gst status: {0}")]
    Gst(String),
}

impl std::string::ToString for AccountKind {
    fn to_string(&self) -> String {
        match self {
            AccountKind::Equity => "equity",
            AccountKind::Revenue => "revenue",
            AccountKind::Expense => "expense",
            AccountKind::Asset(AssetKind::Bank) => "bank",
            AccountKind::Asset(AssetKind::Current) => "current asset",
            AccountKind::Asset(AssetKind::NonCurrent) => "non-current asset",
            AccountKind::Liability(LiabilityKind::Current) => "current liability",
            AccountKind::Liability(LiabilityKind::NonCurrent) => "non-current liability",
        }.to_string()
    }
}

impl std::str::FromStr for AccountKind {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "equity" => Ok(AccountKind::Equity),
            "expense" => Ok(AccountKind::Expense),
            "revenue" => Ok(AccountKind::Revenue),
            "non-current asset" => Ok(AccountKind::Asset(AssetKind::NonCurrent)),
            "current asset" => Ok(AccountKind::Asset(AssetKind::Current)),
            "bank" => Ok(AccountKind::Asset(AssetKind::Bank)),
            "non-current liability" => Ok(AccountKind::Liability(LiabilityKind::NonCurrent)),
            "current liability" => Ok(AccountKind::Liability(LiabilityKind::Current)),
            otherwise => Err(ParseError::AccountKind(otherwise.to_string())),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum AssetKind {
    Bank,
    Current,
    NonCurrent,
}


#[derive(Debug, Deserialize, Serialize, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum LiabilityKind {
    Current,
    NonCurrent,
}

pub trait GenerateTransactions {
    fn generate_transactions(&self) -> Vec<Transaction>;
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum GstStatus {
    Including,
    Excluding,
    None,
}


impl std::string::ToString for GstStatus {
    fn to_string(&self) -> String {
        match self {
            GstStatus::Including => "including",
            GstStatus::Excluding => "excluding",
            GstStatus::None => "none",
        }.to_string()
    }
}

impl std::str::FromStr for GstStatus {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "including" => Ok(GstStatus::Including),
            "excluding" => Ok(GstStatus::Excluding),
            "none" => Ok(GstStatus::None),
            otherwise => Err(ParseError::Gst(otherwise.to_string())),
        }
    }
}

// later add default / cancellation probability
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(bound(deserialize = "R: de::DeserializeOwned"))]
pub struct CashTransfer<R: resolution::DateResolution> {
    amount: rust_decimal::Decimal,
    dr_account: Account,
    cr_account: Account,
    gst: GstStatus,
    range: resolution::TimeRange<R>,
    // this is safe, it just may roll through into the next period
    // if specified too long, eg 8 for weekly based range
    days_after_start: u32,
    narration: String,
}

impl<R> GenerateTransactions for CashTransfer<R>
where
    R: resolution::DateResolution,
{
    fn generate_transactions(&self) -> Vec<Transaction> {
        self.range
            .iter()
            .map(|event| {
                Transaction {
                    date: resolution::Date::from(event.start()).succ_n(self.days_after_start),
                    narration: self.narration.to_string(),
                    amount: self.amount,
                    cr: self.cr_account.clone(),
                    dr: self.dr_account.clone(),
                }
                .add_gst(self.gst)
            })
            .flatten()
            .collect()
    }
}

// later add late payment distribution (exponential?)
// and default / cancellation probability
#[derive(Debug, Deserialize, Serialize)]
#[serde(bound(deserialize = "R: de::DeserializeOwned"))]
pub struct AccrualTransfer<R: resolution::DateResolution> {
    amount: rust_decimal::Decimal,
    raise_dr_account: Account,
    raise_cr_account: Account,
    settle_dr_account: Account,
    settle_cr_account: Account,
    gst: GstStatus,
    range: resolution::TimeRange<R>,
    // this is safe, it just may roll through into the next period
    // if specified too long, eg 8 for weekly based range
    days_after_start_raised: u32,
    payment_terms: u32,
    narration: String,
}

impl<R> GenerateTransactions for AccrualTransfer<R>
where
    R: resolution::DateResolution,
{
    fn generate_transactions(&self) -> Vec<Transaction> {
        let created = self
            .range
            .iter()
            .map(|event| {
                Transaction {
                    date: resolution::Date::from(event.start())
                        .succ_n(self.days_after_start_raised),
                    narration: self.narration.to_string(),
                    amount: self.amount,
                    cr: self.raise_cr_account.clone(),
                    dr: self.raise_dr_account.clone(),
                }
                .add_gst(self.gst)
            })
            .flatten();
        let paid = self
            .range
            .iter()
            .map(|event| {
                Transaction {
                    date: resolution::Date::from(event.start())
                        .succ_n(self.days_after_start_raised)
                        .succ_n(self.payment_terms),
                    narration: self.narration.to_string(),
                    amount: self.amount,
                    cr: self.settle_cr_account.clone(),
                    dr: self.settle_dr_account.clone(),
                }
                .add_gst(self.gst)
            })
            .flatten();

        created.chain(paid).collect()
    }
}

impl Transaction {
    fn add_gst(self, gst: GstStatus) -> impl Iterator<Item = Transaction> {
        let gst_transaction = if let GstStatus::None = gst {
            None
        } else {
            let Transaction {
                amount,
                cr,
                dr,
                narration,
                date,
            } = self.clone();
            let gst_acc = Account {
                name: "GST".to_string(),
                classification: AccountKind::Liability(LiabilityKind::Current),
            };
            let amount = match gst {
                GstStatus::Excluding => self.amount * rust_decimal::Decimal::new(1, 1),
                GstStatus::Including => self.amount / rust_decimal::Decimal::new(11, 0),
                GstStatus::None => unreachable!(),
            };
            let dr = match self.dr.classification {
                AccountKind::Expense => gst_acc.clone(),
                otherwise => dr,
            };
            let cr = match self.cr.classification {
                AccountKind::Revenue => gst_acc,
                otherwise => cr,
            };

            Some(Transaction {
                cr,
                dr,
                amount,
                narration,
                date,
            })
        };
        iter::once(self).chain(gst_transaction.into_iter())
    }
}
