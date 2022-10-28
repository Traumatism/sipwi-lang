use crate::parsing::structs::Expression;

// Tokens
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Cross,                  // #
    At,                     // @
    Newline,                // \n
    Underscore,             // _
    Whitespace,             //
    Sum,                    // +
    Div,                    // /
    Min,                    // -
    Modulo,                 // %
    Mul,                    // *
    Dot,                    // .
    In,                     // ∈
    NotIn,                  // ∉
    And,                    // &
    InclusiveOr,            // ⊽
    ExclusiveOr,            // ⊻
    Semicolon,              // ;
    Colon,                  // :
    OpeningParenthesis,     // ()
    ClosingParenthesis,     // )
    Caret,                  // ^
    Wave,                   // ~
    WaveArrow,              // ~>
    Inter,                  // /\
    Union,                  // \/
    Backslash,              // \
    NotEqual,               // !=
    Exclam,                 // !
    Question,               // ?
    Chain,                  // |>
    StrictEqual,            // ==
    SingleEqual,            // =
    Equivalent,             // <=>
    LowerThan,              // <=
    StrictLowerThan,        // <
    GreaterThan,            // >=
    StrictGreaterThan,      // >
    Pipe,                   // |
    Assignement,            // <-
    True,                   // true
    False,                  // false
    List(Vec<Vec<Token>>),  // [a; b; c]
    Expression(Expression), // { ... }
    String(String),         // "hello world"
    Number(isize),          // -123
    Keyword(String),        // proc
    Identifier(String),     // x
    Comment(String),        // `hello world`
}
