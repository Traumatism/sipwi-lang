use crate::{
    common::{peeker::Peeker, sipwi::Sipwi},
    lexing::{consts, token::Token},
};

use super::types::{Callable, Type};

pub struct Parser<'a> {
    tokens: Peeker<Token>,
    env: &'a mut Sipwi,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token>, env: &'a mut Sipwi) -> Self {
        Self {
            tokens: Peeker::new(tokens),
            env,
        }
    }

    fn parse_next_codeblock(&mut self) -> Vec<Token> {
        match self.tokens.next() {
            Some(Token::Keyword(keyword)) => {
                if keyword != consts::BLOCK_OPENING_MARK {
                    panic!()
                }
            }
            _ => panic!(),
        }

        let mut n = 0;
        let mut tokens = Vec::new();

        loop {
            let next_token = self.tokens.next();

            match next_token {
                Some(Token::Keyword(keyword)) => {
                    if keyword == consts::BLOCK_OPENING_MARK {
                        tokens.push(Token::Keyword(keyword));
                        n += 1;
                    } else if keyword == consts::BLOCK_CLOSING_MARK && n == 0 {
                        break;
                    } else if keyword == consts::BLOCK_CLOSING_MARK {
                        tokens.push(Token::Keyword(keyword));
                        n -= 1;
                    } else {
                        tokens.push(Token::Keyword(keyword));
                    }
                }
                Some(token) => tokens.push(token),
                _ => panic!(),
            }
        }

        tokens
    }

    /// Parse a procedure definition (proc [...] do ... end)
    fn parse_proc(&mut self, identifier: String) {
        let proc_arguments = match self.tokens.next() {
            Some(Token::List(arguments)) => arguments,
            _ => panic!(),
        }
        .iter()
        .map(|token| match token {
            Token::Identifier(argument) => argument.to_owned(),
            _ => panic!(),
        })
        .collect();

        let tokens = self.parse_next_codeblock();

        self.env
            .procedures
            .insert(identifier, (proc_arguments, tokens));
    }

    /// Parse an assignement (a <- ...)
    fn parse_assignement(&mut self, identifier: String) {
        match self.tokens.next() {
            Some(Token::String(string)) => {
                self.env.register_variable(identifier, Type::Str(string))
            }
            Some(Token::List(lst)) => {
                let lst_type = self.token_to_type(Token::List(lst));

                self.env.register_variable(identifier, lst_type);
            }
            Some(Token::Number(number)) => {
                self.env.register_variable(identifier, Type::Number(number))
            }
            Some(Token::Keyword(keyword)) => {
                if keyword == consts::PROC_DEF_MARK {
                    self.parse_proc(identifier)
                } else {
                    panic!()
                }
            }
            Some(Token::Expression(tokens)) => {
                let output = Parser::new(tokens, self.env).parse_tokens().unwrap();
                self.env.register_variable(identifier, output);
            }
            _ => panic!(),
        }
    }

    /// Convert a token into a type
    fn token_to_type(&mut self, token: Token) -> Type {
        match token {
            Token::String(string) => Type::Str(string),
            Token::Number(number) => Type::Number(number),
            Token::Identifier(identifier) => {
                self.env
                    .variables
                    .get(&identifier)
                    .unwrap_or_else(|| panic!())
            }
            .to_owned(),
            Token::Expression(tokens) => Parser::new(tokens, self.env).parse_tokens().unwrap(),
            Token::List(tokens) => Type::List(
                tokens
                    .iter()
                    .map(|token| self.token_to_type(token.to_owned()))
                    .collect(),
            ),
            _ => panic!(),
        }
    }

    /// Convert a type into a token
    pub fn type_to_token(tpe: Type) -> Token {
        match tpe {
            Type::Str(string) => Token::String(string),
            Type::Number(number) => Token::Number(number),
            Type::List(tpes) => Token::List(
                tpes.iter()
                    .map(|tpe| Parser::type_to_token(tpe.to_owned()))
                    .collect(),
            ),
            _ => panic!(),
        }
    }

    pub fn parse_tokens(&mut self) -> Option<Type> {
        let mut last_output = None;

        while let Some(token) = self.tokens.next() {
            match token {
                Token::Expression(tokens) => {
                    last_output = Parser::new(tokens, self.env).parse_tokens()
                }

                Token::Keyword(keyword) => {
                    if keyword.as_str() == consts::FOR_LOOP_MARK {
                        let name = match self.tokens.next() {
                            Some(Token::Identifier(identifier)) => identifier,
                            _ => panic!(),
                        };

                        let Some(Token::Assignement) = self.tokens.next() else {
                            panic!()
                        };

                        let next = self.tokens.next().unwrap();

                        let elements = match self.token_to_type(next) {
                            Type::List(elements) => elements,
                            _ => panic!(),
                        };

                        let tokens = self.parse_next_codeblock();

                        self.env
                            .procedures
                            .insert(String::from("for_proc"), (vec![name], tokens));

                        for element in elements {
                            Parser::new(
                                vec![
                                    Parser::type_to_token(element),
                                    Token::Chain,
                                    Token::Identifier(String::from("for_proc")),
                                ],
                                self.env,
                            )
                            .parse_tokens();
                        }

                        self.env.procedures.remove("for_proc");
                    }
                }

                Token::Assignement => {
                    let identifier = match self.tokens.previous().unwrap() {
                        Token::Identifier(identifier) => identifier,
                        _ => panic!(),
                    };

                    self.tokens.next();
                    self.parse_assignement(identifier)
                }

                Token::Chain => {
                    let previous_token = self.tokens.previous().unwrap();
                    let first_input = self.token_to_type(previous_token);
                    let mut chain = Vec::new();

                    last_output = Some(first_input);

                    while let Some(next_token) = self.tokens.next() {
                        if next_token == Token::Chain {
                            if let Some(Token::Identifier(identifier)) = self.tokens.next() {
                                chain.push(identifier)
                            } else {
                                panic!();
                            }
                            continue;
                        }

                        self.tokens.previous();
                        break;
                    }

                    chain.iter().for_each(|identifier| {
                        match {
                            if let Some(fnc) = &self.env.std_functions.get(identifier) {
                                Callable::Std(fnc)
                            } else if let Some(fnc) = &self.env.procedures.get(identifier) {
                                Callable::Procedure(fnc)
                            } else {
                                panic!()
                            }
                        } {
                            Callable::Procedure(procedure) => {
                                let mut proc_tokens = Vec::new();

                                let args = match &last_output {
                                    Some(Type::List(content)) => content.to_owned(),
                                    Some(tpe) => {
                                        vec![tpe.to_owned()]
                                    }
                                    _ => panic!(),
                                };

                                procedure.0.iter().enumerate().for_each(|(idx, arg)| {
                                    /* arg_name = value */
                                    proc_tokens.append(&mut vec![
                                        Token::Identifier(arg.to_owned()),
                                        Token::Assignement,
                                        Parser::type_to_token(args.get(idx).unwrap().to_owned()),
                                    ])
                                });

                                proc_tokens.append(&mut procedure.1.clone());

                                last_output = Parser::new(proc_tokens, self.env).parse_tokens();
                            }
                            Callable::Std(function) => {
                                last_output =
                                    Some((function)(self.env, last_output.clone().unwrap()))
                            }
                        }
                    })
                }
                _ => {}
            };
        }

        last_output
    }
}
