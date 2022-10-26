# sipwi programming language

## Hello, world!

```
`this is a comment`

main <- fnc [] do
    `variable names are global!`
    hello_world <- "Hello, world!"

    [hello_world] |> puts
end
```

## Fibonacci

(not fully implemented yet)

```
fib <- fnc [n; a; b] do
    if n == 0 do
        [a] |> return
    end elif n == 1 do
        [b] |> return
    end else do
        [{n-1}; b; {a+b}] |> fib |> return
    end
end

main <- fnc [] do
    [10; 0; 1] |> fib |> puts
end
```