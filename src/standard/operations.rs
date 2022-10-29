use crate::lexing::token::Token;
use crate::parsing::structs::{StdFuncResult, Variable};
use crate::sipwi::Sipwi;

pub fn std_gauss_sum(env: &Sipwi, token: Token) -> StdFuncResult {
    let res = match token {
        Token::Number(number) => (number * (number + 1)) / 2,
        // Print a variable
        Token::Identifier(identifier) => {
            let value = env.get_variable(&identifier);

            if let Some(Variable::Number(number)) = value {
                (number * (number + 1)) / 2
            } else {
                panic!()
            }
        }
        _ => panic!(),
    };

    StdFuncResult::new(Token::Number(res))
}

pub fn std_sum(env: &Sipwi, token: Token) -> StdFuncResult {
    let mut total = 0;

    match token {
        Token::List(list) => {
            for sub_list in list {
                for element in sub_list {
                    match element {
                        Token::Number(number) => total += number,
                        Token::Identifier(identifier) => {
                            let value = env.get_variable(&identifier);
                            match value {
                                Some(Variable::Number(number)) => {
                                    total += number;
                                }
                                Some(variable_type) => panic!("Cannot add a {:?}", variable_type),
                                _ => panic!("{} is not defined", identifier),
                            };
                        }
                        token => panic!("Cannot add a {:?}", token),
                    }
                }
            }
        }
        _ => {
            panic!("'sum' expect a list of integers/identifiers as arguments!")
        }
    }

    StdFuncResult::new(Token::Number(total))
}
