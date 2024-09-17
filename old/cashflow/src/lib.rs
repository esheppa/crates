mod pages;
mod accounting;
mod input;
mod ui;
mod fields;
use pages::{opening_balances, generators, edit_accrual_generator, edit_cash_generator, balance_sheet_report, cash_flow_report, profit_and_loss_report};

use seed::{prelude::*, *};

struct Model {
    page: Page,
    url: Url,
}


impl Model {
    fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
        orders.subscribe(Msg::UrlChanged);
        Model {
            page: Page::Home,
            url,
        }
    }
    fn view(model: &Model) -> Node<Msg> {

        div![
            C!["columns"],
            div![
                C!["column", "is-narrow", "has-background-grey-light"],
                menu_bar(),
            ],
            div![
                C!["column"],
                nav![
                    C!["level"],
                    h1![
                        C!["level-item", "has-text-centered", "is-size-1"],
                        model.page.title(),
                    ]
                ],
                model.page.view(),
            ],
        ]
    }
    fn update(&mut self, msg: Msg, orders: &mut impl Orders<Msg>) {
        match msg {
            Msg::UrlChanged(subs::UrlChanged(mut url)) => {
                log!(url);
                // model.url = url;
                match url.next_hash_path_part() {
                    Some("opening") => {
                        self.page = Page::OpeningBalances(opening_balances::Model::init(&mut orders.proxy(Msg::OpeningBalances)));
                    }
                    Some("generators") => {
                        self.page = Page::Generators(generators::Model::init(&mut orders.proxy(Msg::Generators)));
                    }
                    Some("edit_cash_generator") => {
                        self.page = Page::EditCashGenerator(edit_cash_generator::Model::init(&mut orders.proxy(Msg::EditCashGenerator)));
                    }
                    Some("edit_accrual_generator") => {
                        self.page = Page::EditAccrualGenerator(edit_accrual_generator::Model::init(&mut orders.proxy(Msg::EditAccrualGenerator)));
                    }
                    Some("balance_sheet") => {
                        self.page = Page::BalanceSheetReport(balance_sheet_report::Model::init(&mut orders.proxy(Msg::BalanceSheetReport)));
                    }
                    Some("profit_loss") => {
                        self.page = Page::ProfitAndLossReport(profit_and_loss_report::Model::init(&mut orders.proxy(Msg::ProfitAndLossReport)));
                    }
                    Some("cash_flow") => {
                        self.page = Page::CashFlowReport(cash_flow_report::Model::init(&mut orders.proxy(Msg::CashFlowReport)));
                    }
                    None => {
                        self.page = Page::Home
                    }
                    Some(_) => {
                        self.page = Page::NotFound
                    }
                }
            }
            // Msg::GoToUrl(url) => {
            //     orders.request_url(url);
            // }
            Msg::OpeningBalances(msg) => {
                if let Page::OpeningBalances(ref mut model) = self.page {
                    model.update(msg, &mut orders.proxy(Msg::OpeningBalances))
                } else {
                    log!("Unexpected opening_balances::Msg")
                }
            }
            Msg::Generators(msg) => {
                if let Page::Generators(ref mut model) = self.page {
                    model.update(msg, &mut orders.proxy(Msg::Generators))
                } else {
                    log!("Unexpected generators::Msg")
                }
            }
            Msg::EditCashGenerator(msg) => {
                if let Page::EditCashGenerator(ref mut model) = self.page {
                    model.update(msg, &mut orders.proxy(Msg::EditCashGenerator))
                } else {
                    log!("Unexpected edit_cash_generator::Msg")
                }
            }
            Msg::EditAccrualGenerator(msg) => {
                if let Page::EditAccrualGenerator(ref mut model) = self.page {
                    model.update(msg, &mut orders.proxy(Msg::EditAccrualGenerator))
                } else {
                    log!("Unexpected edit_accrual_generator::Msg")
                }
            }
            Msg::BalanceSheetReport(msg) => {
                if let Page::BalanceSheetReport(ref mut model) = self.page {
                    model.update(msg, &mut orders.proxy(Msg::BalanceSheetReport))
                } else {
                    log!("Unexpected balance_sheet_report::Msg")
                }
            }
            Msg::CashFlowReport(msg) => {
                if let Page::CashFlowReport(ref mut model) = self.page {
                    model.update(msg, &mut orders.proxy(Msg::CashFlowReport))
                } else {
                    log!("Unexpected cash_flow_report::Msg")
                }
            }
            Msg::ProfitAndLossReport(msg) => {
                if let Page::ProfitAndLossReport(ref mut model) = self.page {
                    model.update(msg, &mut orders.proxy(Msg::ProfitAndLossReport))
                } else {
                    log!("Unexpected profit_and_loss_report::Msg")
                }
            }
        }
    }
}

