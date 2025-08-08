# simons-parser

A functional parser combinator library for Rust, inspired by Parsec and Haskell's parser combinators.

## Notice

Claude Code with Sonnet 4 basically wrote this entire project, including the README. I'm pleasantly surprised by the results, but I would recommend against using it in production.

## Overview

`simons-parser` is a parser combinator library that takes a functional approach to parsing with a key differentiator: **support for general input stream types**, not just text. While libraries like `nom` are excellent for text parsing, `simons-parser` allows you to parse any structure that implements the required iterator traits.

### Key Features

- **Generic Input Types**: Parse HTML DOM, JSON structures, token streams, or any iterable data
- **Functional Design**: Immutable, composable parsers with algebraic data types
- **Haskell-Inspired API**: Familiar combinators for those coming from Parsec
- **Zero-Copy Parsing**: Efficient parsing with minimal allocations
- **Rich Error Messages**: Contextual error reporting with position information

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
simons-parser = "0.1.0"
```

### Basic Text Parsing

```rust
use simons_parser::*;

// Parse a single character
let parser = char('h');
let result = parser.parse("hello");
// Ok(('h', "ello"))

// Parse a string literal
let parser = string("hello");
let result = parser.parse("hello world");
// Ok(("hello", " world"))

// Parse numbers
let parser = integer();
let result = parser.parse("-42abc");
// Ok((-42, "abc"))
```

### Combinator Composition

```rust
use simons_parser::*;

// Parse whitespace-separated words
let word = many1(alpha());
let spaces = many(space());
let parser = word.then_skip(spaces).then(word);

// Parse optional elements
let maybe_sign = optional(char('-'));
let number = maybe_sign.then(unsigned());
```

### Beyond Text: Parsing Arbitrary Data

```rust
// Example: Parse a custom token stream
#[derive(Debug, PartialEq)]
enum Token {
    Number(i32),
    Plus,
    Minus,
}

let tokens = vec![Token::Number(1), Token::Plus, Token::Number(2)];
let number_parser = satisfy(|t| matches!(t, Token::Number(_)));
// ... compose parsers for your domain-specific language
```

## Core Concepts

### Parsers

A `Parser<I, O>` consumes input of type `I` and produces output of type `O`. Parsers are:

- **Composable**: Combine small parsers into larger ones
- **Pure**: No side effects, immutable state
- **Lazy**: Only consume input as needed

### Error Handling

The library provides structured error types with contextual information:

```rust
pub enum ParseError<I> {
    UnexpectedEof,
    Expected { expected: String, found: Option<String>, input: I },
    Message { message: String, input: I },
}
```

### Input Abstraction

Any type implementing the `Input` trait can be parsed:

```rust
pub trait Input: Clone {
    type Item;
    type Iterator: Iterator<Item = Self::Item>;

    fn iter(&self) -> Self::Iterator;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
}
```

## Available Combinators

### Basic Parsers
- `item()` - Parse any single item
- `satisfy(predicate)` - Parse item matching predicate
- `token(value)` - Parse specific token
- `pure(value)` - Always succeed with value
- `fail(message)` - Always fail with message

### Text-Specific Parsers
- `char(c)` - Parse specific character
- `string(s)` - Parse string literal
- `alpha()`, `digit()`, `alphanumeric()` - Character classes
- `space()`, `spaces()`, `spaces1()` - Whitespace handling
- `unsigned()`, `integer()` - Number parsing

### Combinators
- `parser.then(other)` - Sequential composition
- `parser.or(other)` - Alternative parsing
- `parser.map(f)` - Transform output
- `optional(parser)` - Make parser optional
- `many(parser)` - Zero or more repetitions
- `many1(parser)` - One or more repetitions

## Development

This project uses Nix for reproducible development environments. Use `nix develop` to enter the development shell.

### Building

```bash
# Build the project
cargo build

# Run tests
cargo test

# Format code
just fmt
```

### Testing Philosophy

This project uses **snapshot testing** via the `insta` crate. All tests capture expected behavior through literal value snapshots, ensuring:

- Deterministic test results
- Easy regression detection
- Clear behavior documentation

```bash
# Run tests
cargo test

# Review snapshot changes
cargo insta review

# Accept valid changes
cargo insta accept
```

### Project Structure

```
src/
├── lib.rs          # Main library interface
├── parser.rs       # Core parser types and traits
├── combinators.rs  # Parser combinators
├── input.rs        # Input abstraction
├── error.rs        # Error types
└── text.rs         # Text-specific parsers

tests/              # Integration tests with snapshots
├── basic_parsing.rs
├── combinators.rs
├── text_parsing.rs
└── applicative_examples.rs
```

## Design Philosophy

This library follows functional programming principles:

- **Immutable Data**: No mutation, pure functions
- **Composition**: Build complex parsers from simple ones
- **Type Safety**: Leverage Rust's type system for correctness
- **Clear Ownership**: Explicit ownership semantics, prefer immutable references

Think "OCaml with manual garbage collection" rather than "C++ with borrow checker".

## Status

🚧 **This project is in heavy development** 🚧

The API is not stable and may change between releases. Use in production at your own risk.

## Contributing

1. All tests must be in the `tests/` directory using snapshot testing
2. Follow functional programming style guidelines
3. Update `CLAUDE.md` when adding new features
4. Never use `unwrap()` in production code paths
5. Include comprehensive error handling

## License

MIT License - see [LICENSE](LICENSE) for details.

## Author

Simon Zeng <contact@simonzeng.com> (but really Claude Code with Sonnet 4)
