use crate::sipwi::Sipwi;
use crate::structs::StdFuncResult;
use crate::token::Token;

pub fn std_range_inclusive(env: &&mut Sipwi, token: Token) -> Option<StdFuncResult> {
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
                    Token::Identifier(identifier) => start_end.push(
                        env.variables_numbers
                            .get(identifier.as_str())
                            .expect(&format!("(range): undefined identifier: {}", identifier))
                            .clone(),
                    ),
                    _ => panic!(),
                }
            }
        }
    }

    let numbers = (start_end[0].clone()..=start_end[1].clone())
        .map(|n| Token::Number(n))
        .collect();

    Some(StdFuncResult::new(Token::List(vec![numbers])))
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
                    Token::Identifier(identifier) => start_end.push(
                        env.variables_numbers
                            .get(identifier.as_str())
                            .expect(&format!("(range): undefined identifier: {}", identifier))
                            .clone(),
                    ),
                    _ => panic!(),
                }
            }
        }
    }

    let numbers = (start_end[0].clone()..start_end[1].clone())
        .map(|n| Token::Number(n))
        .collect();

    Some(StdFuncResult::new(Token::List(vec![numbers])))
}

pub fn std_sum(env: &&mut Sipwi, token: Token) -> Option<StdFuncResult> {
    let mut sum = 0;

    if let Token::List(lst_content) = token {
        lst_content.iter().for_each(|lst| {
            for element in lst {
                match element {
                    Token::Number(number) => sum += number,
                    Token::Identifier(identifier) => {
                        sum += env
                            .variables_numbers
                            .get(identifier.as_str())
                            .expect(&format!("(sum): undefined identifier: {}", identifier))
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
                    Token::String(content) => print!("{}", content),
                    Token::Number(content) => print!("{}", content),
                    Token::Identifier(identifier) => {
                        let potential_string = env.variables_strings.get(identifier.as_str());

                        if potential_string.is_none() {
                            let potential_number = env
                                .variables_numbers
                                .get(identifier.as_str())
                                .expect(&format!("(puts): undefined identifier: {}", identifier));

                            print!("{}", potential_number)
                        } else {
                            print!("{}", potential_string.unwrap())
                        }
                    }
                    _ => panic!(),
                }
            }
        });
    }

    None
}
