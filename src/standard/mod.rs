use crate::common::sipwi::Sipwi;
use crate::parsing::types::StdFuncResult;
use crate::parsing::types::Type;

pub fn std_immune(env: &mut Sipwi, input: Type) -> StdFuncResult {
    match input {
        Type::Str(identifier) => env.register_immutable(&identifier),
        _ => panic!("`immune` expected a string"),
    }

    StdFuncResult::List(vec![])
}

pub fn std_puts(_env: &mut Sipwi, input: Type) -> StdFuncResult {
    match input {
        Type::Str(string) => print!("{string}"),
        Type::Number(number) => print!("{number}"),
        Type::List(elements) => elements.iter().for_each(|tpe| {
            std_puts(_env, tpe.to_owned());
        }),
        _ => panic!("`{input:?}` is not printable."),
    }

    StdFuncResult::List(vec![])
}

pub fn std_sum(_: &mut Sipwi, input: Type) -> StdFuncResult {
    let mut sum = 0_isize;

    match input {
        Type::List(elements) => elements.iter().for_each(|tpe| match tpe {
            Type::Number(number) => sum += number,
            _ => panic!("`sum` arguments must be a list of numbers, not `{tpe:?}`."),
        }),
        _ => panic!("`sum` arguments must be a list of numbers, not `{input:?}`."),
    }

    StdFuncResult::Number(sum)
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

    StdFuncResult::List(
        (start.to_owned()..end.to_owned())
            .map(Type::Number)
            .collect(),
    )
}
