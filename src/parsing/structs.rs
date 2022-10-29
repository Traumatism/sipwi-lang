use crate::lexing::token::Token;
use crate::parsing::parser::Parser;
use crate::sipwi::Sipwi;

// Every variable type
#[derive(Debug)]
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

#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    pub tokens: Vec<Token>,
}

impl Expression {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens }
    }

    pub fn evaluate(self, env: &mut Sipwi) -> Option<Token> {
        Parser::new(self.tokens, env, true).parse_tokens()
    }
}

/// Describes a standard function output
pub struct StdFuncResult {
    token: Token,
}

impl StdFuncResult {
    pub fn new(token: Token) -> Self {
        Self { token }
    }

    pub fn empty() -> Self {
        Self {
            token: Token::List(Vec::new()),
        }
    }

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
    pub call: Box<dyn Fn(&Sipwi, Token) -> StdFuncResult>,
}

impl StdFunc {
    pub fn new(func: for<'a> fn(&'a Sipwi, Token) -> StdFuncResult) -> Self {
        Self {
            call: Box::new(func),
        }
    }
}
