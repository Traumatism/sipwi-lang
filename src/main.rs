mod common;
mod lexing;
mod parsing;
mod standard;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(common::sipwi::Sipwi::new(&std::fs::read_to_string(
        &std::env::args().collect::<Vec<String>>().get(1).unwrap(),
    )?)
    .run()?)
}
