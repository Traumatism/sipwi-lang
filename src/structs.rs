use crate::sipwi::Sipwi;
use crate::token::Token;

/// Sipwi function
pub struct Func {
    pub fnc_args: Vec<String>,
    pub fnc_tokens: Vec<Token>,
}

/// Standard function (rust <=> sipwi)
pub struct StdFunc {
    pub call: Box<dyn Fn(&&mut Sipwi, Token) -> Option<()>>,
}

impl StdFunc {
    pub fn new(func: for<'a, 'b> fn(&'a &'b mut Sipwi, Token) -> Option<()>) -> Self {
        Self {
            call: Box::new(func),
        }
    }
}
