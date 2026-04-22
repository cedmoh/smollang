# Smollang

A tiny minimalistic functional programming language with simple syntax.

## Features

- Concise syntax
- Everything is an expression
- Dynamic typing
- Pattern matching
- Simple error handling
- REPL for interactive programming
- Easy to learn and use
- Small standard library
- Interop with Rust

## Examples

```
# Hello World
print 'Hello, World!'
```

```
# Factorial
factorial fn(n) n match
    on 0 do 1
    on _ do n - 1 > factorial it > n * it
```

```
# Fizz Buzz
fizzbuzz fn(n) [n.mod 3, n.mod 5] match
    on [0, 0] do "FizzBuzz"
    on [0, _] do "Fizz"
    on [_, 0] do "Buzz"
    on _ do n.to_string
```
