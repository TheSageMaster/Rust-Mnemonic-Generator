# Contributing to Rust-Mnemonic-Generator

Thank you for your interest in contributing to Rust-Mnemonic-Generator! We appreciate contributions of all kinds, whether it's reporting issues, fixing bugs, adding features, or improving documentation. Please follow the guidelines below to ensure a smooth and collaborative process.

## Table of Contents

- [How to Contribute](#how-to-contribute)
- [Reporting Issues](#reporting-issues)
- [Submitting Code Changes](#submitting-code-changes)
  - [Code Style](#code-style)
  - [Testing](#testing)
  - [Pull Request Process](#pull-request-process)
- [Community Guidelines](#community-guidelines)

## How to Contribute

We welcome all contributions, including bug reports, feature requests, documentation improvements, and code contributions.

### Reporting Issues

If you find a bug or have a suggestion for a new feature, please open an issue in the [issue tracker](https://github.com/TheSageMaster/Rust-Mnemonic-Generator/issues). When reporting an issue, please include:

- A clear description of the problem or feature request.
- Steps to reproduce the issue (if applicable).
- Any error messages, logs, or screenshots if relevant.

### Submitting Code Changes

To contribute code, please follow these steps:

1. **Fork the repository**  
   Click the "Fork" button at the top of this page to create a personal copy of the repository.

2. **Clone your fork**  
   Clone your forked repository to your local machine:
   ```bash
   git clone https://github.com/TheSageMaster/Rust-Mnemonic-Generator.git
   ```

3. **Create a new branch**  
   Create a new branch for your changes:
   ```bash
   git checkout -b my-feature-branch
   ```

4. **Make your changes**  
   Implement your feature or bug fix. Be sure to follow the Rust coding conventions.

5. **Commit your changes**  
   Commit your changes with clear and concise commit messages:
   ```bash
   git add .
   git commit -m "Describe your changes here"
   ```

6. **Push your changes**  
   Push your changes to your forked repository:
   ```bash
   git push origin my-feature-branch
   ```

7. **Open a Pull Request**  
   Open a pull request to merge your changes into the main repository. Provide a detailed description of your changes and reference any issues that they address (e.g., `Closes #123`).

### Code Style

This project follows the [Rust style guide](https://github.com/rust-lang/rfcs/blob/master/text/0000-style-guide.md). Please make sure your code adheres to the following conventions:

- Follow the [Rustfmt](https://github.com/rust-lang/rustfmt) code formatting tool for consistent code style.
  - You can automatically format your code by running:
    ```bash
    cargo fmt
    ```
- Use meaningful and descriptive names for variables, functions, and structs.
- Write documentation comments for public items using `///` for doc comments.
- Ensure that your code is idiomatic and efficient, in line with Rust's ownership and concurrency principles.

### Testing

We rely on tests to ensure the quality and correctness of the code. Please follow these guidelines when working with tests:

- Ensure that your changes don’t break existing functionality by running the tests:
  ```bash
  cargo test
  ```
- Add tests for new features or bug fixes to ensure stability and prevent regressions.
- Write tests in a separate module marked with `#[cfg(test)]`.

### Pull Request Process

- Ensure that your pull request is **up-to-date** with the main branch (usually `main` or `master`).
- Provide a clear description of what your pull request does, and reference any related issues (e.g., `Closes #123`).
- Make sure your pull request passes all tests and formatting checks before submitting.
- Be open to feedback and be willing to make changes based on review comments.

## Community Guidelines

We strive to create a positive and inclusive environment. Please keep the following in mind:

- Be respectful and considerate of others.
- Engage constructively with feedback and other contributors.
- Use inclusive and non-offensive language.
- Report any instances of harassment or abusive behavior to the maintainers.

We ask that all contributors abide by the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct) while participating in the project.

---

Thank you for your interest in contributing! We look forward to your contributions and feedback.

---
