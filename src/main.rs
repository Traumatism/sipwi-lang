mod common;
mod lexing;
mod parsing;
mod standard;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() != 2 {
        panic!("Missing file")
    }

    common::sipwi::Sipwi::new(
        std::fs::read_to_string(args.get(1).unwrap()).expect("Failed to open file"),
    )
    .run()
    .unwrap();
}
