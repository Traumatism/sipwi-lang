use crate::lexing::token::Token;
use crate::parsing::structs::{Function, StdFuncResult};
use crate::sipwi::Sipwi;

pub fn std_for_each(env: &Sipwi, token: Token) -> StdFuncResult {
    if let Token::List(list_content) = token {
        let function = match list_content.get(1).unwrap().get(0) {
            Some(Token::Identifier(function_name)) => env.get_function(function_name),
            _ => panic!(),
        };

        if let Some(Token::List(elements)) = list_content.get(0).unwrap().get(0) {
            elements.iter().for_each(|element| match function {
                Some(Function::Std(function)) => {
                    let _ = &((function.call)(env, Token::List(vec![element.to_owned()])));
                }
                _ => {
                    panic!();
                }
            })
        }
    }

    StdFuncResult::new(Token::List(Vec::new()))
}
