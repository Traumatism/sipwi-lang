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
        func: for<'b> fn(&'b Sipwi, Token) -> StdFuncResult,
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

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.register_std_func("puts", standard::puts::std_puts);
        self.register_std_func("randint", standard::random::std_randint);
        self.register_std_func("sum", standard::operations::std_sum);
        self.register_std_func("gauss_sum", standard::operations::std_gauss_sum);
        self.register_std_func("range", standard::range::std_range);

        self.register_variable("true", Type::Bool(true));
        self.register_immutable("true");

        self.register_variable("false", Type::Bool(false));
        self.register_immutable("false");

        self.register_variable("nl", Type::Str(String::from("\n")));
        self.register_immutable("nl");

        let mut tokens = Lexer::new(&self.code).lex_into_tokens();

        if !verify::verify_do_end(&tokens) {
            panic!()
        }

        Parser::new(tokens.clone(), self, false, None).parse_tokens();

        let import_fn = self.procedures.get("import");

        match import_fn {
            Some(proc) => {
                for token in &proc.tokens {
                    match token {
                        Token::Import(path) => {
                            let mut imported_tokens =
                                Lexer::new(&std::fs::read_to_string(path).unwrap())
                                    .lex_into_tokens();

                            tokens.append(&mut imported_tokens)
                        }
                        _ => panic!(),
                    }
                }
            }
            _ => {}
        };

        Parser::new(tokens.clone(), self, false, None).parse_tokens();

        let main_fn = self
            .procedures
            .get(MAIN_FUNCTION)
            .expect(&format!("{} function not found", MAIN_FUNCTION));

        Parser::new(
            main_fn.tokens.to_owned(),
            self,
            false,
            Some(String::from("main")),
        )
        .parse_tokens();

        Ok(())
    }
}
