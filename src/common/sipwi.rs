use crate::lexing::{lexer::Lexer, token::Token};
use crate::parsing::{parser::Parser, types::Type};
use crate::standard;

use std::collections::HashMap;
use std::fmt::Display;

pub struct Sipwi {
    pub variables: HashMap<String, Type>,
    pub procedures: HashMap<String, (Vec<String>, Vec<Token>)>,
    pub std_functions: HashMap<String, fn(&mut Sipwi, Type) -> Type>,
    pub immutables: Vec<String>,
    code: String,
}

impl Sipwi {
    pub fn new(code: impl Into<String>) -> Self {
        Self {
            immutables: Vec::new(),
            variables: HashMap::new(),
            std_functions: HashMap::new(),
            procedures: HashMap::new(),
            code: code.into(),
        }
    }

    /// Register a new standard function
    pub fn register_std_func(
        &mut self,
        identifier: impl Into<String>,
        func: fn(&mut Sipwi, Type) -> Type,
    ) {
        self.std_functions.insert(identifier.into(), func);
    }

    /// Register a variable
    pub fn register_variable(
        &mut self,
        identifier: impl Into<String> + Display + Clone,
        variable: Type,
    ) {
        if self.immutables.contains(&identifier.clone().into()) {
            panic!("Can't register immutable identifier: `{identifier}`")
        }

        self.variables.insert(identifier.to_string(), variable);
    }

    /// Set a variable as immutable
    pub fn register_immutable(&mut self, identifier: impl Into<String>) {
        self.immutables.push(identifier.into())
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
