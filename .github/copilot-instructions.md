# Conventions

This file describes the coding conventions for the project. These conventions are intended to ensure that the code is consistent and easy to read.

## API Design

The project must follow the guidelines for API design in Rust, described in the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/design.html).

### Error Handling

Using `Result<T, E>` is the preferred way to handle errors in Rust. The `?` operator should be used to propagate errors when possible.

A function returning `Result<T, E>` should define a custom error type `E` that is an enum with variants for each possible error that can occur in the function.

When prototyping, `expect()` could be used to handle errors.

- If a check made before the `expect()` makes sure that the error will never occur, then it is acceptable to use `expect()` in the final code. The error message should be descriptive and describe the assumption that is made that leads to the conclusion that the error will never occur. (e.g. `expect("Expected input to have been validated before being parsed.")`).

### Object Construction

If a struct has only required parameters, then it should have a `new(<required_parameters>)` associated function that constructs the struct.

When constructing objects, the builder pattern should be used when there is at least one optional parameter.

The struct must have a `builder(<required_parameters>)` associated function that returns a builder struct. The builder struct must have methods for setting the optional parameters, and a `build()` method that constructs the final struct.

## Naming Conventions

The project must follow the guidelines for naming conventions in Rust, described in the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/naming.html).

## Asynchronous Code and Concurrency

When using asynchronous code in Rust, prefer `async` functions and the `await` syntax over callbacks or manual future handling.

Error handling in async contexts should follow the same conventions as synchronous code. Use `Result<T, E>` and the `?` operator to propagate errors from async functions.

## Documentation

The project must follow the guidelines for documentation in Rust, described in the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/documentation.html).

Moreover, all functions, structs, struct fields, enums and enum variants must have documentation comments.

Public functions must have examples in their documentation comments.

## Tests

All public functions must have tests. Tests must be written using the [Rust testing framework](https://doc.rust-lang.org/book/ch11-01-writing-tests.html).

Tests should follow the Arrange-Act-Assert pattern, which is a common pattern for writing tests.

Test names should follow the convention of `should_<expected_behavior>_when_<condition>`, for example: `should_return_true_when_input_is_valid`.

## Using `use::<module>::*`

The `use` syntax can be used to import all variants of an enum within the current scope.

This can be useful when using the variants of an error enum within a function that returns a `Result<T, E>` where `E` is the error enum. This allows for using the variants of the error enum without having to prefix them with the enum name.

When using `use::<module>::*`, it should be used at the beginning of the function where the variants are used, and it should be used in a way that does not cause confusion about where the variants are coming from.

Example:

```rust
fn example_function() -> Result<(), MyError> {
    use MyError::*;

    if some_condition {
        return Err(VariantOne);
    } else {
        return Err(VariantTwo);
    }
}
```
