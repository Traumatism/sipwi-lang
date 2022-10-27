use crate::sipwi::Sipwi;
use crate::structs::StdFuncResult;
use crate::token::Token;

pub fn _std_concat(_env: &&mut Sipwi, _token: Token) -> Option<StdFuncResult> {
    None
}

pub fn std_sum(env: &&mut Sipwi, token: Token) -> Option<StdFuncResult> {
    let mut sum = 0;

    if let Token::List(lst_content) = token {
        lst_content.iter().for_each(|lst| match &lst[0] {
            Token::Number(number) => sum += number,
            Token::Identifier(identifier) => {
                sum += env
                    .variables_numbers
                    .get(identifier.as_str())
                    .expect(&format!("(sum): undefined identifier: {}", identifier))
            }
            token => panic!("(sum): can't perform sum with token: {:?}", token),
        })
    }

    let sum_as_token = Token::Number(sum);
    let func_result = StdFuncResult::new(Token::List(vec![vec![sum_as_token]]));

    Some(func_result)
}

pub fn std_print(env: &&mut Sipwi, token: Token) -> Option<StdFuncResult> {
    if let Token::List(lst_content) = token {
        lst_content.iter().for_each(|lst| match &lst[0] {
            Token::String(content) => print!("{}", content),
            Token::Number(content) => print!("{}", content),
            Token::Identifier(identifier) => {
                let potential_string = env.variables_strings.get(identifier.as_str());

                if potential_string.is_none() {
                    let potential_number = env
                        .variables_numbers
                        .get(identifier.as_str())
                        .expect(&format!("(print): undefined identifier: {}", identifier));

                    print!("{}", potential_number)
                } else {
                    print!("{}", potential_string.unwrap())
                }
            }
            _ => panic!(),
        })
    }

    None
}
