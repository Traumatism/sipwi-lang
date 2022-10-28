use crate::sipwi::Sipwi;
use crate::structs::{Function, StdFuncResult};
use crate::token::Token;

pub fn std_for_each(env: &&mut Sipwi, token: Token) -> StdFuncResult {
    if let Token::List(lst_content) = token {
        let fnc = match lst_content.get(1).unwrap().get(0) {
            Some(Token::Identifier(fnc)) => env.get_function(fnc),
            _ => panic!(),
        };

        if let Some(Token::List(elements)) = lst_content.get(0).unwrap().get(0) {
            elements.iter().for_each(|element| match fnc {
                Some(Function::Std(fnc)) => {
                    let _ = &((fnc.call)(&env, Token::List(vec![element.clone()])));
                }
                _ => {
                    panic!();
                }
            })
        }
    }

    StdFuncResult::new(Token::List(Vec::new()))
}
