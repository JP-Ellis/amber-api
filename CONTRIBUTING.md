# Contributing to Amber API

Amber API is a Rust client library for the [Amber Electric](https://amber.com.au/) API. If you're interested in contributing to Amber API, hopefully, this document makes the process for contributing clear.

The [Open Source Guides](https://opensource.guide/) website has a collection of resources for individuals, communities, and companies who want to learn how to run and contribute to an open source project. Contributors and people new to open source alike will find the following guides especially useful:

-   [How to Contribute to Open Source](https://opensource.guide/how-to-contribute/)
-   [Building Welcoming Communities](https://opensource.guide/building-community/)

## Get Involved

There are many ways to contribute to Amber API. Here's a few ideas to get started:

-   Look through the [open issues](https://github.com/JP-Ellis/amber-api/issues). Provide workarounds, ask for clarification, or suggest labels. Help [triage issues](#triaging-issues-and-pull-requests).
-   If you find an issue you would like to fix, [open a pull request](#pull-requests). Issues tagged as [_Good first issue_](https://github.com/JP-Ellis/amber-api/labels/good%20first%20issue) are a good place to get started.
-   Read through the [docs](https://docs.rs/amber-api). If you find anything that is confusing or can be improved, you can open a pull request to improve the documentation.
-   Take a look at the [features requested](https://github.com/JP-Ellis/amber-api/labels/feature) by others in the community and consider opening a pull request if you see something you want to work on.

Contributions are very welcome. If you think you need help planning your contribution, please open a discussion on GitHub and let us know you are looking for a bit of help.

### Join our Community

You can start [discussions](https://github.com/JP-Ellis/amber-api/discussions) on GitHub for broader questions about the library and its development.

### Triaging Issues and Pull Requests

One great way you can contribute to the project without writing any code is to help triage issues and pull requests as they come in.

-   Ask for more information if you believe the issue does not provide all the details required to solve it.
-   Suggest [labels](https://github.com/JP-Ellis/amber-api/labels) that can help categorize issues.
-   Flag issues that are stale or that should be closed.
-   Ask for test plans and review code.

## Our Development Process

Amber API uses [GitHub](https://github.com/JP-Ellis/amber-api) as its source of truth. All changes will be public from the beginning.

All pull requests will be checked by the continuous integration system, GitHub actions. There are unit tests, integration tests, linting checks, formatting checks, and much more.

### Branch Organization

Amber API has one primary branch `main` and we use feature branches to deliver new features with pull requests. Typically, we scope the branch according to the [conventional commit](#conventional-commit-messages) categories. The more common ones are:

-   `feat/<n>` or `feature/<n>` for new features
-   `fix/<n>` for bug fixes
-   `chore/<n>` for chores
-   `docs/<n>` for documentation changes

## Issues

When [opening a new issue](https://github.com/JP-Ellis/amber-api/issues/new), always make sure to fill out the issue template when available. **This step is very important!** Not doing so may slow down the response. Don't take this personally if this happens, and feel free to open a new issue once you've gathered all the information required by the template.

**Please don't use the GitHub issue tracker for questions.** If you have questions about using Amber API, prefer the [Discussion pages](https://github.com/JP-Ellis/amber-api/discussions), and we will do our best to answer your questions.

### Bugs

We use [GitHub Issues](https://github.com/JP-Ellis/amber-api/issues) for our public bugs. If you would like to report a problem, take a look around and see if someone already opened an issue about it. If you are certain this is a new, unreported bug, you can submit a bug report.

-   **One issue, one bug:** Please report a single bug per issue.
-   **Provide reproduction steps:** List all the steps necessary to reproduce the issue. The person reading your bug report should be able to follow these steps to reproduce your issue with minimal effort.

If you're only fixing a bug, it's fine to submit a pull request right away but we still recommend filing an issue detailing what you're fixing. This is helpful in case we don't accept that specific fix but want to keep track of the issue.

### Feature requests

If you would like to request a new feature or enhancement but are not yet thinking about opening a pull request, you can file an issue with a feature request for more thought out ideas.

### Claiming issues

We have a list of [beginner-friendly issues](https://github.com/JP-Ellis/amber-api/labels/good%20first%20issue) to help you get your feet wet in the Amber API codebase and familiar with our contribution process. This is a great place to get started.

Apart from the `good first issue`, it is also worth looking at the [`help wanted`](https://github.com/JP-Ellis/amber-api/labels/help%20wanted) issues. If you have specific knowledge in one domain, working on these issues can make your expertise shine.

If you want to work on any of these issues, just drop a message saying "I'd like to work on this", and we will assign the issue to you and update the issue's status as "claimed".

Once an issue is claimed, we hope to see a pull request; however we understand that life happens and you may not be able to complete the issue. If you are unable to complete the issue, please let us know so we can unassign the issue and make it available for others to work on.

The claiming process is there to help ensure effort is not wasted. Even if you are not sure whether you can complete the issue, claiming it will help us know that someone is working on it. If you are not sure how to proceed, feel free to ask for help.

## Development

### Prerequisites

1.  Ensure you have [Rust](https://rustup.rs/) installed (latest stable version recommended).
2.  Ensure you have [Git](https://git-scm.com/) installed.
3.  An [Amber Electric API key](https://app.amber.com.au/developers/) for testing.

### Installation

1.  Fork the repository on GitHub.

2.  Clone your fork locally:

    ```bash
    git clone https://github.com/your-username/amber-api.git
    cd amber-api
    ```

3.  Set up your API key for testing:

    ```bash
    export AMBER_API_KEY="your-api-key-here"
    ```

4.  Build the project:

    ```bash
    cargo build
    ```

5.  Run the tests to make sure everything is working:

    ```bash
    cargo test
    ```

6.  Try running an example to make sure the API integration works:

    ```bash
    cargo run --example get_sites
    ```

### Code Conventions

-   **Most important: Look around.** Match the style you see used in the rest of the project. This includes formatting, naming files, naming things in code, naming things in documentation, etc.
-   We use `cargo fmt` for formatting and `cargo clippy` for linting to catch most stylistic problems. If you are working locally, these should be run before committing.
-   All public APIs must be documented with rustdoc comments.
-   Use descriptive variable and function names.
-   Follow Rust naming conventions (snake_case for functions/variables, PascalCase for types).

Don't worry too much about styles in general—the maintainers will help you fix them as they review your code.

To help catch a lot of simple formatting or linting issues, you can run `hatch run lint` to run the linter and `hatch run format` to format your code. This process can also be automated by installing [`pre-commit`](https://pre-commit.com/) hooks:

```sh
pre-commit install
```

## Pull Requests

So you are considering contributing to Amber API's code? Great! We'd love to have you. First off, please make sure it is related to an existing issue. If not, please open a new issue to discuss the problem you are trying to solve before investing a lot of time into a pull request. While we do accept PRs that are not related to an issue (especially if the PR is very simple), it is best to discuss it first to avoid wasting your time.

Once you have opened a PR, we will do our best to work with you and get the PR looked at.

Working on your first Pull Request? You can learn how from this free video series:

[**How to Contribute to an Open Source Project on GitHub**](https://egghead.io/courses/how-to-contribute-to-an-open-source-project-on-github)

Please make sure the following is done when submitting a pull request:

1.  **Keep your PR small.** Small pull requests (~300 lines of diff) are much easier to review and more likely to get merged. Make sure the PR does only one thing, otherwise please split it.
2.  **Use descriptive titles.** It is recommended to follow this [commit message style](#conventional-commit-messages).
3.  **Test your changes.** Describe your [**test plan**](#test-plan) in your pull request description.

All pull requests should be opened against the `main` branch.

We have a lot of integration systems that run automated tests to guard against mistakes. The maintainers will also review your code and may fix obvious issues for you. These systems' duty is to make you worry as little about the chores as possible. Your code contributions are more important than sticking to any procedures, although completing the checklist will surely save everyone's time.

### Conventional Commit Messages

Amber API has adopted the [Conventional Commit](https://www.conventionalcommits.org/en/v1.0.0/) convention and we use it to generate our changelog and in the automation of our release process.

The format is:

```text
<type>(<scope>): <subject>
```

`<scope>` is optional. If your change is specific to one/two modules, consider adding the scope. Scopes should be brief but recognizable, e.g. `client`, `models`, `docs`, `ci`, etc. You can take a quick look at the Git history (`git log`) to get the gist.

The various types of commits:

-   `feat`: a new API or behavior **for the end user**.
-   `fix`: a bug fix **for the end user**.
-   `docs`: a change to documentation or other Markdown documents in our repo.
-   `style`: a change to production code that leads to no behavior difference, e.g. splitting files, renaming internal variables, improving code style...
-   `test`: adding missing tests, refactoring tests; no production code change.
-   `chore`: upgrading dependencies, releasing new versions... Chores that are **regularly done** for maintenance purposes.
-   `refactor`: code changes that neither fix bugs nor add features.
-   `perf`: performance improvements.

### Test Plan

A good test plan has the exact commands you ran and their output.

Tests are integrated into our continuous integration system, so you don't always need to run local tests. However, for significant code changes, it saves both your and the maintainers' time if you can do exhaustive tests locally first to make sure your PR is in good shape. There are many types of tests:

-   **Build and format check.** We use `cargo fmt` and `cargo clippy` in our codebase, which can make sure your code is consistent and catches some obvious mistakes early.
-   **Unit tests.** You can run `cargo test` in the root directory to run all tests, and `cargo nextest run` for faster test execution.
-   **Integration tests.** Run `cargo test --test integration` to test real API integration (requires API key).
-   **Examples.** Test that examples work with `cargo run --example <example_name>`.

### Licensing

By contributing to Amber API, you agree that your contributions will be licensed under its MIT license.

### Breaking Changes

When adding a new breaking change, follow this template in your pull request:

```md
### New breaking change here

-   **Who does this affect**:
-   **How to migrate**:
-   **Why make this breaking change**:
-   **Severity (number of people affected x effort)**:
```

### What Happens Next?

The team will be monitoring pull requests. Do help us by keeping pull requests consistent by following the guidelines above.

## API Design Guidelines

When adding new API endpoints:

1.  **Follow Rust naming conventions** (snake_case for functions, PascalCase for types)
2.  **Use builder patterns** for complex parameter sets
3.  **Return typed structs** instead of raw JSON
4.  **Handle errors gracefully** with appropriate error types
5.  **Add comprehensive documentation** with examples

## Getting Help

If you need help or have questions:

-   📚 Check the [documentation](https://docs.rs/amber-api)
-   💬 Start a [discussion](https://github.com/JP-Ellis/amber-api/discussions)
-   🐛 [Open an issue](https://github.com/JP-Ellis/amber-api/issues) for bugs

## Code of Conduct

This project follows the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct). By participating, you are expected to uphold this code.
