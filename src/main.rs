mod lexing;
mod parsing;
mod peeker;
mod sipwi;
mod standard;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let code = std::fs::read_to_string(&std::env::args().collect::<Vec<String>>().get(1).unwrap())?;

    sipwi::Sipwi::new(&code).run()?;

    Ok(())
}
