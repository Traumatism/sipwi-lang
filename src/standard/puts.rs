use crate::lexing::token::Token;
use crate::parsing::structs::{StdFuncResult, Variable};
use crate::sipwi::Sipwi;

use std::io::Write;

// Write content to stdout (no nl)
fn write_to_stdout(content: &str) {
    let _ = std::io::stdout().write(content.as_bytes());
}

pub fn std_puts(env: &Sipwi, token: Token) -> StdFuncResult {
    match token {
        Token::Number(number) => write_to_stdout(&number.to_string()),
        Token::String(string) => write_to_stdout(&string),
        Token::Identifier(identifier) => {
            let value = env.get_variable(&identifier);

            if let Some(Variable::Number(number)) = value {
                write_to_stdout(&number.to_string())
            } else if let Some(Variable::Str(string)) = value {
                write_to_stdout(&string)
            } else {
                panic!("Cannot print a {:?}", value)
            }
        }
        Token::List(list) => {
            for sub_list in list {
                for element in sub_list {
                    std_puts(env, element);
                }
            }
        }
        token => {
            panic!("Cannot print a {:?}", token)
        }
    }

    StdFuncResult::empty()
}
