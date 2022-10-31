// Tokens
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Newline,                // \n
    Whitespace,             //
    Semicolon,              // ;
    Chain,                  // |>
    Assignement,            // <-
    List(Vec<Vec<Token>>),  // [a; b; c]
    Expression(Vec<Token>), // { ... }
    String(String),         // "hello world"
    Number(isize),          // -123
    Keyword(String),        // proc
    Identifier(String),     // x
    Comment(String),        // `hello world`
}
