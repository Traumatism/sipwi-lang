mod common;
mod lexing;
mod parsing;
mod standard;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let usage = "Usage: sipwi <lex/exec> <file>";

    let args = std::env::args().collect::<Vec<String>>();

    if args.len() > 3 {
        panic!("{}", usage)
    }

    let action = args.get(1).expect(usage).as_str();
    let content = std::fs::read_to_string(args.get(2).expect(usage)).expect("Failed to open file");

    match action {
        "lex" => {
            lexing::lexer::Lexer::new(&content)
                .lex_into_tokens()
                .iter()
                .for_each(|token| println!("{:?}", token));

            Ok(())
        }
        "exec" => Ok(common::sipwi::Sipwi::new(&content).run()?),
        _ => panic!("{}", usage),
    }
}
