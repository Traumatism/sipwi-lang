use crate::lexing::token::Token;

pub fn verify_do_end(tokens: &Vec<Token>) -> bool {
    let mut s = 0;

    for token in tokens {
        // N(end) > N(do)
        if s < 0 {
            return false;
        }

        match token {
            Token::Keyword(keyword) => match keyword.as_str() {
                "do" => s += 1,  // block start
                "end" => s -= 1, // block end
                _ => {}          // keyword ∉ {"do", "end"} => ignore
            },
            _ => {} // token != keyword
        }
    }

    s == 0 // N(do) == N(end)
}
