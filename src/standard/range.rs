use crate::sipwi::Sipwi;
use crate::structs::{StdFuncResult, Variable};
use crate::token::Token;

pub fn std_range_inclusive(env: &&mut Sipwi, token: Token) -> StdFuncResult {
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

    let numbers = std::vec::from_elem(
        (start_end[0].clone()..=start_end[1].clone())
            .map(|n| Token::Number(n))
            .collect(),
        1,
    );

    StdFuncResult::new(Token::List(numbers))
}

pub fn std_range(env: &&mut Sipwi, token: Token) -> StdFuncResult {
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
        (start_end[0].clone()..start_end[1].clone())
            .map(|n| Token::Number(n))
            .collect(),
        1,
    )))
}
