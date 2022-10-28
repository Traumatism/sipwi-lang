use crate::lexing::token::Token;
use crate::parsing::structs::StdFuncResult;
use crate::sipwi::Sipwi;

pub fn std_immutable(env: &&mut Sipwi, token: Token) -> StdFuncResult {
    if let Token::List(lst_content) = token {
        for lst in lst_content {
            if lst.len() != 1 {
                panic!()
            }

            match lst.get(0).unwrap() {
                Token::Identifier(identifier) => {}
                _ => panic!(),
            }
        }
    }

    StdFuncResult::new(Token::List(Vec::new()))
}
