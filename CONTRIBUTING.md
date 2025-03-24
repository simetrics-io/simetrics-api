# Contributing to Simetrics API

Thank you for your interest in contributing to the Simetrics API!

We welcome contributions from the community and appreciate your efforts to improve the API.

## How to Contribute

There are several ways you can contribute to the project:

### Reporting Bugs

If you encounter any bugs, please [submit an issue](https://github.com/simetrics-io/simetrics-api/issues) with detailed information about the problem and steps to reproduce it. Include any relevant logs or screenshots that can help us understand the issue.

### Feature Requests

If you have ideas for new features, feel free to [submit an issue](https://github.com/simetrics-io/simetrics-api/issues) with a detailed description of the feature and its potential use cases. We appreciate your input and will consider your suggestions for future releases.

### Code of Conduct

Please note that this project is released with a Contributor Code of Conduct. By participating in this project, you agree to abide by its terms.

### Submitting Pull Requests

We welcome pull requests for bug fixes, new features, and improvements. To submit a pull request, follow these steps:

1. **Fork the repository**: Click the "Fork" button at the top right corner of the repository page to create a copy of the repository in your GitHub account.

2. **Clone the repository**: Clone your forked repository to your local machine using the following command:

    ```sh
    git clone git@github.com:simetrics-io/simetrics-api.git
    ```

3. **Create a new branch**: Create a new branch for your changes using the following command:

    ```sh
    git checkout -b my-feature-branch
    ```

4. **Make your changes**: Make the necessary changes to the codebase. Ensure that your code follows the project's coding standards and includes appropriate tests.

5. **Commit your changes**: Commit your changes with a descriptive commit message using the following command:

    ```sh
    git commit -m "feat: description of my changes"
    ```

6. **Push your changes**: Push your changes to your forked repository using the following command:

    ```sh
    git push origin my-feature-branch
    ```

7. **Create a pull request**: Go to the original repository and click the `New pull request` button. Select your branch and provide a detailed description of your changes. Submit the pull request for review.

### Building the API

To build the API, run the following command:

```sh
cargo build
```

### Testing the API

To run the tests, use:

```sh
cargo test
```

### Code Quality Checks

Run [clippy](https://github.com/rust-lang/rust-clippy) to lint the code:

```sh
cargo clippy --locked --all-targets --all-features --no-deps -- -D warnings
```

Run [rustfmt](https://github.com/rust-lang/rustfmt) to format the code:

```sh
cargo fmt
```

### Documentation

Generate documentation in HTML format:

```bash
cargo doc --open
```

## Getting Help

If you need help or have any questions, feel free to submit an issue or reach out to the maintainers.

Thank you for contributing to Simetrics API!
