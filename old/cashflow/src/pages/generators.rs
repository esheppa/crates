use crate::accounting;
use resolution;
use seed::{*, prelude::*, virtual_dom::node, app::orders};

pub struct Model {
}
impl Model {
    pub fn view(&self) -> node::Node<Msg> {
        p!["Transactions"]
    }
    pub fn init(_orders: &mut impl orders::Orders<Msg>) -> Model {
        Model {

        }
    }
    pub fn title(&self) -> String {
        "Transactions".to_string()
    }
    pub fn update(&mut self, msg: Msg,  orders: &mut impl orders::Orders<Msg>) {
    
    }
}
pub enum Msg {
}    
