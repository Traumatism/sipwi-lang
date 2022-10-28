use crate::lexing::token::Token;
use crate::parsing::structs::{StdFuncResult, Variable};
use crate::sipwi::Sipwi;

use fastrand::Rng;

pub fn std_randint(env: &&mut Sipwi, token: Token) -> StdFuncResult {
    let mut start_end: Vec<isize> = Vec::new();

    if let Token::List(lst_content) = token {
        if lst_content.len() != 2 {
            panic!()
        }

        for lst in lst_content {
            if lst.len() != 1 {
                panic!()
            }

            for element in lst {
                match element {
                    Token::Number(n) => start_end.push(n),
                    Token::Identifier(identifier) => match env.get_variable(&identifier).clone() {
                        Some(&Variable::Number(value)) => start_end.push(value),
                        _ => panic!(),
                    },
                    _ => panic!(),
                }
            }
        }
    }

    StdFuncResult::new(Token::List(std::vec::from_elem(
        std::vec::from_elem(
            Token::Number(Rng::new().isize(start_end[0].clone()..start_end[1].clone())),
            1,
        ),
        1,
    )))
}
