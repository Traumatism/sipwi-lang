use crate::common::sipwi::Sipwi;
use crate::parsing::structs::StdFuncResult;
use crate::parsing::structs::Type;

/// Make an identifier immutable
pub fn std_immune(env: &mut Sipwi, input: Type) -> StdFuncResult {
    match input {
        Type::Str(identifier) => env.register_immutable(&identifier),
        _ => panic!("`immune` expected a string"),
    }

    StdFuncResult::empty()
}

/// Write to stdout
pub fn std_puts(env: &mut Sipwi, input: Type) -> StdFuncResult {
    match input {
        Type::Str(string) => print!("{}", string),
        Type::Number(number) => print!("{}", number),
        Type::List(elements) => elements.iter().for_each(|tpe| {
            std_puts(env, tpe.to_owned());
        }),
        _ => panic!("`{:?}` is not printable.", input),
    }

    StdFuncResult::empty()
}

/// Sum
pub fn std_sum(_: &mut Sipwi, input: Type) -> StdFuncResult {
    let mut sum = 0 as isize;

    match input {
        Type::List(elements) => elements.iter().for_each(|tpe| match tpe {
            Type::Number(number) => sum += number,
            _ => panic!(
                "`sum` arguments must be a list of numbers, not `{:?}`.",
                tpe
            ),
        }),
        _ => panic!(
            "`sum` arguments must be a list of numbers, not `{:?}`.",
            input
        ),
    }

    StdFuncResult::new(Type::Number(sum))
}

pub fn std_range(_: &mut Sipwi, input: Type) -> StdFuncResult {
    let elements = match input {
        Type::List(elements) => elements,
        _ => panic!(),
    };

    if elements.len() != 2 {
        panic!("`range` expect two arguments")
    }

    let (start, end) = match (elements.get(0).unwrap(), elements.get(1).unwrap()) {
        (Type::Number(start), Type::Number(end)) => (start, end),
        _ => panic!(),
    };

    StdFuncResult::new(Type::List(
        (start.to_owned()..end.to_owned())
            .map(|n| (Type::Number(n)))
            .collect(),
    ))
}
