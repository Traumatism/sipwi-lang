# sipwi programming language

## Compile

`cargo build --release && mv target/release/sipwi .`

## Execute sipwi file

`./sipwi <path.spw>`

## Hello, world!

```
main <- fnc [] do
    
    a <- "hello, world!"
    [a] |> puts

    `or`

    ["hello, world!"] |> puts
end
```

## 1+1

```
main <- fnc [] do
    [1; 1] |> sum |> puts
end
```

## Writing standard function

```rust
// in src/standard.rs

// import required modules
use crate::sipwi::Sipwi;
use crate::token::Token;
use crate::structs::StdFuncResult;

// - 'env' can be used to manage variables ect.
// - 'token' should be a Token::List
pub fn std_name(env: &&mut Sipwi, token: Token) -> Option<StdFuncResult> {
 
    // your code here ...

    // The result should also be a Token::List
    Some(StdFuncResult::new(Token::List(...)))
}
```

## Register standard function

```rust
// in src/sipwi.rs:Sipwi::run()
self.register_std_func("sum", standard::std_name);
```