enum Msg {
    UrlChanged(subs::UrlChanged),
    // GoToUrl(Url),
    OpeningBalances(opening_balances::Msg),
    Generators(generators::Msg),
    EditCashGenerator(edit_cash_generator::Msg),
    EditAccrualGenerator(edit_accrual_generator::Msg),
    BalanceSheetReport(balance_sheet_report::Msg),
    CashFlowReport(cash_flow_report::Msg),
    ProfitAndLossReport(profit_and_loss_report::Msg),
}

fn menu_bar() -> Node<Msg> {

    aside![
        C!["menu"],
        ul![
            C!["menu-list"],
            li![
                a![
                    attrs! {
                        At::Href => "#",
                    },
                    "Home",
                ]
            ],
            li![
                a![
                    attrs! {
                        At::Href => "#opening",
                    },
                    "Opening balances"
                ]
            ],
            li![
                a![
                    attrs! {
                        At::Href => "#generators",
                    },
                    "Generators"
                ]
            ],
            li![
                a![
                    attrs! {
                        At::Href => "#balance_sheet",
                    },
                    "Balance sheet report"
                ]
            ],
            li![
                a![
                    attrs! {
                        At::Href => "#cash_flow",
                    },
                    "Cash flow report"
                ]
            ],
            li![
                a![
                    attrs! {
                        At::Href => "#profit_loss",
                    },
                    "Profit and loss report"
                ]
            ],
        ]
    ]
}



enum Page {
    Home,
    NotFound,
    OpeningBalances(opening_balances::Model),
    Generators(generators::Model),
    EditCashGenerator(edit_cash_generator::Model),
    EditAccrualGenerator(edit_accrual_generator::Model),
    BalanceSheetReport(balance_sheet_report::Model),
    CashFlowReport(cash_flow_report::Model),
    ProfitAndLossReport(profit_and_loss_report::Model),
}

impl Page {
    fn view(&self) -> Node<Msg> {
        match &self {
            Page::Home => {
                p!["Home Page"]
            },
            Page::NotFound => {
                p!["Not found"]
            },
            Page::OpeningBalances(model) => {
                model.view().map_msg(Msg::OpeningBalances)
            },
            Page::Generators(model) => {
                model.view().map_msg(Msg::Generators)
            },
            Page::EditCashGenerator(model) => {
                model.view().map_msg(Msg::EditCashGenerator)
            },
            Page::EditAccrualGenerator(model) => {
                model.view().map_msg(Msg::EditAccrualGenerator)
            },
            Page::BalanceSheetReport(model) => {
                model.view().map_msg(Msg::BalanceSheetReport)
            },
            Page::CashFlowReport(model) => {
                model.view().map_msg(Msg::CashFlowReport)
            },
            Page::ProfitAndLossReport(model) => {
                model.view().map_msg(Msg::ProfitAndLossReport)
            },
        }
    }
    fn title(&self) -> String {
        match &self {
            Page::Home => "Home".to_string(),
            Page::NotFound => "Not Found".to_string(),
            Page::OpeningBalances(model) => model.title(),
            Page::Generators(model) => model.title(),
            Page::EditCashGenerator(model) => model.title(),
            Page::EditAccrualGenerator(model) => model.title(),
            Page::BalanceSheetReport(model) => model.title(),
            Page::CashFlowReport(model) => model.title(),
            Page::ProfitAndLossReport(model) => model.title(),
        }
    }
}




#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", Model::init, |msg, model, orders| model.update(msg, orders), Model::view);
}



// fn main() {

//     let salary = CashTransfer {
//         amount: rust_decimal::Decimal::new(50_000, 0),
//         dr_account: Account {
//             name: "Bank".to_string(),
//             classification: AccountKind::Asset(AssetKind::Cash),
//         },
//         cr_account: Account {
//             name: "Salary".to_string(),
//             classification: AccountKind::Expense,
//         },
//         gst: GstStatus::None,
//         range: resolution::TimeRange::new(
//             resolution::Month::from_date(chrono::NaiveDate::from_ymd(2020, 1, 1)),
//             600,
//         ),
//         days_after_start: 10,
//         narration: "Salary payments".to_string(),
//     };

//     let data = vec![salary.clone(), salary];

//     println!("{}", ser::to_string_pretty(&data, ser::PrettyConfig::new()).unwrap());
// }
