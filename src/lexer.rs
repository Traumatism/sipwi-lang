#![allow(unreachable_code)]

use crate::consts::*;
use crate::peeker::Peeker;
use crate::structs::Expression;
use crate::token::Token;

pub struct Lexer {
    chars_peeker: Peeker<char>,
}

impl Lexer {
    pub fn new(code: &str) -> Self {
        Self {
            chars_peeker: Peeker::new(code.chars().collect()),
        }
    }

    /// Parse the next string (between double_quotes)
    fn parse_string(&mut self) -> Token {
        let mut content = String::new();

        while let Some(next_char) = self.chars_peeker.next() {
            if next_char == '"' {
                break;
            }

            content.push(next_char)
        }

        Token::String(content)
    }

    /// Parse the next comment
    fn parse_comment(&mut self) -> Token {
        let mut content = String::new();

        while let Some(next_char) = self.chars_peeker.next() {
            if next_char == COMMENT_MARK {
                break;
            }

            content.push(next_char)
        }

        Token::Comment(content)
    }

    /// Parse the next identifier
    fn parse_identifier(&mut self, char: char) -> Token {
        let mut content = String::from(char);

        while let Some(next_char) = self.chars_peeker.next() {
            if !LC_LETTERS.contains(&next_char)
                && !UC_LETTERS.contains(&next_char)
                && !NUMBERS.contains(&next_char)
                && next_char != '_'
            {
                self.chars_peeker.cursor -= 1;
                break;
            }

            content.push(next_char)
        }

        let str_content = content.as_str();

        if KEYWORDS.contains(&str_content) {
            // token is a keyword
            Token::Keyword(content)
        } else if BOOLEANS.contains(&str_content) {
            // token is a boolean
            match str_content {
                "true" => Token::True,
                "false" => Token::False,
                _ => panic!("EOF"),
            }
        } else {
            Token::Identifier(content) // token is NOT(keyword OR std function OR boolean)
        }
    }

    /// Parse the next number (no float)
    fn parse_number(&mut self, char: char) -> Token {
        let mut content = String::from(char);

        while let Some(next_char) = self.chars_peeker.next() {
            if !NUMBERS.contains(&next_char) {
                self.chars_peeker.cursor -= 1;
                break;
            }
            content.push(next_char)
        }
        Token::Number(content.parse::<isize>().unwrap())
    }

    /// Parse the next expression
    fn parse_expression(&mut self) -> Token {
        // The content of the list (vector of tokens)
        let mut content = Vec::new();

        // The content of the current element
        let mut element_content = String::new();

        loop {
            match self.chars_peeker.next() {
                // End of list
                Some('}') => {
                    break;
                }
                Some('{') => content.push(self.parse_expression()),
                // Same element
                Some(next_char) => element_content.push(next_char),
                // List never ends until EOF
                None => panic!("EOF"),
            }
        }

        Token::Expression(Expression::new(content))
    }

    /// Parse the next list
    fn parse_list(&mut self) -> Token {
        // The content of the list (vector of tokens)
        let mut content = Vec::new();

        // The content of the current element
        let mut element_content = String::new();

        loop {
            match self.chars_peeker.next() {
                // End of list
                Some(']') => {
                    let element_tokens = Lexer::new(&element_content).lex_into_tokens();

                    if element_content.len() > 0 {
                        content.push(element_tokens); // push the current element
                    }

                    break;
                }
                // End of element
                Some(';') => {
                    let element_tokens = Lexer::new(&element_content).lex_into_tokens();

                    if element_content.len() > 0 {
                        content.push(element_tokens); // push the current element
                    }

                    element_content = String::new(); // flush content for the next element
                    continue;
                }
                Some('[') => content.push(std::vec::from_elem(self.parse_list(), 1)),
                // Same element
                Some(next_char) => element_content.push(next_char),
                // List never ends until EOF
                None => panic!("EOF"),
            }
        }

        // Remove the empty elements (they doesn't actually exists)
        let mut new_content = Vec::new();

        content.iter().for_each(|element| {
            if element.len() > 0 {
                new_content.push(element.clone())
            }
        });

        Token::List(new_content)
    }

