use crate::sipwi::Sipwi;
use crate::structs::FuncResult;
use crate::token::Token;

pub fn std_sum(env: &&mut Sipwi, token: Token) -> Option<FuncResult> {
    let mut sum = 0;

    match token {
        Token::List(lst_content) => lst_content.iter().for_each(|lst| match &lst[0] {
            Token::Number(number) => sum += number,
            _ => panic!(),
        }),
        _ => panic!(),
    }

    Some(FuncResult {
        data: sum.to_string(),
        to_tokens: |data: String| vec![Token::Number(data.parse::<isize>().unwrap())],
    })
}

pub fn std_print(env: &&mut Sipwi, token: Token) -> Option<FuncResult> {
    match token {
        Token::List(lst_content) => lst_content.iter().for_each(|lst| match &lst[0] {
            Token::String(content) => print!("{}", content),
            Token::Number(content) => print!("{}", content),
            Token::Identifier(identifier) => {
                let potential_string = env.variables_strings.get(identifier);

                if potential_string.is_none() {
                    let potential_number = env.variables_numbers.get(identifier);

                    if potential_number.is_none() {
                        panic!()
                    }
                    print!("{}", potential_number.unwrap())
                } else {
                    print!("{}", potential_string.unwrap())
                }
            }
            _ => panic!(),
        }),
        _ => panic!(),
    };

    None
}
