use std::ops::RangeInclusive;

pub const KEYWORDS: [&str; 4] = [
    "proc", // define a procedure
    "do",   // define a block starting point
    "end",  // define a block ending point
    "for",  // define a loop
];

pub const COMMENT_MARK: char = '*';
pub const BLOCK_OPENING_MARK: &str = "do";
pub const BLOCK_CLOSING_MARK: &str = "end";
pub const FOR_LOOP_MARK: &str = "for";
pub const PROC_DEF_MARK: &str = "proc";
pub const NUMBERS: RangeInclusive<char> = '0'..='9';
pub const LC_LETTERS: RangeInclusive<char> = 'a'..='z';
pub const UC_LETTERS: RangeInclusive<char> = 'A'..='Z';
