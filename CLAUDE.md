# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with 
code in this repository.

## Project Overview

This is a Rust library called `simons-parser`. It is a parser combinator library
greatly inspired by Parsec and friends in Haskell. The main difference between
this and `nom` is that we support general input stream types, rather than
only operating over text. For instance, using `simons-parser` one can write a
parser that operates over an HTML dom, or a JSON-style data structure. As long
as it can implement an iterator trait that traverses the entire structure,
`simons-parser` can parse it

## Style

Try to keep the style as functional as possible ("Ocaml with manual garbage 
collection", as opposed to "C++ with borrow checker"). Use features like 
Algebraic Data Types and Traits liberally, with an algebra-oriented design 
mindset

Always have a clear idea of what owns what. Use immutable references whenever possible

When writing new documentation files, ensure to clarify that "Documentation written 
by Claude Code" somewhere in the file.

ALL tests should be in the `tests/` directory, and should follow the snapshot 
testing instructions in the `## Testing` section.

This project is in heavy development. Whenever you make a change, make sure to 
check `CLAUDE.md` and update it if necessary to reflect any newly added/changed 
features or structures

## Error Handling & Safety Guidelines

Based on comprehensive bug audits, follow these critical safety practices:

### Never Use `unwrap()` in Production Code
- **NEVER** use `.unwrap()` on `Option` or `Result` types in production paths
- Use proper error handling with `?`, `.ok_or()`, `.map_err()`, or pattern matching
- Example: Replace `tag_name.chars().nth(1).unwrap()` with proper error handling
- Exception: Only use `unwrap()` in tests or when preceded by explicit checks that guarantee safety

### Error Message Quality
- Include contextual information in error messages
- Use structured error types instead of plain strings where possible
- Provide actionable information for debugging

## Architecture

The codebase follows a true compiler-style architecture with distinct phases of data transformation:

## Common Commands

### Development
```bash
# Run the application
just run

# Run with custom config
just run -c path/to/config.json

# Auto-recompile and run on changes (cargo watch)
just watch

# Format code using treefmt
just fmt
```

### Build & Run
```bash
# Standard cargo commands
cargo run
cargo build --release
cargo check
```

 `ollama`: Uses local Ollama installation

## Development Environment

This project uses Nix for reproducible builds and development environments. The
`flake.nix` provides all necessary dependencies including OpenSSL, libiconv, and
pkg-config. You should already be in the nix environment so no need to run `nix develop`

## Testing

The project uses **snapshot testing** via the `insta` crate for all test
assertions. This testing paradigm provides deterministic, maintainable tests
that capture expected behavior through literal value snapshots.

### Snapshot Testing Approach

All tests follow these principles:
- **Single assertion per test**: Each test has exactly one `insta::assert_snapshot!()` or `insta::assert_json_snapshot!()` call
- **Deterministic snapshots**: Dynamic data (timestamps, file sizes, temp paths) is normalized to ensure reproducible results
- **Literal value snapshots**: Snapshots contain only concrete, expected values without variables
- **Offline resilience**: All tests must pass in offline environments (CI, restricted networks) by using dual-snapshot patterns or graceful degradation

 **Golden outputs**: Reference outputs in `tests/golden_output/`

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test file
cargo test --test <test_name>

# Review and accept snapshot changes
cargo insta review

# Auto-accept all snapshot changes (use carefully)
cargo insta accept
```

### Snapshot Management

- Snapshots are stored in `src/snapshots/` (unit tests) and `tests/snapshots/` (integration tests)
- When test behavior changes, run `cargo insta review` to inspect differences
- Accept valid changes with `cargo insta accept` or reject with `cargo insta reject`
- Never commit `.snap.new` files - these are pending snapshot updates

### Deterministic Test Strategies

To ensure reproducible snapshots, the tests employ several normalization techniques:

- **Timestamp normalization**: Replace dynamic timestamps with fixed values
- **File size ranges**: Use `size > min && size < max` instead of exact sizes
- **Path abstraction**: Extract relevant path components, ignore temp directories
- **Content summarization**: Focus on structural properties rather than exact values

This approach makes tests resilient to environmental differences while maintaining strict behavioral validation.

## Notes

- Error handling follows functional programming principles - no panics in production paths
- Always have a clear idea over what should own what. Use immutable references when possible

## Performance Considerations

### Memory Management
- Avoid excessive string cloning - use string slices where possible
- Pre-allocate vectors with known capacity when feasible
- Use streaming approaches for large documents when necessary

### Network Efficiency
- HTTP clients configured with appropriate timeouts:
  - Standard requests: 30 seconds
  - Connection timeout: 10 seconds  
  - AI operations: 2 minutes
- Proper User-Agent headers for all requests
- Retry logic with exponential backoff for AI operations
