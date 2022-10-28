# sipwi programming language

## Compile

`cargo build --release && mv target/release/sipwi .`

## Execute sipwi file

`./sipwi <path.spw>`

## Variable definition

`name <- "string"` (string)

`name <- -123` (isize)

## Function calling

`name_b(name_a(arg1, arg2))` (Python)

<=>

`[arg1; arg2] |> name_a |> name_b` (Sipwi)

## Hello, world!

```
main <- fnc [] do
    ["hello, world!"] |> puts
end
```

## 1+1

```
main <- fnc [] do
    [1; 1] |> sum |> puts
end
```

## Calling other 'functions'

```
f <- fnc [arg_a; arg_b] do
    [arg_a; arg_b] |> sum |> puts
end

main <- fnc [] do
    [5; 5] |> f
end
```

## Sum of all numbers from 25 to 50 (inclusive and exclusive)

```
main <- fnc [] do
    ["25 to 50 exclusive => "] |> puts
    [25; 50] |> range |> sum |> puts
    
    [nl] |> puts

    ["25 to 50 inclusive => "] |> puts
    [25; 50] |> irange |> sum |> puts

    [nl] |> puts
end

```

## Writing standard function

```rust
// in src/standard.rs

// import required modules
use crate::sipwi::Sipwi;
use crate::lexing::token::Token;
use crate::parsing::structs::StdFuncResult;

// - 'env' can be used to manage variables ect.
// - 'token' should be a Token::List
pub fn std_name(env: &&mut Sipwi, token: Token) -> StdFuncResult {
 
    // your code here ...

    // The result should also be a Token::List
    StdFuncResult::new(Token::List(...))
}
```

## Register standard function

```rust
// in src/sipwi.rs:Sipwi::run()
self.register_std_func("sum", standard::std_name);
```