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

## Project Structure

- `ast/`: Contains types and functions related to building and representing the abstract syntax tree (AST).
- `bytecode/`: Contains types and functions related to generating and representing bytecode.
- `cli/`: The entry point of the language, responsible for parsing command-line arguments and running the file. Provides helper commands for parsing files into AST and bytecode.
- `compiler/`: The compiler of Smollang. Takes a Smollang program and compiles it into bytecode. Also performs optimizations on the AST and bytecode.
- `examples/`: Contains example programs written in Smollang.
- `parser/`: The parser, which takes a string representation of a program and converts it into an AST.
- `vm/`: The virtual machine, which takes bytecode and executes it.
