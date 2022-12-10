use std::ops::RangeInclusive;

pub const KEYWORDS: [&str; 4] = [
    "proc", // define a procedure
    "do",   // define a block starting point
    "end",  // define a block ending point
    "for",  // define a loop
];

pub const COMMENT_MARK: char = '`';
pub const NUMBERS: RangeInclusive<char> = '0'..='9';
pub const LC_LETTERS: RangeInclusive<char> = 'a'..='z';
pub const UC_LETTERS: RangeInclusive<char> = 'A'..='Z';
