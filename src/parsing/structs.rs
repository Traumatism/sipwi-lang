use crate::common::sipwi::Sipwi;
use crate::lexing::token::Token;

#[derive(Debug, Clone)]
pub enum Type {
    Str(String),
    Number(isize),
    Bool(bool),
    List(Vec<Type>),
}

pub enum Callable<'a> {
    Procedure(&'a Procedure),
    Std(&'a StdFunc),
}

pub struct Procedure {
    pub args: Vec<String>,
    pub tokens: Vec<Token>,
}

pub struct StdFunc {
    pub call: fn(&mut Sipwi, Type) -> StdFuncResult,
}

pub struct StdFuncResult {
    pub output: Type,
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
}
