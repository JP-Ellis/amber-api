# Amber API

<!-- markdownlint-disable no-inline-html -->
<div align="center">
    <img src="logo.svg" alt="Amber API Logo" height="200" align="left" hspace="20">
    <span>
        <b>
            A Rust client library for the Amber Electric API, providing easy access to renewable energy data and pricing information.
        </b>
    </span>
</div>

<div align="center"><table>
    <tr>
        <td>Package</td>
        <td>
            <a href="https://crates.io/crates/amber-api"><img src="https://img.shields.io/crates/v/amber-api.svg" alt="Version"></a>
            <a href="https://crates.io/crates/amber-api"><img src="https://img.shields.io/crates/d/amber-api.svg" alt="Downloads"></a>
            <a href="https://docs.rs/amber-api"><img src="https://docs.rs/amber-api/badge.svg" alt="Documentation"></a>
        </td>
    </tr>
    <tr>
        <td>CI/CD</td>
        <td>
            <a
                href="https://github.com/JP-Ellis/amber-api/actions/workflows/deploy.yml"><img
                src="https://img.shields.io/github/actions/workflow/status/JP-Ellis/amber-api/deploy.yml?branch=main&label=CI"
                alt="CI Status"></a>
            <a
                href="https://github.com/JP-Ellis/amber-api/actions/workflows/test.yml"><img
                src="https://img.shields.io/github/actions/workflow/status/JP-Ellis/amber-api/test.yml?branch=main&label=tests"
                alt="Test Status"></a>
        </td>
    </tr>
    <tr>
        <td>Meta</td>
        <td>
            <a
                href="https://github.com/rust-lang/cargo"><img
                src="https://img.shields.io/badge/🦀-Cargo-blue.svg"
                alt="Cargo project"></a>
            <a href="https://github.com/rust-lang/rustfmt"><img
                src="https://img.shields.io/badge/code%20style-rustfmt-brightgreen.svg"
                alt="Code style - rustfmt"></a>
            <a href="https://github.com/rust-lang/rust-clippy"><img
                src="https://img.shields.io/badge/linting-clippy-blue.svg"
                alt="Linting - Clippy"></a>
            <a
                href="https://opensource.org/licenses/MIT"><img
                src="https://img.shields.io/badge/License-MIT-green.svg"
                alt="License"></a>
        </td>
    </tr>
    <tr>
        <td>Community</td>
        <td>
            <a
                href="https://github.com/JP-Ellis/amber-api/issues"><img
                src="https://img.shields.io/github/issues/JP-Ellis/amber-api.svg"
                alt="Issues"></a>
            <a
                href="https://github.com/JP-Ellis/amber-api/discussions"><img
                src="https://img.shields.io/github/discussions/JP-Ellis/amber-api.svg"
                alt="Discussions"></a>
            <a
                href="https://github.com/JP-Ellis/amber-api"><img
                src="https://img.shields.io/github/stars/JP-Ellis/amber-api.svg?style=social"
                alt="GitHub Stars"></a>
        </td>
    </tr>
</table></div>
<!-- markdownlint-enable no-inline-html -->

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
amber-api = "~1"
```

## Quick Start

```rust
use amber_api::Amber;

// Create a client with your API key
let client = Amber::builder()
    .api_key("your-api-key-here") // Prefer setting AMBER_API_KEY in an environment variable
    .build()?;

// Get all your electricity sites
let sites = client.sites()?;
println!("Found {} sites", sites.len());

// Get renewable energy data for Victoria
let renewables = client.renewables().state("vic").call()?;
println!("Current renewable: {}%", renewables.percentage);
```

## Authentication

You'll need an API key from [Amber Electric](https://app.amber.com.au/developers/). You can provide it in several ways:

### Environment Variable (Recommended)

```bash
export AMBER_API_KEY="your-api-key-here"
```

Then use the default client:

```rust
let client = Amber::default();
```

### Direct Configuration

```rust
let client = Amber::builder()
    .api_key("your-api-key-here")
    .build()?;
```

## Examples

Check out the [examples directory](./examples/) for comprehensive usage examples. You can run them directly using Cargo and they will demonstrate various API features.

Most of the examples require that the `AMBER_API_KEY` environment be set, with the exception of the renewables:

```bash
cargo run --example renewables
```

## API Coverage

This library provides access to:

-   **Sites API**: Manage your electricity sites
-   **Renewables API**: Access renewable energy data with configurable resolution
-   **Pricing API**: Real-time and forecast pricing information
-   **Usage API**: Historical and current usage data

For detailed API documentation, visit the [Amber Electric API docs](https://app.amber.com.au/developers/).

## Documentation

-   [API Documentation](https://docs.rs/amber-api)
-   [Amber Electric Website](https://amber.com.au/)
-   [Amber Electric API Documentation](https://app.amber.com.au/developers/)

## Contributing

We welcome contributions! Please see our [Contributing Guide](./CONTRIBUTING.md) for details on:

-   Setting up the development environment
-   Running tests and examples
-   Code style and formatting guidelines
-   Submitting pull requests

## Testing

Run the test suite:

```bash
# Run tests with nextest (faster)
cargo nextest run

# Run integration tests
cargo test --test integration
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Support

-   📚 [Documentation](https://docs.rs/amber-api)
-   🐛 [Issue Tracker](https://github.com/JP-Ellis/amber-api/issues)
-   💬 [Discussions](https://github.com/JP-Ellis/amber-api/discussions)
