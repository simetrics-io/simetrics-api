<br />
<br />
<p align="center">
<img src="https://avatars.githubusercontent.com/u/196379875?s=200&v=4" width="240" alt="simetrics">
</p>
<br />
<br />

## Reference implementation

[![test](https://github.com/simetrics-io/tokenomics-api/actions/workflows/test-workflow.yml/badge.svg)](https://github.com/simetrics-io/tokenomics-api/actions/workflows/test-workflow.yml)
[![build](https://github.com/simetrics-io/tokenomics-api/actions/workflows/build-workflow.yml/badge.svg?branch=main)](https://github.com/simetrics-io/tokenomics-api/actions/workflows/build-workflow.yml)

## About simetrics

Simetrics is a tool for simulating the tokenomics of a project.

It allows users to simulate trades, calculate various metrics, and predict user behaviour over different time intervals.

## Prerequisites

Before you begin, ensure you have met the following requirements:

- You have installed [Docker](https://www.docker.com/).
- You have installed [Rust](https://doc.rust-lang.org/cargo/getting-started/installation.html).
- You have installed [Cargo Watch](https://crates.io/crates/cargo-watch).

## Getting Started

### Clone the repository

```bash
git clone git@github.com:simetrics-io/tokenomics-simulator-rs.git
```

### Set up environment variables

```bash
cp .env.dev .env
```

### Run Docker

```bash
docker-compose up -d
```

### Start an application

> To start in the watch mode, please use: `cargo watch -x 'run -p tokenomics-simulator-api'`.

```bash
cargo run -p $SERVICE_NAME
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

To review and accept/reject snapshots, install [cargo-insta](https://crates.io/crates/cargo-insta) and run:

```bash
cargo-insta review
```

### Check the codebase

Catch common mistakes and improve the overall code quality.

```bash
cargo clippy --all-targets --all-features --no-deps -- -D warnings
```
