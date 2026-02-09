pub mod handlers;

#[derive(Debug)]
pub struct Request {
    pub ip: String,
    pub token: String,
    pub role: String,
}

#[derive(Debug, PartialEq)]
pub enum AuthError {
    InvalidIP,
    InvalidToken,
    InvalidRole,
}

pub trait AccessHandler {
    fn set_next(&mut self, next: Box<dyn AccessHandler>);
    fn handle(&self, request: &Request) -> Result<(), AuthError>;
}
