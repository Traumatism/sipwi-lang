use crate::lexing::token::Token;
use crate::parsing::structs::{StdFuncResult, Variable};
use crate::sipwi::Sipwi;

pub fn std_sum(env: &&mut Sipwi, token: Token) -> StdFuncResult {
    let mut sum = 0;

    if let Token::List(list_content) = token {
        list_content.iter().for_each(|list| {
            for element in list {
                match element {
                    Token::Number(number) => sum += number,
                    Token::Identifier(identifier) => {
                        let value = env.get_variable(identifier);
                        match value {
                            Some(Variable::Number(content)) => sum += content,
                            _ => panic!(),
                        }
                    }
                    _ => panic!(),
                }
            }
        })
    }

    StdFuncResult::new(Token::List(std::vec::from_elem(
        std::vec::from_elem(Token::Number(sum), 1),
        1,
    )))
}
