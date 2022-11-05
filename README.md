## The Sipwi Programming Language

Sipwi is a _(trashy)_ dynamic weakly typed toy programming language fully made on top of Rust (for safety and high performances). It have been designed for writing cute short programs with a functionnal approach.

## Compile

`$ git clone https://github.com/traumatism/sipwi-lang`

`$ make install`

`$ make build`

## Execute your scripts

`$ sipwi exec <path.spw>`

## Hello, world!

```
["Hello world"; nl] |> puts
```

## Types

Sipwi uses a bunch of differents types:

* String (`"foo bar"`)
* Int (`-123`, `123`)
* List (`[123; -123; "foo"; [hihi; "bar"]]`)
* Bool (`false`, `true`) boolean operations aren't implemented yet!

## Define a variable

To define a variable, use the next syntax:

`variable_name <- data` where data must be one of the types

## Define a procedure

To define a procedure, use the next syntax:

```
procedure_name <- proc [argument_1; argument_2; and_so_on] do

    `your code here`

end
```

Note that comments are between `\``

## Call a procedure

Simple call:
```
[arg_1; arg_2; and_so_on] |> procedure_name
```

Stacked call:
```
[...] |> first_procedure |> ... |> last_procedure
```

## Expressions

An expression is a code snippet that MUST return a value.
To define an expression, use the next syntax:

`variable_name <- ([-1; 2; 3] |> sum)`

Will execute as:

`variable_name <- 4`

## 1 + 1

```
[1; 1] |> sum |> puts
nl |> puts
```

## Implement putsln

```
putsln <- proc [content] do
    [content; nl] |> puts
end
```

## Sum of all numbers from 1 to n

### in lib.spw
```
putsln <- proc [content] do
    [content; nl] |> puts
end

f <- proc [n] do
    n <- ([1; n] |> sum) `range returns all digits from start to end excluded`
    [1; n] |> range |> sum |> putsln
end

```

### in main.spw
```
@"lib.spw"

n <- 100

["Using next n value: "; n; nl] |> puts

n |> f
```

Note that you can import procedures between differents files using `@"file_path"`

### Errors

Sipwi expection design is the simply the best! just try it yourself ><
```
n <- hihi
n <| putsln
```

## Write standard function

Standard functions are located in src/standards.
To register a standard function use `self.register_std_function` inside src/common/sipwi.rs:Sipwi::run()


```rust
/// Implements puts standard function
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
```