use std::error;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub struct Validation {
    field_level: Vec<String>,
    struct_level: Vec<String>,
    // table_level: Vec<String>,
}


pub trait UserInput {
    type UI;
    type Msg;
    type Output;
    type Input;
    type Error: error::Error;
    fn render<F>(&self, msg_ctor: F) -> Self::UI
    where 
        F: FnOnce(String) -> Self::Msg + Clone + 'static,
        Self::Msg: 'static;
    fn get_input(&self) -> &Self::Input;
    fn update(&mut self, input: Self::Input);
    fn parse(&self) -> Result<Self::Output, Self::Error>;
}


pub trait Validate {
    // where None -> successful validation
    // Some(message) -> validation failure message
    fn validate(&self) -> Option<String>;
}
