use crate::consts::MAIN_FUNCTION;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::standard;
use crate::structs::{Func, StdFunc, StdFuncResult, Variable};
use crate::token::Token;
use crate::verify;

use std::collections::HashMap;

pub struct Sipwi {
    pub variables: HashMap<String, Variable>,
    pub functions: HashMap<String, Func>,
    pub std_functions: HashMap<String, StdFunc>,
    code: String,
}

impl Sipwi {
    pub fn new(code: &str) -> Self {
        Self {
            variables: HashMap::new(),
            std_functions: HashMap::new(),
            functions: HashMap::new(),
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

    pub fn get_variable(&self, identifier: &str) -> Option<&Variable> {
        self.variables.get(identifier)
    }

    pub fn register_variable(&mut self, identifier: String, variable: Variable) {
        self.variables.insert(identifier, variable);
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.register_std_func("puts", standard::std_print);
        self.register_std_func("sum", standard::std_sum);
        self.register_std_func("range", standard::std_range);
        self.register_std_func("irange", standard::std_range_inclusive);

        self.register_variable(String::from("nl"), Variable::Str(String::from("\n")));

        let tokens = Lexer::new(&self.code).lex_into_tokens();

        if !verify::verify_do_end(&tokens) {
            panic!("there isn't the same number of 'do' and 'end', kekw")
        }

        Parser::new(tokens, self).parse_tokens();

        let main_fn = self
            .functions
            .get(MAIN_FUNCTION)
            .expect(&format!("{} function not found", MAIN_FUNCTION));

        Parser::new(main_fn.tokens.clone(), self).parse_tokens();

        Ok(())
    }
}
