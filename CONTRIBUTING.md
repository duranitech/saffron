# Contributing to Saffron

Thank you for your interest in contributing to Saffron! This document provides guidelines for contributing to the project.

## Code of Conduct

This project follows the [Contributor Covenant Code of Conduct v2.1](https://www.contributor-covenant.org/version/2/1/code_of_conduct/). By participating, you agree to uphold its principles.

## How to Contribute

### Reporting Bugs

1. Check existing issues to avoid duplicates
2. Use the bug report template
3. Include: Saffron version, OS, steps to reproduce, expected vs actual behavior
4. If possible, include a minimal `.saffron` file that reproduces the issue

### Suggesting Features

1. Open a discussion in the Discussions tab first
2. For language changes, submit an RFC in the `rfcs/` repository
3. Include motivation, detailed design, and alternatives considered

### Contributing Code

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/my-feature`
3. Make your changes following our coding standards
4. Write tests for new functionality
5. Run the full test suite: `cargo test`
6. Submit a pull request

### Contributing Ingredients (SID)

1. Fork the `sid` repository
2. Add ingredient data in JSON format following the schema in `sid/schema/`
3. Include citations from authoritative sources (USDA, peer-reviewed papers, etc.)
4. Run validation: `python sid/tools/validate.py`
5. Submit a pull request

### Contributing Recipes

1. Fork the `recipes` repository
2. Write your recipe in `.saffron` format
3. Verify it compiles: `saffron check my_recipe.saffron`
4. Include a brief description as a comment
5. Submit a pull request

## Coding Standards

### Rust Code

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Run `cargo fmt` before committing
- Run `cargo clippy` and address all warnings
- Write doc comments for all public items
- Every new feature requires tests

### Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
feat(lexer): add unit literal tokenization
fix(parser): handle empty ingredient blocks
docs(sid): add contribution guide for new ingredients
test(physics): add Maillard reaction validation
refactor(ast): simplify expression enum variants
```

### Branch Naming

- `feature/description` — New features
- `fix/description` — Bug fixes
- `docs/description` — Documentation
- `refactor/description` — Code refactoring
- `test/description` — Test additions

## Development Setup

```bash
# Clone the repository
git clone https://github.com/saffron-lang/saffron.git
cd saffron

# Install Rust (if needed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build
cargo build

# Run tests
cargo test

# Run clippy
cargo clippy --all-targets

# Format code
cargo fmt
```

## RFC Process

Language changes follow a formal RFC process:

1. Create a new file in `rfcs/` following the template
2. Submit as a pull request
3. Community review period: minimum 14 days
4. Core team vote: 2/3 majority required
5. Implementation follows in the compiler repository

## Questions?

- Open a Discussion on GitHub
- Join our community channels (coming soon)

Thank you for helping make Saffron better!
