//! A parser combinator library inspired by Parsec and Haskell's parser combinators.
//!
//! This library provides a functional approach to parsing with support for general
//! input stream types, not just text. You can parse HTML DOM, JSON structures, or
//! any type that implements the required iterator traits.

pub mod error;
pub mod input;
pub mod parser;
pub mod combinators;
pub mod text;

pub use error::{ParseError, ParseResult};
pub use input::Input;
pub use parser::{Parser, pure, fail, Pure, Fail};
pub use combinators::*;
pub use text::*;