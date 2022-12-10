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

pub type Procedure = (Vec<String>, Vec<Token>);

pub type StdFuncResult = Type;

pub type StdFunc = fn(&mut Sipwi, Type) -> StdFuncResult;
