use crate::common::sipwi::Sipwi;
use crate::lexing::token::Token;

// Every variable type
#[derive(Debug, Clone)]
pub enum Type {
    Str(String),
    Number(isize),
    Bool(bool),
    // List(Vec<Type>),
}

// Every Callable type
pub enum Callable<'a> {
    Procedure(&'a Procedure),
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

/// Describes a procedure
pub struct Procedure {
    pub args: Vec<String>,
    pub tokens: Vec<Token>,
}

impl Procedure {
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
