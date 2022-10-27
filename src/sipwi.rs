use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::standard::std_print;
use crate::structs::{Func, FuncResult, StdFunc};
use crate::token::Token;
use crate::verify::verify_do_end;

use std::collections::HashMap;

pub struct Sipwi {
    pub variables_strings: HashMap<String, String>,
    pub variables_numbers: HashMap<String, isize>,
    pub functions: HashMap<String, Func>,
    pub std_functions: HashMap<String, StdFunc>,
    code: String,
}

impl Sipwi {
    pub fn new(code: &str) -> Self {
        Self {
            std_functions: HashMap::new(),
            functions: HashMap::new(),
            variables_numbers: HashMap::new(),
            variables_strings: HashMap::new(),
            code: String::from(code),
        }
    }

    pub fn register_std_func(
        &mut self,
        identifier: &str,
        func: for<'a, 'b> fn(&'a &'b mut Sipwi, Token) -> Option<FuncResult>,
    ) {
        self.std_functions
            .insert(String::from(identifier), StdFunc::new(func));
    }

    pub fn run(&mut self) {
        self.register_std_func("puts", std_print);

        let tokens = Lexer::new(&self.code).lex_into_tokens();

        if !verify_do_end(&tokens) {
            panic!("hum...")
        }

        Parser::new(tokens, self).parse_tokens();

        let main_fn = self.functions.get("main").expect("No main function!");

        Parser::new(main_fn.fnc_tokens.clone(), self).parse_tokens();
    }
}
