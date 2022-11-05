use crate::common::peeker::Peeker;
use crate::common::sipwi::Sipwi;
use crate::lexing::consts::MAIN_FUNCTION;
use crate::lexing::token::Token;
use crate::parsing::structs::{Callable, Procedure, Type};

/// Parse tokens <=> run the program
pub struct Parser<'a> {
    tokens_peeker: Peeker<Token>,
    env: &'a mut Sipwi,
    expression: bool,
    function: Option<String>,
}

impl<'a> Parser<'a> {
    pub fn new(
        tokens: Vec<Token>,
        env: &'a mut Sipwi,
        expression: bool,
        function: Option<String>,
    ) -> Self {
        Self {
            expression,
            tokens_peeker: Peeker::new(tokens),
            env,
            function,
        }
    }

    pub fn parse_tokens(&mut self) -> Option<Token> {
        let mut last_output = Vec::new();

        while let Some(token) = self.tokens_peeker.next() {
            match token {
                Token::Import(_) => match &self.function {
                    Some(name) => {
                        if name != &String::from("import") {
                            panic!("`import` procedure must only contains imports (@\"path.spw\"")
                        }
                    }
                    _ => panic!("imports must be inside of the `import` procedure"),
                },

                Token::Expression(tokens) => {
                    return Parser::new(tokens, self.env, true, self.function.clone())
                        .parse_tokens()
                }

                Token::Chain => {
                    let mut functions = Vec::new();

                    let first_input = match self.tokens_peeker.previous().unwrap() {
                        Token::Expression(expression) => {
                            Parser::new(expression, self.env, true, self.function.clone())
                                .parse_tokens()
                                .unwrap()
                        }
                        other => other,
                    };

                    while let Some(next_token) = self.tokens_peeker.next() {
                        if next_token == Token::Chain {
                            if let Some(Token::Identifier(identifier)) = self.tokens_peeker.next() {
                                functions.push(identifier)
                            } else {
                                panic!();
                            }
                            continue;
                        }

                        self.tokens_peeker.previous();
                        break;
                    }

                    last_output = std::vec::from_elem(first_input, 1);

                    functions.iter().enumerate().for_each(|(idx, func_name)| {
                        match self.env.get_callable(&func_name) {
                            // Calling a Rust function
                            Callable::Std(func) => {
                                let new_output = (func.call)(
                                    self.env,
                                    last_output.to_owned().get(0).unwrap().to_owned(),
                                );

                                let new_output_tokens = new_output.get_tokens();

                                match new_output_tokens {
                                    Token::List(list_content) => {
                                        if list_content.len() <= 0 && idx != functions.len() - 1 {
                                            panic!()
                                        }
                                    }
                                    _ => {}
                                }

                                last_output = std::vec::from_elem(new_output_tokens.to_owned(), 1);
                            }
                            // Calling a Sipwi procedure
                            Callable::Procedure(func) => {
                                if let Some(Token::List(args_list)) = last_output.get(0) {
                                    let mut base = Vec::new();

                                    func.args.iter().enumerate().for_each(|(idx, arg)| {
                                        base.append(&mut vec![
                                            Token::Identifier(arg.to_owned()),
                                            Token::Assignement,
                                            args_list
                                                .get(idx)
                                                .expect(&format!(
                                                    "`{}` expected more arguments!",
                                                    func_name
                                                ))
                                                .to_owned(),
                                        ]);
                                    });

                                    base.append(&mut func.tokens.to_owned());

                                    Parser::new(base, self.env, false, self.function.clone())
                                        .parse_tokens();
                                } else {
                                    panic!()
                                }
                            }
                        }
                    });
                }

                Token::Identifier(identifier) => {
                    match self.tokens_peeker.next() {
                        // name <- ...
                        Some(Token::Assignement) => {
                            match self.tokens_peeker.next() {
                                Some(Token::Identifier(identifier_bis)) => {
                                    let value =
                                        self.env.get_variable(identifier_bis.as_str()).clone();

                                    self.env.register_variable(identifier.as_str(), value);
                                }
                                // name <- { ... }
                                Some(Token::Expression(tokens)) => {
                                    let expression_output =
                                        Parser::new(tokens, self.env, true, self.function.clone())
                                            .parse_tokens();

                                    match expression_output.unwrap() {
                                        Token::String(value) => {
                                            self.env
                                                .register_variable(&identifier, Type::Str(value));
                                        }
                                        Token::Number(value) => {
                                            self.env.register_variable(
                                                &identifier,
                                                Type::Number(value),
                                            );
                                        }
                                        _ => panic!(),
                                    }
                                }
                                // name <- "Hello, World!"
                                Some(Token::String(value)) => {
                                    self.env.register_variable(&identifier, Type::Str(value));
                                }
                                // name <- 123
                                Some(Token::Number(value)) => {
                                    self.env.register_variable(&identifier, Type::Number(value));
                                }
                                // name <- proc
                                Some(Token::Keyword(keyword)) => match keyword.as_str() {
                                    "proc" => {
                                        let mut fnc_tokens: Vec<Token> = Vec::new();
                                        let mut fnc_args = Vec::new();

                                        // get function arguments names
                                        if let Some(Token::List(list)) = self.tokens_peeker.next() {
                                            list.iter().for_each(|element| {
                                                // we want a single identifier <=> a single token
                                                if let Token::Identifier(argument_name) = element {
                                                    fnc_args.push(argument_name.to_owned())
                                                } else {
                                                    panic!()
                                                }
                                            });
                                        }

                                        // verify "do"
                                        if let Some(Token::Keyword(keyword)) =
                                            self.tokens_peeker.next()
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
                                                    fnc_tokens
                                                        .push(Token::Keyword(keyword.to_owned()));
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

                                        self.env.register_procedure(
                                            &identifier,
                                            Procedure::new(fnc_args, fnc_tokens),
                                        );
                                    }
                                    _ => {
                                        panic!(
                                            "Error! did you mean: {} <- proc [...] do ... end",
                                            identifier
                                        )
                                    }
                                },
                                _ => {}
                            }
                        }
                        _ => {
                            self.tokens_peeker.previous();
                        }
                    }
                }
                _ => {}
            };
        }

        if self.expression == true {
            Some(last_output.get(0).unwrap().to_owned())
        } else {
            None
        }
    }
}
