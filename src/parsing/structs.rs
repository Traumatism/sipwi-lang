use crate::common::sipwi::Sipwi;
use crate::lexing::token::Token;

// Every variable type
#[derive(Debug, Clone)]
pub enum Variable {
    Str(String),
    Number(isize),
    Bool(bool),
}

// Every function type
pub enum Function<'a> {
    NonStd(&'a Func),
    Std(&'a StdFunc),
}

/// Describes a standard function output
pub struct StdFuncResult {
    token: Token,
}

impl StdFuncResult {
    pub fn new(token: Token) -> Self {
        Self { token }
    }

    /// Empty result
    pub fn empty() -> Self {
        Self {
            token: Token::List(Vec::new()),
        }
    }

    /// Get tokens
    pub fn get_tokens(&self) -> &Token {
        &self.token
    }
}

/// Describes a function
pub struct Func {
    pub args: Vec<String>,
    pub tokens: Vec<Token>,
}

impl Func {
    pub fn new(args: Vec<String>, tokens: Vec<Token>) -> Self {
        Self { args, tokens }
    }
}

/// Describes a standard function written in Rust
pub struct StdFunc {
    pub call: fn(&Sipwi, Token) -> StdFuncResult,
}

impl StdFunc {
    pub fn new(func: for<'a> fn(&'a Sipwi, Token) -> StdFuncResult) -> Self {
        Self { call: func }
    }
}
