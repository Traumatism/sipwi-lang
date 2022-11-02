#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Chain,                  // |>
    Assignement,            // <-
    List(Vec<Token>),       // [a; b; c]
    Expression(Vec<Token>), // ( ... )
    Number(isize),          // -123
    Keyword(String),        // do, end, fnc...
    String(String),         // "hello world"
    Identifier(String),     // abc
    Comment(String),        // `hello world`
}