    /// Lex current code into tokens
    pub fn lex_into_tokens(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(char) = self.chars_peeker.next() {
            tokens.push(match char {
                ' ' => Token::Whitespace,
                '\n' => Token::Newline,
                '+' => Token::Sum,
                '%' => Token::Modulo,
                '*' => Token::Mul,
                '.' => Token::Dot,
                '?' => Token::Question,
                '∈' => Token::In,
                '∉' => Token::NotIn,
                '&' => Token::And,
                '^' => Token::Caret,
                '⊽' => Token::InclusiveOr,
                '⊻' => Token::ExclusiveOr,
                ';' => Token::Semicolon,
                ':' => Token::Colon,
                '(' => Token::OpeningParenthesis,
                ')' => Token::ClosingParenthesis,
                '{' => self.parse_expression(),
                '_' => Token::Underscore,
                '#' => Token::Cross,
                '@' => Token::At,

                '[' => self.parse_list(),
                '"' => self.parse_string(),

                COMMENT_MARK => self.parse_comment(),

                'a'..='z' | 'A'..='Z' => self.parse_identifier(char),
                '0'..='9' => self.parse_number(char),

                '-' => match self.chars_peeker.next() {
                    // -123
                    Some('0'..='9') => self.parse_number(char),

                    // -
                    Some(_) => {
                        self.chars_peeker.cursor -= 1;
                        todo!();
                        Token::Min
                    }
                    None => panic!("EOF"),
                },

                '~' => match self.chars_peeker.next() {
                    // ~>
                    Some('>') => {
                        todo!();
                        Token::WaveArrow
                    }

                    // ~
                    Some(_) => {
                        self.chars_peeker.cursor -= 1;
                        todo!();
                        Token::Wave
                    }
                    None => panic!("EOF"),
                },

                '/' => match self.chars_peeker.next() {
                    // /\
                    Some('\\') => {
                        todo!();
                        Token::Inter
                    }

                    // /
                    Some(_) => {
                        self.chars_peeker.cursor -= 1;
                        todo!();
                        Token::Div
                    }
                    None => panic!("EOF"),
                },

                '\\' => match self.chars_peeker.next() {
                    // \/
                    Some('/') => {
                        todo!();
                        Token::Union
                    }

                    //  \
                    Some(_) => {
                        self.chars_peeker.cursor -= 1;
                        todo!();
                        Token::Backslash
                    }
                    None => panic!("EOF"),
                },

                '!' => match self.chars_peeker.next() {
                    // !=
                    Some('=') => {
                        todo!();
                        Token::NotEqual
                    }

                    // !
                    Some(_) => {
                        self.chars_peeker.cursor -= 1;
                        todo!();
                        Token::Exclam
                    }
                    None => panic!("EOF"),
                },

                '=' => match self.chars_peeker.next() {
                    // ==
                    Some('=') => {
                        todo!();
                        Token::StrictEqual
                    }

                    // =
                    Some(_) => {
                        self.chars_peeker.cursor -= 1;
                        todo!();
                        Token::SingleEqual
                    }
                    None => panic!("EOF"),
                },

                '|' => match self.chars_peeker.next() {
                    // |>
                    Some('>') => Token::Chain,

                    // |
                    Some(_) => {
                        self.chars_peeker.cursor -= 1;
                        todo!();
                        Token::Pipe
                    }
                    None => panic!("EOF"),
                },

                '>' => match self.chars_peeker.next() {
                    // >=
                    Some('=') => {
                        todo!();
                        Token::GreaterThan
                    }

                    // >
                    Some(_) => {
                        self.chars_peeker.cursor -= 1;
                        todo!();
                        Token::StrictGreaterThan
                    }
                    None => panic!("EOF"),
                },

                '<' => match self.chars_peeker.next() {
                    // <-
                    Some('-') => Token::Assignement,

                    // <=...
                    Some('=') => match self.chars_peeker.next() {
                        // <=>
                        Some('>') => {
                            todo!();
                            Token::Equivalent
                        }

                        // <=
                        Some(_) => {
                            self.chars_peeker.cursor -= 1;
                            todo!();
                            Token::LowerThan
                        }
                        _ => panic!("EOF"),
                    },
                    // <
                    Some(_) => {
                        self.chars_peeker.cursor -= 1;
                        todo!();
                        Token::StrictLowerThan
                    }
                    None => panic!("EOF"),
                },

                _ => panic!("Unknown token!"),
            })
        }

        let mut filtered_tokens = Vec::new();
        let mut double_nl = false;

        // Remove bloat stuff
        tokens.iter().for_each(|token| match token {
            // Remove comments and whitespaces
            Token::Comment(_) | Token::Whitespace => {}
            // Remove doubles-newlines
            Token::Newline => {
                if double_nl == false {
                    filtered_tokens.push(token.clone());
                    double_nl = true;
                }
            }
            _ => {
                double_nl = false;
                filtered_tokens.push(token.clone());
            }
        });

        // Remove first newline
        if filtered_tokens.first() == Some(&Token::Newline) {
            filtered_tokens.remove(0);
        }

        // Remove last newline
        if filtered_tokens.last() == Some(&Token::Newline) {
            filtered_tokens.remove(filtered_tokens.len() - 1);
        }

        filtered_tokens
    }
}
