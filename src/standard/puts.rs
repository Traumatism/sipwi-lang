use crate::lexing::token::Token;
use crate::parsing::structs::{StdFuncResult, Variable};
use crate::sipwi::Sipwi;

use std::io::Write;

// Write content to stdout (no nl)
fn write_to_stdout(content: &str) {
    let _ = std::io::stdout().write(content.as_bytes());
}

pub fn std_puts(env: &Sipwi, token: Token) -> StdFuncResult {
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
                            Some(Variable::Str(content)) => write_to_stdout(content),
                            Some(Variable::Number(content)) => {
                                let _ = write_to_stdout(content.to_string().as_str());
                            }
                            _ => panic!(),
                        }
                    }
                    _ => panic!(),
                }
            }
        });
    }

    StdFuncResult::new(Token::List(Vec::new()))
}
