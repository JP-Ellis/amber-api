---
description: "Rust coding conventions and best practices"
applyTo: "**/*.rs"
---

# Rust Coding Conventions and Best Practices

Follow idiomatic Rust practices and community standards when writing Rust code.

These instructions are based on [The Rust Book](https://doc.rust-lang.org/book/), [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/), and the broader Rust community at [users.rust-lang.org](https://users.rust-lang.org).

## General Instructions

-   Always prioritize readability, clarity and maintainability
-   Use strong typing and leverage Rust's ownership system for memory safety
-   Break down complex functions into smaller, more manageable functions
-   Handle errors gracefully using `Result<T, E>` and provide meaningful error messages
-   For external dependencies, mention their usage and purpose in documentation
-   Use consistent naming conventions following [RFC 430](https://github.com/rust-lang/rfcs/blob/master/text/0430-finalizing-naming-conventions.md)
-   Use feature flags for optional functionality
-   Keep `main.rs` or `lib.rs` minimal and move logic to modules

## Patterns to Follow

-   Use modules (`mod`) and public interfaces (`pub`) to encapsulate logic
-   Handle errors properly using `?`, `match`, or `if let`, and prefer early returns over nested logic
-   Prefer the following crates for common tasks:
    -   `serde` for serialization/deserialization
    -   `snafu` for error type creation and handling
    -   `tracing` for logging and instrumentation
    -   `tokio` for asynchronous programming
    -   `bon` for builder patterns
-   Prefer enums over flags and states for type safety
-   Split binary and library code (`main.rs` vs `lib.rs`) for testability and reuse
-   Use iterators instead of index-based loops as they're often faster and safer
-   Prefer borrowing and zero-copy operations to avoid unnecessary allocations

## Patterns to Avoid

-   Don't ever use `unwrap()`, and only use `expect()` if absolutely necessary
-   Avoid panics in library code; return `Result` instead
-   Don't rely on global mutable state; use dependency injection or thread-safe containers
-   Avoid deeply nested logic; use early exits, or refactor with functions or combinators
-   Avoid `unsafe` unless required and fully documented
-   Don't overuse `clone()`, use borrowing instead of cloning unless ownership transfer is needed
-   Avoid premature `collect()`, keep iterators lazy until you actually need the collection
-   Avoid unnecessary allocations; prefer borrowing and zero-copy operations

## Code Style, Formatting and Linting

-   Use `cargo clippy` to catch common mistakes and enforce best practices. Prefer its auto-fixes where possible
-   Follow the Rust Style Guide and use `cargo fmt`/`rustfmt` for automatic formatting

## Error Handling

-   Use `Result<T, E>` for errors and only `panic!` when absolutely necessary (e.g., unrecoverable errors) or in situations where safety is guaranteed (internal/static code)
-   Prefer `?` operator over `unwrap()` or `expect()` for error propagation
-   Create custom error types using `snafu`
-   Provide meaningful error messages and context, including source errors when applicable
-   Validate function arguments and return appropriate errors for invalid input

## Testing

-   Write comprehensive unit tests using `#[cfg(test)]` modules and `#[test]` annotations
-   Use test modules alongside the code they test (`mod tests { ... }`)
-   Write integration tests in `tests/` directory with descriptive filenames
    -   Prefer organizing tests in individual directories for related integration tests
-   Ensure functions have descriptive names and include comprehensive documentation. For complex tests, consider adding comments and/or docstrings to explain the purpose and logic
-   Use `cargo nextest` for running tests
-   Use `rstest` for parameterized tests and fixtures

## API Design Guidelines

### Common Traits Implementation

Eagerly implement common traits where appropriate:

-   `Copy`, `Clone`, `Eq`, `PartialEq`, `Ord`, `PartialOrd`, `Hash`, `Debug`, `Display`, `Default`
-   Use standard conversion traits: `From`, `AsRef`, `AsMut`
-   Collections should implement `FromIterator` and `Extend`
-   Note: `Send` and `Sync` are auto-implemented by the compiler when safe; avoid manual implementation unless using `unsafe` code

### Type Safety and Predictability

-   Use newtypes to provide static distinctions
-   Arguments should convey meaning through types; prefer specific types over generic `bool` parameters
-   Use `Option<T>` appropriately for truly optional values
-   Functions with a clear receiver should be methods
-   Only smart pointers should implement `Deref` and `DerefMut`

### Future Proofing

-   Use sealed traits to protect against downstream implementations
-   Structs should have private fields, or implement `#[non_exhaustive]` if fields must be public
-   Functions should validate their arguments
-   All public types must implement `Debug`

## Documentation

-   Place function and struct documentation immediately before the item using `///`
-   Ensure all public items have rustdoc comments, often including examples (unless trivial). This includes functions, traits, structs, enums, and modules
-   Follow the [API Guidelines](https://rust-lang.github.io/api-guidelines/)
-   Document error conditions, panic scenarios, and safety considerations
-   All functions must follow the following docstring format:

    ````rust
    /// Brief description of the function.
    ///
    /// Optional detailed description of the function.
    ///
    /// # Arguments
    ///
    /// * `arg1` - Description of arg1.
    /// * `arg2` - Description of arg2.
    ///
    /// # Returns
    ///
    /// Description of the return value.
    ///
    /// # Errors
    ///
    /// Description of possible errors.
    ///
    /// # Example
    ///
    /// ```rust
    /// let result = example_function(arg1, arg2);
    /// assert_eq!(result, expected_value);
    /// ```
    fn example_function(arg1: Type1, arg2: Type2) -> Result<ReturnType> {
        // function body
    }
    ````

## Quality Checklist

Before publishing or reviewing Rust code, ensure:

### Core Requirements

-   [ ] **Naming**: Follows RFC 430 naming conventions
-   [ ] **Traits**: Implements `Debug`, `Clone`, `PartialEq` where appropriate
-   [ ] **Error Handling**: Uses `Result<T, E>` and provides meaningful error types
-   [ ] **Documentation**: All public items have rustdoc comments with examples
-   [ ] **Testing**: Comprehensive test coverage including edge cases

### Safety and Quality

-   [ ] **Safety**: No unnecessary `unsafe` code, proper error handling
-   [ ] **Performance**: Efficient use of iterators, minimal allocations
-   [ ] **API Design**: Functions are predictable, flexible, and type-safe
-   [ ] **Future Proofing**: Private fields in structs, sealed traits where appropriate
-   [ ] **Tooling**: Code passes `cargo fmt`, `cargo clippy`, and `cargo nextest`
