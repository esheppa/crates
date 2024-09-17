// #[derive(Serialize, UserInput)]
// #[ui(struct_name="PersonInput")]
// struct Person {
//     name: String,
//     #[ui = "Integer"]
//     #[validations(

//     )]
//     age: u64,

// }

// creates
// struct PersonInput {
//     name: Text,
//     age: Integer,
//     account: AccountInput
// }

// enum PersonMsg {
//     Name(String),
//     Age(String),
//     Account(AccountMsg)
// }

// impl PersonInput {
//     fn update(&mut self, change: PersonMsg) {
//         match change {
//             PersonMsg::Name(i) => self.name.update(i),
//             PersonMsg::Age(i) => self.age.update(i),
//             PersonMsg::Account(i) => self.account.update(i),
//         }
//     }
// }

use crate::{accounting, ui};
use std::{num, marker, error, fmt, convert};
use seed::{*, prelude::*, virtual_dom::node};








struct AccountInput<M> {
    name: String,
    style: String,
    m: marker::PhantomData<M>,
}


impl<M> From<accounting::Account> for AccountInput<M> {
    fn from(account: accounting::Account) -> AccountInput<M> {
        AccountInput { name: account.name, style: account.classification.to_string(), m: marker::PhantomData }
    }
}

impl<M> Default for AccountInput<M> {
    fn default() -> AccountInput<M> {
        AccountInput {
            name: String::new(),
            style: String::new(),
            m: marker::PhantomData,
        }
    }
}


enum AccountMsg {
    Name(String),
    Style(String),
}


impl<M> ui::UserInput for AccountInput<M> {
    type UI = node::Node<M>;
    type Msg = M;
    type Output = accounting::Account;
    type Input =  AccountMsg;
    type Error = accounting::ParseError;
    fn render<F>(&self, msg_ctor: F) -> Self::UI 
    where 
        F: FnOnce(String) -> Self::Msg + Clone + 'static,
        Self::Msg: 'static,
    {    
        todo!()
    }
    fn get_input(&self) -> &Self::Input {
        &AccountMsg::Name(self.name)
    }
    fn update(&mut self, input: Self::Input) {
        match input {
            AccountMsg::Name(i) => self.name = i,
            AccountMsg::Style(i) => self.style = i,
        }
    }
    fn parse(&self) -> Result<Self::Output, Self::Error> {
        Ok(accounting::Account { 
            name: self.name.to_string(),
            classification: self.style.parse()?,
        })
    }
}

pub struct Text<M> {
    pub input: String,
    m: marker::PhantomData<M>,
}

impl<M> Text<M> {
    fn new(input: String) -> Text<M> {
        Text { input, m: marker::PhantomData }
    }
}

impl<M> Default for Text<M> {
    fn default() -> Text<M> {
        Text {
            input: String::new(),
            m: marker::PhantomData,
        }
    }
}

impl<M> ui::UserInput for Text<M> {
    type UI = node::Node<M>;
    type Msg = M;
    type Output = String;
    type Input =  String;
    type Error = convert::Infallible;

    fn render<F>(&self, msg_ctor: F) -> Self::UI 
    where 
        F: FnOnce(String) -> Self::Msg + Clone + 'static,
        Self::Msg: 'static,
    {
        div![
            C!["field"],
            div![
                C!["control"],
                input![
                    C!["input"],
                    attrs!{
                        At::Type => "text",
                        At::Value => self.input,
                    },
                    input_ev(Ev::Input, msg_ctor),
                ]
            ]
        ]
    }
    fn get_input(&self) -> &Self::Input {
        &self.input
    }
    fn update(&mut self, input: Self::Input) {
        self.input = input;
    }
    fn parse(&self) -> Result<Self::Output, Self::Error> {
        Ok(self.input.to_string())
    }
}

#[derive(Debug)]
pub struct SelectError {
    selected: String,
}

impl fmt::Display for SelectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Value {} is not in the list of allowed options", self.selected)
    }

}
impl error::Error for SelectError {}

struct Select<M> {
    input: String,
    options: Vec<String>,
    m: marker::PhantomData<M>,
}

