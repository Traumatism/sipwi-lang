use crate::lexing::token::Token;
use crate::parsing::structs::{StdFuncResult, Variable};
use crate::sipwi::Sipwi;

pub fn std_sum(env: &Sipwi, token: Token) -> StdFuncResult {
    let mut sum = 0;

    match token {
        Token::List(list) => {
            for sub_list in list {
                for element in sub_list {
                    match element {
                        Token::Number(number) => sum += number,
                        Token::Identifier(identifier) => {
                            let value = env.get_variable(&identifier);
                            match value {
                                Some(Variable::Number(number)) => {
                                    sum += number;
                                }
                                _ => panic!(),
                            };
                        }
                        _ => panic!(),
                    }
                }
            }
        }
        _ => {}
    }

    StdFuncResult::new(Token::Number(sum))
}
