use std::ops::RangeInclusive;

// Keywords
pub const KEYWORDS: [&str; 3] = [
    "proc", // define a procedure
    "do",   // define a block starting point
    "end",  // define a block ending point
];

// COMMENT_MARK comment_content COMMENT_MARK
pub const COMMENT_MARK: char = '`';

// Numbers
pub const NUMBERS: RangeInclusive<char> = '0'..='9';

// Lowercase alpha letters
pub const LC_LETTERS: RangeInclusive<char> = 'a'..='z';

// Uppercase alpha letters
pub const UC_LETTERS: RangeInclusive<char> = 'A'..='Z';
