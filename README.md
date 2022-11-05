## The Sipwi Programming Language

### Compile

`cargo build --release && mv target/release/sipwi .`

### Execute sipwi file

`./sipwi <path.spw>`

### Variable definition

`name <- "string"` (string)
`name <- true` (bool)
`name <- -123` (isize)

### Procedure calling

`name_b(name_a(arg1, arg2))` (Python)

<=>

`[arg1; arg2] |> name_a |> name_b` (Sipwi)


(Python)
`name("hello")`
`name(123)`
`name(x)`

<=>

(Sipwi)
`"hello" |> name`
`123 |> name`
`x |> name`


### Hello, world!

```
main <- proc [] do
    "hello, world!" |> puts
end
```

### 1+1

```
main <- proc [] do
    [1; 1] |> sum |> puts
end
```

### Importing procedures

Imports must be inside the `import` procedure.

```
import <- proc [] do
    @"lib.spw"
    @"logging.spw"
end
```

### Calling other procedures

```
f <- proc [arg_a; arg_b] do
    [arg_a; arg_b] |> sum |> puts
end

main <- proc [] do
    [5; 5] |> f
end
```

### Using expressions

```
main <- proc [] do
    a <- 1
    b <- 100

    `store the sum of all numbers from 1 to 100 in 'x'`
    x <- ([a; b] |> range |> sum)

    [x; nl] |> puts
end
```

## Writing standard function

```rust
// in src/standard.rs

// import required modules
use crate::common::sipwi::Sipwi;
use crate::lexing::token::Token;
use crate::parsing::structs::StdFuncResult;

// - 'env' can be used to read variables ect.
// - 'token' should be a Token::List
pub fn std_name(env: &Sipwi, token: Token) -> StdFuncResult {
 
    // your code here ...

    // The result should also be a Token::List
    StdFuncResult::new(Token::List(...))
}
```

### Register standard function

```rust
// in src/common/sipwi.rs:Sipwi::run()
self.register_std_func("jame", standard::std_name);
```

### Implementing sum() standard function

```rust
pub fn std_sum(env: &Sipwi, token: Token) -> StdFuncResult {
    let mut total = 0;

    match token {
        Token::List(list) => {
            for element in list {
                match element {
                    Token::Number(number) => total += number,
                    Token::Identifier(identifier) => {
                        let value = env.get_variable(&identifier);
                        match value {
                            Type::Number(number) => {
                                total += number;
                            }
                            variable_type => panic!("Cannot add a {:?}", variable_type),
                        };
                    }
                    token => panic!("Cannot add a {:?}", token),
                }
            }
        }
        _ => {
            panic!("'sum' expect a list of integers/identifiers as arguments!")
        }
    }

    StdFuncResult::new(Token::Number(total))
}
```
