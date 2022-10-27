mod consts;
mod lexer;
mod parser;
mod peeker;
mod sipwi;
mod standard;
mod structs;
mod token;
mod verify;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let code = std::fs::read_to_string(
        &std::env::args()
            .collect::<Vec<String>>()
            .get(1)
            .expect("Please provide a file to execute"),
    )
    .expect("Failed to read from file");

    sipwi::Sipwi::new(&code).run();

    Ok(())
}
