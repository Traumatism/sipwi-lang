use std::io::Write;

use crate::sipwi::Sipwi;
use crate::structs::{StdFuncResult, Variable};
use crate::token::Token;

pub fn std_range_inclusive(env: &&mut Sipwi, token: Token) -> Option<StdFuncResult> {
    let mut start_end: Vec<isize> = Vec::new();

    if let Token::List(lst_content) = token {
        if lst_content.len() != 2 {
            panic!("(irange) expected two arguments")
        }

        for lst in lst_content {
            if lst.len() != 1 {
                panic!("(irange) expected one number or identifier for each element")
            }

            for element in lst {
                match element {
                    Token::Number(n) => start_end.push(n),
                    Token::Identifier(identifier) => match env.get_variable(&identifier).clone() {
                        Some(&Variable::Number(value)) => start_end.push(value),
                        _ => panic!(),
                    },
                    _ => panic!("(irange) expected one number or identifier for each element"),
                }
            }
        }
    }

    let numbers = vec![(start_end[0].clone()..=start_end[1].clone())
        .map(|n| Token::Number(n))
        .collect()];

    Some(StdFuncResult::new(Token::List(numbers)))
}

pub fn std_range(env: &&mut Sipwi, token: Token) -> Option<StdFuncResult> {
    let mut start_end: Vec<isize> = Vec::new();

    if let Token::List(lst_content) = token {
        if lst_content.len() != 2 {
            panic!("(range) expected two arguments")
        }

        for lst in lst_content {
            if lst.len() != 1 {
                panic!("(range) expected one number or identifier for each element")
            }
            for element in lst {
                match element {
                    Token::Number(n) => start_end.push(n),
                    Token::Identifier(identifier) => match env.get_variable(&identifier).clone() {
                        Some(&Variable::Number(value)) => start_end.push(value),
                        _ => panic!(),
                    },
                    _ => panic!("(irange) expected one number or identifier for each element"),
                }
            }
        }
    }

    let numbers = vec![(start_end[0].clone()..start_end[1].clone())
        .map(|n| Token::Number(n))
        .collect()];

    Some(StdFuncResult::new(Token::List(numbers)))
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
                    token => panic!("(sum): can't perform sum with token: {:?}", token),
                }
            }
        })
    }

    let sum_as_token = Token::Number(sum);
    let func_result = StdFuncResult::new(Token::List(vec![vec![sum_as_token]]));

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
