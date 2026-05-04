# Conventions

This file describes the coding conventions for the project. These conventions are intended to ensure that the code is consistent and easy to read.

## API Design

The project must follow the guidelines for API design in Rust, described in the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/design.html).

### Error Handling

Using `Result<T, E>` is the preferred way to handle errors in Rust. The `?` operator should be used to propagate errors when possible.

When prototyping, `expect()` should be used to handle errors.

- If a check made before the `expect()` makes sure that the error will never occur, then it is acceptable to use `expect()` in the final code. The error message should be descriptive and describe the assumption that is made that leads to the conclusion that the error will never occur. (e.g. `expect("Expected input to have been validated before being parsed.")`).

### Object Construction

If a struct has only required parameters, then it should have a `new(<required_parameters>)` associated function that constructs the struct.

When constructing objects, the builder pattern should be used when there is at least one optional parameter.

The struct must have a `builder(<required_parameters>)` associated function that returns a builder struct. The builder struct must have methods for setting the optional parameters, and a `build()` method that constructs the final struct.

## Naming Conventions

The project must follow the guidelines for naming conventions in Rust, described in the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/naming.html).

## Documentation

The project must follow the guidelines for documentation in Rust, described in the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/documentation.html).

Moreover, all functions, structs, struct fields, enums and enum variants must have documentation comments.

Public functions must have examples in their documentation comments.

## Tests

All public functions must have tests. Tests must be written using the [Rust testing framework](https://doc.rust-lang.org/book/ch11-01-writing-tests.html).

Tests should follow the Arrange-Act-Assert pattern, which is a common pattern for writing tests.

Test names should follow the convention of `should_<expected_behavior>_when_<condition>`, for example: `should_return_true_when_input_is_valid`.
