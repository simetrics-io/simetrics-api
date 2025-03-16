# Simetrics API

## Reference implementation

[![test](https://github.com/simetrics-io/simetrics-api/actions/workflows/test.yml/badge.svg)](https://github.com/simetrics-io/simetrics-api/actions/workflows/test.yml)

## Prerequisites

Before you begin, ensure you have met the following requirements:

- You have installed [Rust](https://doc.rust-lang.org/cargo/getting-started/installation.html).
- You have installed [Cargo Watch](https://crates.io/crates/cargo-watch).

## Getting Started

### Clone the repository

```bash
git clone https://github.com/simetrics-io/simetrics-api.git
```

### Set up environment variables

```bash
cp .env.dev .env
```

### Start an application

```bash
cargo watch -x 'run -p simetrics-api'
```

### Run tests

> To run both unit & integration tests, use the following command: `cargo test`

To run only unit tests, use the following command:

```bash
cargo test --lib
```

To run only integration tests, use the following command:

```bash
cargo test --test '*'
```

To run only one specific test, use the following command and replace `test_name` with the name of the test:

```bash
cargo test --test test_name
```

To run tests in watch mode, use the following command:

```bash
cargo watch -x test
```

To run tests with logs, use the following command:

```bash
cargo test -- --nocapture
```

### Check the codebase

Catch common mistakes and improve the overall code quality.

```bash
cargo clippy --all-targets --all-features --no-deps -- -D warnings
```
