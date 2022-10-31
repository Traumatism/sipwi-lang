mod lexing;
mod parsing;
mod peeker;
mod sipwi;
mod standard;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(sipwi::Sipwi::new(&std::fs::read_to_string(
        &std::env::args().collect::<Vec<String>>().get(1).unwrap(),
    )?)
    .run()?)
}
