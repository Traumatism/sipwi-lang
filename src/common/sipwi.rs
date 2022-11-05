use crate::lexing::consts::IMPORT_FUNCTION;
use crate::lexing::{consts::MAIN_FUNCTION, lexer::Lexer, token::Token};

use crate::parsing::{
    parser::Parser,
    structs::{Callable, Procedure, StdFunc, StdFuncResult, Type},
    verify,
};
use crate::standard;

use std::collections::HashMap;

/// Sipwi environment (manages variables, functions...)
pub struct Sipwi {
    pub variables: HashMap<String, Type>,
    pub procedures: HashMap<String, Procedure>,
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
            procedures: HashMap::new(),
            code: String::from(code),
        }
    }

    /// Register a new standard function
    pub fn register_std_func(
        &mut self,
        identifier: &str,
        func: fn(&mut Sipwi, Type) -> StdFuncResult,
    ) {
        self.std_functions
            .insert(String::from(identifier), StdFunc::new(func));
    }

    /// Register a procedure
    pub fn register_procedure(&mut self, identifier: &str, proc: Procedure) {
        self.procedures.insert(String::from(identifier), proc);
    }

    /// Register a variable
    pub fn register_variable(&mut self, identifier: &str, variable: Type) {
        if self.immutables.contains(&identifier.to_string()) {
            panic!()
        }

        self.variables.insert(String::from(identifier), variable);
    }

    /// Get a variable
    pub fn get_variable(&self, identifier: &str) -> &Type {
        self.variables
            .get(identifier)
            .expect(&format!("Undefined variable identifier: {}", identifier))
    }

    /// Get a callable
    pub fn get_callable(&self, identifier: &str) -> Callable {
        if let Some(fnc) = self.std_functions.get(&String::from(identifier)) {
            Callable::Std(fnc)
        } else if let Some(fnc) = self.procedures.get(&String::from(identifier)) {
            Callable::Procedure(fnc)
        } else {
            panic!("Undefined callable identifier: {}", identifier)
        }
    }

    /// Set a variable as immutable
    pub fn register_immutable(&mut self, identifier: &str) {
        self.immutables.push(String::from(identifier))
    }

    fn resolve_imports(&mut self) {
        match self.procedures.get(IMPORT_FUNCTION) {
            Some(proc) => {
                for import_proc_token in proc.tokens.clone() {
                    match import_proc_token.clone() {
                        Token::Import(path) => {
                            let imported_tokens = Lexer::new(
                                &std::fs::read_to_string(&path)
                                    .expect(&format!("Failed to import: {}", path)),
                            )
                            .lex_into_tokens();

                            Parser::new(imported_tokens, self)
                                .parse_tokens(Some(String::from(IMPORT_FUNCTION)));
                        }
                        _ => panic!("`import` procedure must only contains imports (@\"path.spw\""),
                    };
                }
            }
            _ => {}
        };
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.register_std_func("puts", standard::std_puts);
        self.register_std_func("sum", standard::std_sum);
        self.register_std_func("range", standard::std_range);

        self.register_variable("true", Type::Bool(true));
        self.register_immutable("true");

        self.register_variable("false", Type::Bool(false));
        self.register_immutable("false");

        self.register_variable("nl", Type::Str(String::from("\n")));
        self.register_immutable("nl");

        let tokens = Lexer::new(&self.code).lex_into_tokens();

        if !verify::verify_do_end(&tokens) {
            panic!()
        }

        Parser::new(tokens.clone(), self).parse_tokens(None);

        self.resolve_imports();

        let main_fn = self
            .procedures
            .get(MAIN_FUNCTION)
            .expect(&format!("{} function not found", MAIN_FUNCTION));

        // Run the main procedure
        Parser::new(main_fn.tokens.to_owned(), self)
            .parse_tokens(Some(String::from(MAIN_FUNCTION)));

        Ok(())
    }
}
