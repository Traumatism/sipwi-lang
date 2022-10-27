use std::io::Write;

use crate::sipwi::Sipwi;
use crate::structs::{StdFuncResult, Variable};
use crate::token::Token;

pub fn std_range_inclusive(env: &&mut Sipwi, token: Token) -> Option<StdFuncResult> {
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

    Some(StdFuncResult::new(Token::List(numbers)))
}

pub fn std_range(env: &&mut Sipwi, token: Token) -> Option<StdFuncResult> {
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

    Some(StdFuncResult::new(Token::List(std::vec::from_elem(
        (start_end[0].clone()..start_end[1].clone())
            .map(|n| Token::Number(n))
            .collect(),
        1,
    ))))
}

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

pub fn std_print(env: &&mut Sipwi, token: Token) -> Option<StdFuncResult> {
    if let Token::List(lst_content) = token {
        lst_content.iter().for_each(|lst| {
            for element in lst {
                match element {
                    Token::String(content) => {
                        let _ = std::io::stdout().write(content.as_bytes());
                    }
                    Token::Number(content) => {
                        let _ = std::io::stdout().write(content.to_string().as_bytes());
                    }
                    Token::Identifier(identifier) => {
                        let value = env.get_variable(identifier);
                        match value {
                            Some(Variable::Str(content)) => {
                                let _ = std::io::stdout().write(content.to_string().as_bytes());
                            }
                            Some(Variable::Number(content)) => {
                                let _ = std::io::stdout().write(content.to_string().as_bytes());
                            }
                            _ => panic!(),
                        }
                    }
                    _ => panic!(),
                }
            }
        });
    }

    None
}
