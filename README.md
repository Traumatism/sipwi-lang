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
// in src/standard/mod.rs

// - 'env' can be used to read variables ect.
// - 'token' should be a Token::List
pub fn std_name(env: &Sipwi, input: Type) -> StdFuncResult {
 
    // your code here ...

    StdFuncResult::new(...)
}
```

### Register standard function

```rust
// in src/common/sipwi.rs:Sipwi::run()
self.register_std_func("name", standard::std_name);
```

### Implementing sum() standard function

```rust
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

```
