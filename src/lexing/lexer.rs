use crate::common::peeker::Peeker;
use crate::lexing::consts::{COMMENT_MARK, KEYWORDS, LC_LETTERS, NUMBERS, UC_LETTERS};
use crate::lexing::token::Token;

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
                self.chars_peeker.previous();
                break;
            }

            content.push(next_char)
        }

        let str_content = content.as_str();

        if KEYWORDS.contains(&str_content) {
            // token is a keyword
            Token::Keyword(content)
        } else {
            Token::Identifier(content) // token is NOT(keyword OR std function OR boolean)
        }
    }

    /// Parse the next number (no float)
    fn parse_number(&mut self, chr: char, neg: bool) -> Token {
        let mut content = String::new();
        if neg {
            content.push('-');
        }
        content.push(chr);

        while let Some(next_char) = self.chars_peeker.next() {
            if next_char == '_' {
                continue;
            }

            if !NUMBERS.contains(&next_char) {
                self.chars_peeker.previous();
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
                Some(')') => {
                    break;
                }
                Some('(') => content.push(self.parse_expression()),
                // Same element
                Some(next_char) => element_content.push(next_char),
                // List never ends until EOF
                None => panic!("EOF"),
            }
        }

        Token::Expression(Lexer::new(&element_content).lex_into_tokens())
    }

    /// Parse the next list
    fn parse_list(&mut self) -> Token {
        // The content of the list (vector of tokens)
        let mut content: Vec<Token> = Vec::new();

        // The content of the current element
        let mut element_content = String::new();

        loop {
            match self.chars_peeker.next() {
                // End of list
                Some(']') => {
                    let element_tokens = Lexer::new(&element_content).lex_into_tokens();

                    let len = element_tokens.len();

                    match len {
                        1 => content.push(element_tokens.get(0).unwrap().to_owned()),
                        len if len > 1 => panic!("List element must contains... a single element?"),
                        _ => {}
                    }

                    break;
                }
                // End of element
                Some(';') => {
                    let element_tokens = Lexer::new(&element_content).lex_into_tokens();

                    let len = element_tokens.len();

                    match len {
                        1 => content.push(element_tokens.get(0).unwrap().to_owned()),
                        len if len > 1 => panic!("List element must contains... a single element?"),
                        _ => {}
                    }

                    element_content = String::new(); // flush content for the next element

                    continue;
                }
                Some('[') => content.push(self.parse_list()),
                // Same element
                Some(next_char) => element_content.push(next_char),
                // List never ends until EOF
                None => panic!("EOF"),
            }
        }

        Token::List(content)
    }

    /// Lex current code into tokens
    pub fn lex_into_tokens(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(char) = self.chars_peeker.next() {
            tokens.push(match char {
                '(' => self.parse_expression(),
                '[' => self.parse_list(),
                '"' => self.parse_string(),
                COMMENT_MARK => self.parse_comment(),
                '@' => {
                    self.chars_peeker.next();
                    match self.parse_string() {
                        Token::String(path) => Token::Import(path),
                        _ => panic!(),
                    }
                }
                'a'..='z' | 'A'..='Z' => self.parse_identifier(char),
                '0'..='9' => self.parse_number(char, false),
                '-' => {
                    let next = self.chars_peeker.next();
                    match next {
                        // -123
                        Some('0'..='9') => self.parse_number(next.unwrap(), true),
                        None => panic!("EOF"),
                        _ => panic!("Expected a number after `-`"),
                    }
                }

                '|' => match self.chars_peeker.next() {
                    // |>
                    Some('>') => Token::Chain,
                    None => panic!("EOF"),
                    _ => panic!("Expected a `>` after a `|` to form a `|>`"),
                },

                '=' => Token::Assignement,

                ' ' | '\n' | '\t' => {
                    continue;
                }

                token => panic!("Unknown token: {token:?}"),
            })
        }

        let mut filtered_tokens = Vec::new();

        // Remove bloat stuff
        tokens.iter().for_each(|token| match token {
            // Remove comments and whitespaces
            Token::Comment(_) => {}
            _ => {
                filtered_tokens.push(token.to_owned());
            }
        });

        filtered_tokens
    }
}
