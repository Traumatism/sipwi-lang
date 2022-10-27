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
