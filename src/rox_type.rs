#[derive(Debug, Clone, PartialEq)]
pub enum RoxType {
    String(String),
    Number(f64),
    Nil,
    Boolean(bool),
}

impl RoxType {
    pub fn to_string(&self) -> String{
        match self{
            RoxType::String(x) => x.clone(),
            RoxType::Number(x) => x.to_string(),
            RoxType::Nil => String::from("Nil"),
            RoxType::Boolean(x) => x.to_string(),
        }
    }
}