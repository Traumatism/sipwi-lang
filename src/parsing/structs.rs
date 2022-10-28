use crate::lexing::token::Token;
use crate::parsing::parser::Parser;
use crate::sipwi::Sipwi;

// Every variable type
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

    pub fn evaluate(&self, env: &mut Sipwi) {
        Parser::new(self.tokens.clone(), env).parse_tokens()
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
    pub call: Box<dyn Fn(&&mut Sipwi, Token) -> StdFuncResult>,
}

impl StdFunc {
    pub fn new(func: for<'a, 'b> fn(&'a &'b mut Sipwi, Token) -> StdFuncResult) -> Self {
        Self {
            call: Box::new(func),
        }
    }
}
