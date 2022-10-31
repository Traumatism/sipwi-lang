use crate::parsing::structs::Expression;

// Tokens
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Newline,                // \n
    Underscore,             // _
    Whitespace,             //
    Semicolon,              // ;
    Chain,                  // |>
    Assignement,            // <-
    List(Vec<Vec<Token>>),  // [a; b; c]
    Expression(Expression), // { ... }
    String(String),         // "hello world"
    Number(isize),          // -123
    Keyword(String),        // proc
    Identifier(String),     // x
    Comment(String),        // `hello world`
}
