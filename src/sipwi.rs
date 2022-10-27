use crate::consts::MAIN_FUNCTION;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::standard;
use crate::structs::{Func, StdFunc, StdFuncResult};
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
        func: for<'b, 'c> fn(&'b &'c mut Sipwi, Token) -> Option<StdFuncResult>,
    ) {
        self.std_functions
            .insert(String::from(identifier), StdFunc::new(func));
    }

    pub fn register_string(&mut self, name: &str, value: &str) {
        self.variables_strings
            .insert(String::from(name), String::from(value));
    }

    pub fn run(&mut self) {
        self.register_std_func("puts", standard::std_print); // ["hello"] |> puts
        self.register_std_func("sum", standard::std_sum); // [1; 2; 3] |> sum
        self.register_std_func("range", standard::std_range);
        self.register_std_func("irange", standard::std_range_inclusive);

        self.register_string("nl", "\n");

        let tokens = Lexer::new(&self.code).lex_into_tokens();

        if !verify_do_end(&tokens) {
            panic!("there isn't the same number of 'do' and 'end', kekw")
        }

        Parser::new(tokens, self).parse_tokens();

        let main_fn = self
            .functions
            .get(MAIN_FUNCTION)
            .expect(&format!("{} function not found.", MAIN_FUNCTION));

        Parser::new(main_fn.fnc_tokens.clone(), self).parse_tokens();
    }
}
