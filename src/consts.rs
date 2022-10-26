use std::ops::RangeInclusive;

// Keywords
pub const KEYWORDS: [&str; 8] = [
    "fnc",  // define a function
    "do",   // define a block starting point
    "end",  // define a block ending point
    "if",   // if condition
    "in",   // check if something is in a list
    "not",  // not
    "elif", // else if condition
    "else", // else condition
];

// Booleans
pub const BOOLEANS: [&str; 2] = [
    "false", // 0
    "true",  // 1
];

// COMMENT_MARK comment_content COMMENT_MARK
pub const COMMENT_MARK: char = '`';

// Numbers
pub const NUMBERS: RangeInclusive<char> = '0'..='9';

// Lowercase alpha letters
pub const LC_LETTERS: RangeInclusive<char> = 'a'..='z';

// Uppercase alpha letters
pub const UC_LETTERS: RangeInclusive<char> = 'A'..='Z';
