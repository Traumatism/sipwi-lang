use crate::common::sipwi::Sipwi;
use crate::lexing::token::Token;
use crate::parsing::structs::{StdFuncResult, Type};
use fastrand::Rng;

pub fn _std_shuffle(_env: &Sipwi, _token: Token) -> StdFuncResult {
    todo!()
}

pub fn _std_randchoice(_env: &Sipwi, _token: Token) -> StdFuncResult {
    todo!()
}

pub fn std_randint(env: &Sipwi, token: Token) -> StdFuncResult {
    let mut start_end: Vec<isize> = Vec::new();

    if let Token::List(list_content) = token {
        if list_content.len() != 2 {
            panic!()
        }

        for sub_list in list_content {
            if sub_list.len() != 1 {
                panic!()
            }

            for element in sub_list {
                match element {
                    Token::Number(n) => start_end.push(n),
                    Token::Identifier(identifier) => match env.get_variable(&identifier).to_owned()
                    {
                        Type::Number(value) => start_end.push(value),
                        _ => panic!(),
                    },
                    _ => panic!(),
                }
            }
        }
    }

    let random_number = Rng::new().isize(start_end[0].to_owned()..start_end[1].to_owned());

    StdFuncResult::new(Token::Number(random_number))
}
