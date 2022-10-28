use crate::lexing::consts::MAIN_FUNCTION;
use crate::lexing::lexer::Lexer;
use crate::lexing::token::Token;
use crate::parsing::parser::Parser;
use crate::parsing::structs::{Func, Function, StdFunc, StdFuncResult, Variable};
use crate::parsing::verify;
use crate::standard;

use std::collections::HashMap;

pub struct Sipwi {
    pub variables: HashMap<String, Variable>,
    pub functions: HashMap<String, Func>,
    pub std_functions: HashMap<String, StdFunc>,
    pub immutables: Vec<String>,
    code: String,
}

impl Sipwi {
    pub fn new(code: &str) -> Self {
        Self {
            immutables: Vec::new(),
            variables: HashMap::new(),
            std_functions: HashMap::new(),
            functions: HashMap::new(),
            code: String::from(code),
        }
    }

    pub fn register_std_func(
        &mut self,
        identifier: &str,
        func: for<'b, 'c> fn(&'b &'c mut Sipwi, Token) -> StdFuncResult,
    ) {
        self.std_functions
            .insert(String::from(identifier), StdFunc::new(func));
    }

    pub fn get_variable(&self, identifier: &str) -> Option<&Variable> {
        self.variables.get(identifier)
    }

    pub fn get_function(&self, identifier: &str) -> Option<Function> {
        if let Some(fnc) = self.std_functions.get(&String::from(identifier)) {
            return Some(Function::Std(fnc));
        }

        if let Some(fnc) = self.functions.get(&String::from(identifier)) {
            return Some(Function::NonStd(fnc));
        }

        return None;
    }

    pub fn register_function(&mut self, identifier: &str, fnc: Func) {
        self.functions.insert(String::from(identifier), fnc);
    }

    pub fn register_variable(&mut self, identifier: &str, variable: Variable) {
        if self.immutables.contains(&identifier.to_string()) {
            panic!()
        }

        self.variables.insert(String::from(identifier), variable);
    }

    pub fn register_immutable(&mut self, identifier: &str) {
        self.immutables.push(String::from(identifier))
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.register_std_func("puts", standard::puts::std_puts);
        self.register_std_func("sum", standard::operations::std_sum);
        self.register_std_func("range", standard::range::std_range);
        self.register_std_func("irange", standard::range::std_range_inclusive);
        self.register_std_func("for_each", standard::iter::std_for_each);
        self.register_std_func("randint", standard::random::std_randint);

        self.register_variable("true", Variable::Bool(true));
        self.register_immutable("true");

        self.register_variable("false", Variable::Bool(false));
        self.register_immutable("false");

        self.register_variable("nl", Variable::Str(String::from("\n")));
        self.register_immutable("nl");

        let tokens = Lexer::new(&self.code).lex_into_tokens();

        if !verify::verify_do_end(&tokens) {
            panic!()
        }

        Parser::new(tokens, self, false).parse_tokens();

        let main_fn = self
            .functions
            .get(MAIN_FUNCTION)
            .expect(&format!("{} function not found", MAIN_FUNCTION));

        Parser::new(main_fn.tokens.to_owned(), self, false).parse_tokens();

        Ok(())
    }
}
