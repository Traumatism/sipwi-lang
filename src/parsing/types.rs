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
    Procedure(&'a (Vec<String>, Vec<Token>)),
    Std(&'a fn(&mut Sipwi, Type) -> Type),
}
