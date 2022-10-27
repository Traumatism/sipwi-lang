use crate::peeker::Peeker;
use crate::sipwi::Sipwi;
use crate::structs::Func;
use crate::token::Token;

pub struct Parser<'a> {
    tokens_peeker: Peeker<Token>,
    env: &'a mut Sipwi,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token>, env: &'a mut Sipwi) -> Self {
        Self {
            tokens_peeker: Peeker::new(tokens),
            env,
        }
    }

    pub fn parse_tokens(&mut self) {
        while let Some(token) = self.tokens_peeker.next() {
            match token {
                Token::Chain => {
                    let func_id = self
                        .tokens_peeker
                        .next()
                        .expect("Expected a function name after chain!");

                    let func_name = match func_id {
                        Token::Identifier(name) => name,
                        _ => panic!(),
                    };

                    let func = self.env.std_functions.get(&func_name).unwrap();

                    self.tokens_peeker.cursor -= 3;

                    let previous = self.tokens_peeker.next().unwrap();

                    match &previous {
                        Token::List(_) => &(func.call)(&self.env, previous),
                        _ => panic!("Chain element must be list OR functions"),
                    };

                    self.tokens_peeker.cursor += 1;
                }
                Token::Identifier(identifier) => {
                    match self.tokens_peeker.next() {
                        // name <- ...--
                        Some(Token::Assignement) => match self.tokens_peeker.next() {
                            // name <- "Hello, World!"
                            Some(Token::String(value)) => {
                                self.env.variables_strings.insert(identifier, value);
                            }
                            // name <- 123
                            Some(Token::Number(value)) => {
                                self.env.variables_numbers.insert(identifier, value);
                            }
                            // name <- fnc
                            Some(Token::Keyword(keyword)) => match keyword.as_str() {
                                "fnc" => {
                                    let mut fnc_tokens: Vec<Token> = Vec::new();
                                    let mut fnc_args = Vec::new();

                                    // gather function arguments names
                                    match self.tokens_peeker.next() {
                                        Some(Token::List(list)) => {
                                            for element in &list {

                                                // we want a single identifier <=> a single token
                                                if element.len() != 1 {
                                                    panic!("expected a list of single identifiers after function assignement")
                                                }

                                                match element.first() {
                                                    Some(Token::Identifier(argument_name)) => {
                                                        fnc_args.push(argument_name.clone())
                                                    }
                                                    _ => panic!("expected a list of identifiers after function assignement"),
                                                }
                                            }
                                        }
                                        _ => panic!("expected a list of identifiers after function assignement"),
                                    }

                                    // verify 'do'
                                    match self.tokens_peeker.next() {
                                        Some(Token::Keyword(keyword)) => {
                                            if keyword != "do" {
                                                panic!("expected 'do' after function arguments definition")
                                            }
                                        }
                                        _ => panic!(
                                            "expected 'do' after function arguments definition"
                                        ),
                                    }

                                    // prevent stopping a function after the
                                    // first 'end', even if a 'do' was used inside
                                    // that function
                                    let mut ignore_dos = 0;

                                    loop {
                                        if ignore_dos < 0 {
                                            // too much 'end'
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

                                    if identifier == String::from("main") && fnc_args.len() > 0 {
                                        panic!("main function doesn't take any arguments.")
                                    }

                                    self.env.functions.insert(
                                        identifier.clone(),
                                        Func {
                                            fnc_args,
                                            fnc_tokens,
                                        },
                                    );
                                }
                                _ => {}
                            },
                            _ => {
                                panic!("expected a string, a number of or a function definition")
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
