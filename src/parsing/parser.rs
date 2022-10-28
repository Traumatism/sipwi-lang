use core::panic;

use crate::lexing::consts::MAIN_FUNCTION;
use crate::lexing::token::Token;
use crate::parsing::structs::{Func, Function, Variable};
use crate::peeker::Peeker;
use crate::sipwi::Sipwi;

#[allow(dead_code)]
pub struct Parser<'a> {
    tokens_peeker: Peeker<Token>,
    env: &'a mut Sipwi,
    expression: bool,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token>, env: &'a mut Sipwi, expression: bool) -> Self {
        Self {
            expression: expression,
            tokens_peeker: Peeker::new(tokens),
            env,
        }
    }

    pub fn parse_tokens(&mut self) {
        while let Some(token) = self.tokens_peeker.next() {
            match token {
                Token::Expression(expr) => expr.evaluate(self.env),
                Token::Chain => {
                    let first_input = self.tokens_peeker.previous().unwrap();

                    match first_input {
                        Token::List(_) => {}
                        _ => panic!(),
                    }

                    let mut functions = Vec::new();

                    while let Some(next_token) = self.tokens_peeker.next() {
                        if next_token == Token::Chain {
                            if let Some(Token::Identifier(identifier)) = self.tokens_peeker.next() {
                                functions.push(identifier)
                            } else {
                                panic!();
                            }

                            continue;
                        }

                        self.tokens_peeker.cursor -= 1;
                        break;
                    }

                    let mut last_output = std::vec::from_elem(first_input, 1);

                    for (idx, func_name) in functions.iter().enumerate() {
                        // grab the FuncDef

                        let func = self.env.get_function(func_name.as_str());

                        match func {
                            Some(Function::Std(fnc)) => {
                                let new_output = &(fnc.call)(
                                    &self.env,
                                    last_output.clone().get(0).unwrap().clone(),
                                );

                                let new_output_tokens = new_output.get_tokens();

                                match new_output_tokens {
                                    Token::List(lst_content) => {
                                        if lst_content.len() <= 0 && idx != functions.len() - 1 {
                                            panic!()
                                        }
                                    }
                                    _ => panic!(),
                                }

                                last_output = std::vec::from_elem(new_output_tokens.clone(), 1);
                            }
                            Some(Function::NonStd(fnc)) => {
                                Parser::new(fnc.tokens.clone(), self.env, false).parse_tokens();
                            }
                            None => {
                                panic!()
                            }
                        }
                    }
                }
                Token::Identifier(identifier) => {
                    let identifier = identifier;

                    match self.tokens_peeker.next() {
                        // name <- ...--
                        Some(Token::Assignement) => match self.tokens_peeker.next() {
                            // name <- "Hello, World!"
                            Some(Token::String(value)) => {
                                self.env.register_variable(identifier, Variable::Str(value));
                            }
                            // name <- 123
                            Some(Token::Number(value)) => {
                                self.env
                                    .register_variable(identifier, Variable::Number(value));
                            }
                            // name <- fnc
                            Some(Token::Keyword(keyword)) => match keyword.as_str() {
                                "fnc" => {
                                    let mut fnc_tokens: Vec<Token> = Vec::new();
                                    let mut fnc_args = Vec::new();

                                    // get function arguments names
                                    if let Some(Token::List(list)) = self.tokens_peeker.next() {
                                        for element in &list {
                                            // we want a single identifier <=> a single token
                                            if element.len() != 1 {
                                                panic!()
                                            }

                                            if let Some(Token::Identifier(argument_name)) =
                                                element.first()
                                            {
                                                fnc_args.push(argument_name.clone())
                                            } else {
                                                panic!()
                                            }
                                        }
                                    } else {
                                        panic!()
                                    }

                                    // verify "do"
                                    if let Some(Token::Keyword(keyword)) = self.tokens_peeker.next()
                                    {
                                        if keyword != "do" {
                                            panic!()
                                        }
                                    } else {
                                        panic!()
                                    }

                                    // prevent stopping a function after the
                                    // first 'end', even if a 'do' was used inside
                                    // that function
                                    let mut ignore_dos = 0;

                                    loop {
                                        // too much 'end'
                                        if ignore_dos < 0 {
                                            panic!()
                                        }

                                        match self.tokens_peeker.next() {
                                            Some(Token::Keyword(keyword)) => {
                                                if keyword == String::from("do") {
                                                    // 'do' joined the game
                                                    ignore_dos += 1
                                                } else if keyword == String::from("end") {
                                                    // the final 'end'
                                                    if ignore_dos == 0 {
                                                        break;
                                                    }
                                                    // the end of the nearest 'do'
                                                    ignore_dos -= 1
                                                }

                                                // we want the keyword still
                                                fnc_tokens.push(Token::Keyword(keyword.clone()));
                                            }

                                            Some(token) => fnc_tokens.push(token),

                                            // forgot 'end'
                                            None => panic!(),
                                        }
                                    }

                                    if identifier == String::from(MAIN_FUNCTION)
                                        && fnc_args.len() > 0
                                    {
                                        panic!()
                                    }

                                    self.env.register_function(
                                        &identifier,
                                        Func::new(fnc_args, fnc_tokens),
                                    )
                                }
                                _ => {}
                            },
                            _ => {
                                panic!()
                            }
                        },
                        _ => self.tokens_peeker.cursor -= 1,
                    }
                }
                _ => {}
            }
        }
    }
}
