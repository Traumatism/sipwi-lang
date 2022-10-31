use crate::lexing::consts::MAIN_FUNCTION;
use crate::lexing::token::Token;
use crate::parsing::structs::{Func, Function, Variable};
use crate::peeker::Peeker;
use crate::sipwi::Sipwi;

/// Parse tokens <=> run the program
pub struct Parser<'a> {
    tokens_peeker: Peeker<Token>,
    env: &'a mut Sipwi,
    expression: bool,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token>, env: &'a mut Sipwi, expression: bool) -> Self {
        Self {
            expression,
            tokens_peeker: Peeker::new(tokens),
            env,
        }
    }

    pub fn parse_tokens(&mut self) -> Option<Token> {
        while let Some(token) = self.tokens_peeker.next() {
            match token {
                Token::Expression(expr) => return expr.evaluate(self.env),
                Token::Chain => {
                    let mut functions = Vec::new();

                    let first_input = self.tokens_peeker.previous().unwrap();

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
                        match self.env.get_function(&func_name) {
                            Function::Std(func) => {
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
                            Function::NonStd(func) => {
                                if let Some(Token::List(args_list)) = last_output.get(0) {
                                    let mut base = Vec::new();

                                    func.args.iter().enumerate().for_each(|(idx, arg)| {
                                        base.append(&mut vec![
                                            Token::Identifier(arg.to_owned()),
                                            Token::Assignement,
                                            args_list.get(idx).unwrap().get(0).unwrap().to_owned(),
                                        ]);
                                    });

                                    base.append(&mut func.tokens.to_owned());

                                    Parser::new(base, self.env, false).parse_tokens();
                                } else {
                                    panic!()
                                }
                            }
                        }
                    }

                    if self.expression == true && last_output.len() == 1 {
                        return Some(last_output.get(0).unwrap().to_owned());
                    }
                }
                Token::Identifier(identifier) => {
                    match self.tokens_peeker.next() {
                        // name <- ...
                        Some(Token::Assignement) => match self.tokens_peeker.next() {
                            // name <- { ... }
                            Some(Token::Expression(expression)) => {
                                match expression.evaluate(self.env).unwrap() {
                                    Token::String(value) => {
                                        self.env
                                            .register_variable(&identifier, Variable::Str(value));
                                    }
                                    Token::Number(value) => {
                                        self.env.register_variable(
                                            &identifier,
                                            Variable::Number(value),
                                        );
                                    }
                                    _ => panic!(),
                                }
                            }
                            // name <- "Hello, World!"
                            Some(Token::String(value)) => {
                                self.env
                                    .register_variable(&identifier, Variable::Str(value));
                            }
                            // name <- 123
                            Some(Token::Number(value)) => {
                                self.env
                                    .register_variable(&identifier, Variable::Number(value));
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
                                                fnc_args.push(argument_name.to_owned())
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
                                                fnc_tokens.push(Token::Keyword(keyword.to_owned()));
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
                                    );
                                }
                                _ => {}
                            },
                            _ => {}
                        },
                        _ => self.tokens_peeker.cursor -= 1,
                    }
                }
                Token::Newline => {}
                _ => {}
            };
        }
        None
    }
}