impl<M> Select<M> {
    fn new(input: String, options: Vec<String>) -> Select<M> {
        Select { input, options, m: marker::PhantomData }
    }
}

impl<M> ui::UserInput for Select<M> {
    type UI = node::Node<M>;
    type Msg = M;
    type Output = String;
    type Input =  String;
    type Error = SelectError;

    fn render<F>(&self, msg_ctor: F) -> Self::UI 
    where 
        F: FnOnce(String) -> Self::Msg + Clone + 'static,
        Self::Msg: 'static,
    {
        let selectable = self.options.iter().map(|opt| 
            option![
                IF![opt == &self.input => attrs!{At::Selected => ""}],
                opt,
            ]
        ).collect::<Vec<Node<M>>>();
        
        div![
            C!["field"],
            div![
                C!["select"],
                select![
                    input_ev(Ev::Input, msg_ctor),
                    selectable,
                ]  
            ]
        ]
    }
    fn get_input(&self) -> &Self::Input {
        &self.input
    }
    fn update(&mut self, input: Self::Input) {
        self.input = input;
    }
    fn parse(&self) -> Result<Self::Output, Self::Error> {
        if let Some(_) = self.options.iter().find(|i| *i == &self.input) {
            Ok(self.input.to_string())
        } else {
            Err(SelectError {
                selected: self.input.to_string(),
            })
        }
    }
}

struct Integer<M> {
    input: String,
    m: marker::PhantomData<M>,
}

impl<M> Integer<M> {
    fn new(input: String) -> Integer<M> {
        Integer { input, m: marker::PhantomData }
    }
}

impl<M> Default for Integer<M> {
    fn default() -> Integer<M> {
        Integer {
            input: "0".to_string(),
            m: marker::PhantomData,
        }
    }
}

impl<M> ui::UserInput for Integer<M> {
    type UI = node::Node<M>;
    type Msg = M;
    type Output = i64;
    type Input =  String;
    type Error = num::ParseIntError;
    fn render<F>(&self, msg_ctor: F) -> Self::UI 
    where 
        F: FnOnce(String) -> Self::Msg + Clone + 'static,
        Self::Msg: 'static,
    {
        div![
            C!["field"],
            div![
                C!["control"],
                input![
                    C!["input"],
                    attrs!{
                        At::Type => "number",
                        At::Value => self.input,
                    },
                    input_ev(Ev::Input, msg_ctor),
                ]
            ]
        ]
    }
    fn get_input(&self) -> &Self::Input {
        &self.input
    }
    fn update(&mut self, input: Self::Input) {
        self.input = input;
    }
    fn parse(&self) -> Result<Self::Output, Self::Error> {
        Ok(self.input.parse()?)
    }
}

pub struct Date<M> {
    input: String,
    format: &'static str,
    m: marker::PhantomData<M>,
}

impl<M> Date<M> {
    fn new(input: String, format: &'static str) -> Date<M> {
        Date { input, format, m: marker::PhantomData }
    }
}

impl<M> Default for Date<M> {
    fn default() -> Date<M> {
        Date {
            input: "0".to_string(),
            format: "%Y-%m-%d",
            m: marker::PhantomData,
        }
    }
}


impl<M> ui::UserInput for Date<M> {
    type UI = node::Node<M>;
    type Msg = M;
    type Output = chrono::NaiveDate;
    type Input =  String;
    type Error = chrono::ParseError;
    fn render<F>(&self, msg_ctor: F) -> Self::UI 
    where 
        F: FnOnce(String) -> Self::Msg + Clone + 'static,
        Self::Msg: 'static,
    {
        div![
            C!["field"],
            div![
                C!["control"],
                input![
                    C!["input"],
                    attrs!{
                        At::Type => "date",
                        At::Value => self.input,
                    },
                    input_ev(Ev::Input, msg_ctor),
                ]
            ]
        ]
    }
    fn get_input(&self) -> &Self::Input {
        &self.input
    }
    fn update(&mut self, input: Self::Input) {
        self.input = input;
    }
    fn parse(&self) -> Result<Self::Output, Self::Error> {
        Ok(chrono::NaiveDate::parse_from_str(&self.input, self.format)?)
    }
}