# Curly Braces

Curly braces denote an expression block. This is the only use case for the curly braces.

```
x val { 1 + 2 }
```

# Parenthesis

Parenthesis are only used to list function parameters. They are not used for grouping expressions. For grouping expressions, or changing precedence curly braces must be used.

# Square Brackets

Square brackets are used for list literals and pattern matching on lists.

# Angle Brackets

Angle Bracket pairs are used for generic type parameters.

A single right angle bracket is used in the pipe expression to denote the value being piped.

There are no greater than or less than operators, instead there are `gt` and `lt` functions in the standard library.

# Simple Expressions

Simple expressions are expressions that do not contain any curly braces. They include:

- literals (e.g. string, number, boolean )
- identifiers (e.g. variable names, function names)
- field access (e.g. `myStruct.field`)
- function calls (e.g. `myFunction`, `myFunction arg1, { 1 + 2 }`)
- then expressions (e.g. `isTrue then 'Yes' else 'No'`)
- pipe expressions (e.g. `myValue > myFunction it`)
- match expressions (e.g. `myValue match on 0 do 'Zero' on _ do 'Non-zero'`)

# Complex Expressions

Complex expressions are expressions that contain curly braces. They are any expression that is not a simple expression.

```
myFunction { 1 + 2 }
```

```
x val {
myBoolean then ret 0

    myFunction

}
```

# Then Expression

Then expressions are a way to conditionally execute an expression based on a boolean value.

```
isTrue then 'Yes' else 'No'
```

# Pipe Expression

Pipe expressions are a way to pass the result of one expression as an argument to another expression.

```
myValue > myFunction it
```

# Function Declaration

Function declaration in Smollang is done using the `fn` keyword followed by the function name and its parameters.

Function declaration without parameters or body:

```
myFunction fn
```

Function declaration with body:

```
muFunction fn print 'Hello, World!'
```

Function declaration with parameters:

```
myFunction fn(param1) print param1
```

# Function Call

Functions can be called by using their name followed by an expression.

```
myFunction { 1 + 2 }
```

The expression must generally be wrapped in curly braces, however curly bracevites can be omitted if the expression is a simple expression.

```
# Calling a function with a string literal

print 'Hello, World!'
```

```
# Calling a function with an identifier

myFunction param1
```

If the expression is not a simple expression, curly braces must be used.

```
myFunction { 1 + 2 \* 3 }
```

```
muFunction { isGood && isShort }
```
