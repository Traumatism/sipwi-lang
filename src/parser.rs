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
                    self.tokens_peeker.cursor -= 2;

                    let first_input = self.tokens_peeker.next().unwrap();
                    let mut functions = Vec::new();

                    while let Some(next_token) = self.tokens_peeker.next() {
                        if next_token == Token::Chain {
                            if let Some(Token::Identifier(identifier)) = self.tokens_peeker.next() {
                                functions.push(identifier)
                            } else {
                                panic!("expected only functions identifiers in chain calls");
                            }

                            continue;
                        }

                        self.tokens_peeker.cursor -= 1;
                        break;
                    }

                    let mut last_output = vec![first_input];

                    for (idx, func_name) in functions.iter().enumerate() {
                        // grab the FuncDef
                        let func = self
                            .env
                            .std_functions
                            .get(func_name)
                            .expect(&format!("unknown function: {}", func_name));

                        // call the next function with the last arguments
                        let new_output =
                            &(func.call)(&self.env, last_output.clone().get(0).unwrap().clone());

                        if new_output.is_none() {
                            // check if the call that returned None is the last one.
                            // if it's not, panic!
                            if idx != functions.len() - 1 {
                                panic!()
                            }
                        } else {
                            // set the
                            last_output = vec![new_output.as_ref().unwrap().tokens.clone()];
                        }
                    }
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

                                    // get function arguments names
                                    if let Some(Token::List(list)) = self.tokens_peeker.next() {
                                        for element in &list {
                                            // we want a single identifier <=> a single token
                                            if element.len() != 1 {
                                                panic!("expected a list of single identifiers after function assignement")
                                            }

                                            if let Some(Token::Identifier(argument_name)) =
                                                element.first()
                                            {
                                                fnc_args.push(argument_name.clone())
                                            } else {
                                                // no identifier
                                                panic!("expected a list of identifiers after function assignement")
                                            }
                                        }
                                    } else {
                                        panic!("expected a list of identifiers after function assignement")
                                    }

                                    // verify "do"
                                    if let Some(Token::Keyword(keyword)) = self.tokens_peeker.next()
                                    {
                                        if keyword != "do" {
                                            panic!(
                                                "expected 'do' after function arguments definition"
                                            )
                                        }
                                    } else {
                                        panic!("expected 'do' after function arguments definition")
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
                                            None => panic!(
                                                "error parsing function {}: did you forgot the end keyword to finish the function?",
                                                &identifier
                                            ),
                                        }
                                    }

                                    if identifier == String::from("main") && fnc_args.len() > 0 {
                                        panic!(
                                            "error parsing function {}: {} function shouldn't take any argument", 
                                            &identifier, &identifier
                                        )
                                    }

                                    self.env.functions.insert(
                                        identifier,
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
