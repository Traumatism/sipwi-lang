use crate::{
    common::{peeker::Peeker, sipwi::Sipwi},
    lexing::token::Token,
};

use super::structs::{Callable, Procedure, Type};

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

    fn parse_proc(&mut self, identifier: String) {
        let mut proc_arguments = Vec::new();
        let mut proc_tokens = Vec::new();

        match self.tokens.next() {
            Some(Token::List(arguments)) => arguments,
            _ => panic!(),
        }
        .iter()
        .for_each(|token| match token {
            Token::Identifier(argument) => proc_arguments.push(argument.to_owned()),
            _ => panic!(),
        });

        match self.tokens.next() {
            Some(Token::Keyword(keyword)) => {
                if keyword != String::from("do") {
                    panic!()
                }
            }
            _ => panic!(),
        }

        let mut n = 0;

        loop {
            match self.tokens.next() {
                Some(token) => match &token {
                    Token::Keyword(keyword) => {
                        if keyword == &String::from("end") && n == 0 {
                            break;
                        }

                        if keyword == &String::from("do") {
                            n += 1;
                            continue;
                        }

                        if keyword == &String::from("end") {
                            n -= 1;
                            continue;
                        }

                        proc_tokens.push(token)
                    }
                    _ => proc_tokens.push(token),
                },
                _ => panic!(),
            }
        }

        let procedure = Procedure::new(proc_arguments, proc_tokens);

        self.env.register_procedure(&identifier, procedure)
    }

    fn parse_assignement(&mut self, identifier: String) {
        match self.tokens.next() {
            Some(Token::String(string)) => {
                self.env.register_variable(&identifier, Type::Str(string))
            }
            Some(Token::List(lst)) => {
                let lst_type = self.token_to_type(Token::List(lst));

                self.env.register_variable(&identifier, lst_type);
            }
            Some(Token::Number(number)) => self
                .env
                .register_variable(&identifier, Type::Number(number)),
            Some(Token::Keyword(keyword)) => match keyword.as_str() {
                "proc" => self.parse_proc(identifier),
                _ => panic!(),
            },
            Some(Token::Expression(expression)) => {
                let output = self.parse_expression(Token::Expression(expression));
                self.env.register_variable(&identifier, output.unwrap());
            }
            _ => panic!(),
        }
    }

    fn parse_expression(&mut self, expression: Token) -> Option<Type> {
        match expression {
            Token::Expression(tokens) => Parser::new(tokens, self.env).parse_tokens(None),
            _ => panic!(),
        }
    }

    fn token_to_type(&mut self, token: Token) -> Type {
        match token {
            Token::String(string) => Type::Str(string),
            Token::Number(number) => Type::Number(number),
            Token::Identifier(identifier) => self.env.get_variable(&identifier).to_owned(),
            Token::Expression(tokens) => self.parse_expression(Token::Expression(tokens)).unwrap(),
            Token::List(tokens) => Type::List(
                tokens
                    .iter()
                    .map(|token| self.token_to_type(token.to_owned()))
                    .collect(),
            ),
            _ => panic!("Can't convert `{:?}`", token),
        }
    }

    fn type_to_token(&self, tpe: Type) -> Token {
        match tpe {
            Type::Str(string) => Token::String(string),
            Type::Number(number) => Token::Number(number),
            Type::List(tpes) => Token::List(
                tpes.iter()
                    .map(|tpe| self.type_to_token(tpe.to_owned()))
                    .collect(),
            ),
            _ => panic!(),
        }
    }

    pub fn parse_tokens(&mut self, from: Option<String>) -> Option<Type> {
        let mut last_output = None;

        while let Some(token) = self.tokens.next() {
            match token {
                Token::Import(_) => match &from {
                    Some(name) => {
                        if name != &String::from("import") {
                            panic!("`import` procedure must only contains imports (@\"path.spw\"")
                        }
                    }
                    _ => panic!("imports must be inside of the `import` procedure"),
                },

                Token::Expression(_) => last_output = self.parse_expression(token),

                Token::Assignement => {
                    let identifier = match self.tokens.previous().unwrap() {
                        Token::Identifier(identifier) => identifier,
                        _ => panic!(),
                    };

                    self.tokens.next();
                    self.parse_assignement(String::from(identifier))
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

                    chain
                        .iter()
                        .for_each(|identifier| match self.env.get_callable(identifier) {
                            Callable::Procedure(procedure) => {
                                let mut proc_tokens = Vec::new();

                                match &last_output {
                                    Some(Type::List(content)) => {
                                        procedure.args.iter().enumerate().for_each(|(idx, arg)| {
                                            proc_tokens.append(&mut vec![
                                                Token::Identifier(arg.to_owned()),
                                                Token::Assignement,
                                                self.type_to_token(
                                                    content.get(idx).unwrap().to_owned(),
                                                ),
                                            ])
                                        })
                                    }
                                    _ => panic!(),
                                }

                                proc_tokens.append(&mut procedure.tokens.clone());

                                last_output = Some(
                                    Parser::new(proc_tokens, self.env)
                                        .parse_tokens(None)
                                        .unwrap(),
                                );
                            }
                            Callable::Std(function) => {
                                last_output = Some(
                                    (function.call)(self.env, last_output.clone().unwrap())
                                        .get_output()
                                        .to_owned(),
                                )
                            }
                        })
                }
                _ => {}
            };
        }

        last_output
    }
}
