use crate::common::sipwi::Sipwi;
use crate::lexing::token::Token;

// Every variable type
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Type {
    Str(String),
    Number(isize),
    Bool(bool),
    List(Vec<Type>), // Not implemented yet
}

// Every Callable type
pub enum Callable<'a> {
    Procedure(&'a Procedure),
    Std(&'a StdFunc),
}

/// Describes a standard function output
pub struct StdFuncResult {
    output: Type,
}

impl StdFuncResult {
    pub fn new(output: Type) -> Self {
        Self { output }
    }

    /// Empty result
    pub fn empty() -> Self {
        Self {
            output: Type::List(Vec::new()),
        }
    }

    /// Get tokens
    pub fn get_output(&self) -> &Type {
        &self.output
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
    pub call: fn(&mut Sipwi, Type) -> StdFuncResult,
}

impl StdFunc {
    pub fn new(func: fn(&mut Sipwi, Type) -> StdFuncResult) -> Self {
        Self { call: func }
    }
}
