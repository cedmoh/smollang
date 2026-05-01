# Syntax

## Comments

Single-Line Comment:

```
# this is a single line comment
```

Multi-Line Comment:

```
###
This is a multi line comment
Also used for documentation
###
```

Inline Comment:

```
## this is an inline comment ##
```

## Parenthesis

Parenthesis are used to group expressions and construct complex expressions.

## Curly Braces

Curly braces are used to denote objects.

Example:

```
person val {
  name 'Alice',
  age 30,
  isStudent false,

  salary getSalary(),

  study || print 'Studying...',
}
```

## Square Brackets

Square brackets are used to denote arrays.

Example:

```
numbers val [1, 2, 3, 4, 5]
```

Square brackets cannot be used for array indexing. Instead either the `get` function from the standard library or field access with a dynamic field name can be used.

```
numbers.get 0
```

```
numbers.(index)
```

## Angle Brackets

Use cases for angle brackets include:

- generic type parameters in function declarations
- pipe expressions; a single right angle bracket is used to denote the value being piped

There are no greater than or less than operators, instead there are `gt` and `lt` functions in the standard library.

# Operators

## Unary Operators

### Arithmetic

Negation: `-`

### Logical

Logical Not: `!`

## Binary Operators

Assignment: `=`

### Arithmetic

Addition: `+`

Subtraction: `-`

Multiplication: `*`

Division: `/`

Modulus: `%`

Exponent: `^`

### Arithmetic Assignment

Addition Assignment: `+=`

Subtraction Assignment: `-=`

Multiplication Assignment : `*=`

Division Assignment: `/=`

Modulus Assignment: `%=`

### Comparison

Equal to: `==`

Not Equal to: `!=`

### Logical

Logical AND: `and`

Logical OR: `or`

# Data Types

## Scalar

### Boolean

True value: `true`

False Value: `false`

Boolean Value in Literal Assignment:

```
x val false
```

Boolean Value in Assignment:

```
x val y
```

### Numeric

#### Integer

Integer in Literal Assignment:

```
x val 42
```

```
y val -100_234
```

#### Decimal

Decimal in Literal Assignment:

```
offsetX val 99.95
```

```
offsetY val -1_234.005
```

#### Hexadecimal

Hexadecimal in Literal Assignment:

```
color val #FF0099
```

```
alpha val #000000FF
```

#### Binary

Binary in Literal Assignment:

```
num val b0010
```

```
mask val b_1010100
```

#### Octal

Octal in Literal Assignment:

```
octalValue val o0755
```

```
octalValue val o_0755
```

## Compound

### String

String in Literal Assignment:

```
msg val 'Hello, World!'
```

String with Escape Sequences:

```
msg val 'Hello, \nWorld!'
```

String with Unicode Characters:

```
msg val 'Hello, 世界!'
```

# Simple Expressions

Simple expressions are expressions that do not contain any parenthesis. They include:

- literals (e.g. string, number, boolean, array)
- identifiers (e.g. variable names, function names)
- field access (e.g. `myStruct.field`)
- function calls (e.g. `myFunction()`, `myFunction(arg1, 1 + 2)`)
- then expressions (e.g. `isTrue then 'Yes' else 'No'`)
- pipe expressions (e.g. `myValue |> myFunction it`)
- match expressions (e.g. `myValue match 0 do 'Zero' _ do 'Non-zero'`)

# Complex Expressions

They are any expression that is not a simple expression.

```
myFunction(1 + 2)
```

```
x val (
  myBoolean then ret 0

  myFunction
)
```

# Then Expression

Then expressions are a way to conditionally execute an expression based on a boolean value.

```
isTrue then 'Yes' else 'No'
```

# Pipe Expression

Pipe expressions are a way to pass the result of one expression as an argument to another expression.

```
myValue to myFunction it
```

Nested pipe expressions inside blocks and using symbols instead of the `to` keyword:

```
1
|> 2
|> 3
|> 4
|> (
    a |> b |> c |> d
  )
|> 5
|> 6
```

# Match Expression

Match expressions are a way to pattern match on a value and execute different expressions based on the pattern.

```
myValue match
  0 do 'Zero',
  1 do 'One',
  _ do 'Other'
```

With symbols instead of keywords:

```
myValue ~=
  1 -> 1
  2 -> 'bye'
  3 -> ()
  4 -> print 'hello'
  5 -> hello
```

# Field Access

Field access is a way to access the fields of an object.

```
myObject.field
```

Field access can also be used to access the fields of an object returned by a function call.

```
myFunction().field
```

Field access can be done with a dynamic field name as well:

```
myObject.(fieldName)
```

# Operator Precedence

**TODO**

# Variables

Immutable Uninitialized Variable Declaration:

```
x val
```

**NOTE:** Immutable variable declaration without initialization is illegal and will not compile.

**NOTE:** Uninitialized variable declaration requires the type to include `nil`.

Immutable Variable Initialization with Literal:

```
x val 'hello'
```

Mutable Variable Declaration Initialized with Literal:

```
x var 'hello'
```

Variable Initialization with complex Expression:

```
x val ( 2 + 2 )
```

```
x val (2 * 3) + (4 / 2)
```

Variable Initialization with Multiline Block Expression:

```
x val (
  2 + 2
)
```

Variable Initialization with Function Call:

```
x val myFunction
```

Variable Initialization with Identifier:

```
x val y
```

# Functions

## Function Declaration

Function declaration without parameters or body:

```
myFunction ||
```

Function declaration with body:

```
myFunction || print 'Hello, World!'
```

Function declaration with parameters:

```
myFunction |param1, param2| print(param1, param2)
```

## Function Call

Functions can be called using their name followed by an expression.

```
myFunction 'Hello, World!'
```

If the function has no parameters, the call can be made with parenthesis.

```
myFunctionWithNoParameters()
```

If the function has one parameter and the argument is a simple expression, the call can be made without parenthesis.

```
myFunctionWithOneSimpleParameter isGood then 'Good' else 'Bad'
```

If the function has multiple parameters or the argument is not a simple expression, the call must be made with parenthesis.

```
myFunctionWithOneComplexParameter(1 + 2)

myFunctionWithMultipleSimpleParameters(param1, 123, 'Hello')
```

Calling a function with a string literal

```
print 'Hello, World!'
```

Calling a function with an identifier

```
myFunction param1
```

Calling a function with a field access

```
myFunction myObject.field
```

Calling a function with a function call as an argument

```
myFunction anotherFunction(arg1, arg2)
```
