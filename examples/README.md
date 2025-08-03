# Amber API Examples

This directory contains practical examples demonstrating how to use the Amber Electric API client.

## Prerequisites

Most examples require an API key. Set your API key in the environment variable:

```bash
export AMBER_API_KEY="your-api-key-here"
```

```pwsh
$env:AMBER_API_KEY="your-api-key-here"
```

You can get your API key from the [developer section of the Amber Electric web app](https://app.amber.com.au/developers/).

## Available Examples

### Site Management

-   **`get_sites`**: Get all electricity sites linked to your account

    ```console
    cargo run --example get_sites
    ```

### Renewable Energy Data

-   **`renewables`**: Get current renewable energy percentage for Victoria with default 30-minute resolution

    ```console
    cargo run --example renewables
    ```

-   **`renewables_with_resolution`**: Get SA renewable data with 5-minute resolution

    ```console
    cargo run --example renewables_with_resolution
    ```

-   **`renewables_complete`**: Comprehensive example retrieving historical, current, and forecast renewable data

    ```console
    cargo run --example renewables_complete
    ```

### Price and Usage Data

-   **`current_prices`**: Get current price intervals for your first site

    ```console
    cargo run --example current_prices
    ```

-   **`current_prices_complete`**: Comprehensive example retrieving previous, current, and next price intervals for your first site

    ```console
    cargo run --example current_prices_complete
    ```

-   **`prices`**: Get historical price intervals for your first site with all optional arguments

    ```console
    cargo run --example prices
    ```

-   **`usage`**: Get usage data for your first site for a given date range

    ```console
    cargo run --example usage
    ```

## Contributing

When adding new examples:

1.  Follow the existing naming convention: `<feature>.rs` or `<feature>_<variation>.rs`.
2.  Include comprehensive documentation in the file header and follow the structure and patterns of existing examples.
3.  Update this README with the new example
