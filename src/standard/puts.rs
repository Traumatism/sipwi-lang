use crate::lexing::token::Token;
use crate::parsing::structs::{StdFuncResult, Variable};
use crate::sipwi::Sipwi;

use std::io::Write;

pub fn std_puts(env: &&mut Sipwi, token: Token) -> StdFuncResult {
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

    StdFuncResult::new(Token::List(Vec::new()))
}
