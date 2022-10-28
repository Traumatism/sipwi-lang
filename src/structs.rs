use crate::sipwi::Sipwi;
use crate::token::Token;

pub enum Variable {
    Str(String),
    Number(isize),
}

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

    pub fn _evaluate(&self, _env: &&mut Sipwi) -> Vec<Token> {
        // self.tokens.iter().map(|token| token);

        Vec::new()
    }
}

/// Describes a standard function output
pub struct StdFuncResult {
    tokens: Token,
}

impl StdFuncResult {
    pub fn new(tokens: Token) -> Self {
        match tokens {
            Token::List(_) => Self { tokens },
            _ => panic!(),
        }
    }

    pub fn get_tokens(&self) -> &Token {
        &self.tokens
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
    pub call: Box<dyn Fn(&&mut Sipwi, Token) -> Option<StdFuncResult>>,
}

impl StdFunc {
    pub fn new(func: for<'a, 'b> fn(&'a &'b mut Sipwi, Token) -> Option<StdFuncResult>) -> Self {
        Self {
            call: Box::new(func),
        }
    }
}
