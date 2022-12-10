use crate::lexing::{lexer::Lexer, token::Token};

use crate::parsing::{
    parser::Parser,
    types::{Callable, Procedure, StdFunc, StdFuncResult, Type},
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
        self.std_functions.insert(String::from(identifier), func);
    }

    /// Register a procedure
    pub fn register_procedure(&mut self, identifier: &str, proc: Procedure) {
        self.procedures.insert(String::from(identifier), proc);
    }

    /// Unregister a procedure
    pub fn unregister_procedure(&mut self, identifier: &str) {
        self.procedures.remove(identifier);
    }

    /// Register a variable
    pub fn register_variable(&mut self, identifier: &str, variable: Type) {
        if self.immutables.contains(&identifier.to_string()) {
            panic!("Can't register immutable identifier: `{identifier}`")
        }

        self.variables.insert(String::from(identifier), variable);
    }

    /// Get a variable
    pub fn get_variable(&self, identifier: &str) -> &Type {
        self.variables
            .get(identifier)
            .unwrap_or_else(|| panic!("Undefined variable identifier: {identifier}"))
    }

    /// Get a callable
    pub fn get_callable(&self, identifier: &str) -> Callable {
        if let Some(fnc) = self.std_functions.get(&String::from(identifier)) {
            Callable::Std(fnc)
        } else if let Some(fnc) = self.procedures.get(&String::from(identifier)) {
            Callable::Procedure(fnc)
        } else {
            panic!("Undefined callable identifier: {identifier}")
        }
    }

    /// Set a variable as immutable
    pub fn register_immutable(&mut self, identifier: &str) {
        self.immutables.push(String::from(identifier))
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.register_std_func("puts", standard::std_puts);
        self.register_std_func("sum", standard::std_sum);
        self.register_std_func("range", standard::std_range);
        self.register_std_func("immune", standard::std_immune);

        self.register_variable("true", Type::Bool(true));
        self.register_variable("false", Type::Bool(false));
        self.register_variable("nl", Type::Str(String::from("\n")));

        self.register_immutable("true");
        self.register_immutable("false");
        self.register_immutable("nl");

        let mut final_tokens = Vec::new();

        let mut tokens = Lexer::new(&self.code).lex_into_tokens();

        tokens.iter().for_each(|token| {
            if let Token::Import(path) = token {
                final_tokens.append(
                    &mut Lexer::new(
                        &std::fs::read_to_string(path)
                            .unwrap_or_else(|_| panic!("Failed to import: {path}")),
                    )
                    .lex_into_tokens(),
                );
            }
        });

        final_tokens.append(&mut tokens);

        Parser::new(final_tokens, self).parse_tokens();

        Ok(())
    }
}
