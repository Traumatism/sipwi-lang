use crate::common::sipwi::Sipwi;
use crate::lexing::token::Token;
use crate::parsing::structs::{StdFuncResult, Variable};

pub fn std_range_inclusive(env: &Sipwi, token: Token) -> StdFuncResult {
    let (start, end) = match token.clone() {
        Token::List(list) => {
            if list.len() != 2 {
                panic!()
            }

            let (s, e) = (list.get(0).unwrap(), list.get(1).unwrap());

            if s.len() != 1 || e.len() != 1 {
                panic!()
            }

            let (s, e) = (s.get(0).unwrap(), e.get(0).unwrap());

            let (start, end) = match (s, e) {
                (Token::Number(start_value), Token::Number(end_value)) => {
                    (start_value.to_owned(), end_value.to_owned())
                }
                (Token::Identifier(start_identifier), Token::Identifier(end_identifier)) => {
                    match (
                        env.get_variable(&start_identifier).to_owned(),
                        env.get_variable(&end_identifier).to_owned(),
                    ) {
                        (
                            Some(&Variable::Number(start_value)),
                            Some(&Variable::Number(end_value)),
                        ) => (start_value.to_owned(), end_value.to_owned()),
                        _ => panic!(),
                    }
                }
                (Token::Number(start_value), Token::Identifier(end_identifier)) => {
                    let end_value = match env.get_variable(&end_identifier) {
                        Some(&Variable::Number(end_value)) => end_value,
                        _ => panic!(),
                    };

                    (start_value.to_owned(), end_value)
                }
                (Token::Identifier(start_identifier), Token::Number(end_value)) => {
                    let start_value = match env.get_variable(&start_identifier) {
                        Some(&Variable::Number(start_value)) => start_value,
                        _ => panic!(),
                    };

                    (start_value, end_value.to_owned())
                }
                _ => panic!(),
            };

            (start, end)
        }
        _ => panic!(),
    };

    StdFuncResult::new(Token::List(std::vec::from_elem(
        (start..end).map(|n| Token::Number(n)).collect(),
        1,
    )))
}

pub fn std_range(env: &Sipwi, token: Token) -> StdFuncResult {
    let (start, end) = match token.clone() {
        Token::List(list) => {
            if list.len() != 2 {
                panic!()
            }

            let (s, e) = (list.get(0).unwrap(), list.get(1).unwrap());

            if s.len() != 1 || e.len() != 1 {
                panic!()
            }

            let (s, e) = (s.get(0).unwrap(), e.get(0).unwrap());

            let (start, end) = match (s, e) {
                (Token::Number(start_value), Token::Number(end_value)) => {
                    (start_value.to_owned(), end_value.to_owned())
                }
                (Token::Identifier(start_identifier), Token::Identifier(end_identifier)) => {
                    match (
                        env.get_variable(&start_identifier).to_owned(),
                        env.get_variable(&end_identifier).to_owned(),
                    ) {
                        (
                            Some(&Variable::Number(start_value)),
                            Some(&Variable::Number(end_value)),
                        ) => (start_value.to_owned(), end_value.to_owned()),
                        _ => panic!(),
                    }
                }
                (Token::Number(start_value), Token::Identifier(end_identifier)) => {
                    let end_value = match env.get_variable(&end_identifier) {
                        Some(&Variable::Number(end_value)) => end_value,
                        _ => panic!(),
                    };

                    (start_value.to_owned(), end_value)
                }
                (Token::Identifier(start_identifier), Token::Number(end_value)) => {
                    let start_value = match env.get_variable(&start_identifier) {
                        Some(&Variable::Number(start_value)) => start_value,
                        _ => panic!(),
                    };

                    (start_value, end_value.to_owned())
                }
                _ => panic!(),
            };

            (start, end)
        }
        _ => panic!(),
    };

    StdFuncResult::new(Token::List(std::vec::from_elem(
        (start..=end).map(|n| Token::Number(n)).collect(),
        1,
    )))
}
