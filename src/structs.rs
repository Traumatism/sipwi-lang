use crate::sipwi::Sipwi;
use crate::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    pub tokens: Vec<Token>,
}

impl Expression {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens }
    }

    pub fn _evaluate(&self, _env: &&mut Sipwi) -> Vec<Token> {
        let tokens = Vec::new();

        tokens
    }
}

/// Describes a standard function output
pub struct StdFuncResult {
    pub tokens: Token,
}

/// Describes a function
pub struct Func {
    pub fnc_args: Vec<String>,
    pub fnc_tokens: Vec<Token>,
}

/// Describe a standard function written in Rust
pub struct StdFunc {
    pub call: Box<dyn Fn(&&mut Sipwi, Token) -> Option<StdFuncResult>>,
}

impl StdFunc {
    pub fn new(func: for<'a, 'b> fn(&'a &'b mut Sipwi, Token) -> Option<StdFuncResult>) -> Self {
        Self {
            call: Box::new(func),
        }
    }
}
