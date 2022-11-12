#[allow(dead_code)] //boolean
#[derive(Debug, Clone)]
pub enum RoxType {
    String(String),
    Number(f64),
    Nil,
    Boolean(bool),
}