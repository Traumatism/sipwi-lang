use crate::sipwi::Sipwi;
use crate::structs::{StdFuncResult, Variable};
use crate::token::Token;

pub fn std_sum(env: &&mut Sipwi, token: Token) -> Option<StdFuncResult> {
    let mut sum = 0;

    if let Token::List(lst_content) = token {
        lst_content.iter().for_each(|lst| {
            for element in lst {
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

    let func_result = StdFuncResult::new(Token::List(std::vec::from_elem(
        std::vec::from_elem(Token::Number(sum), 1),
        1,
    )));

    Some(func_result)
}
