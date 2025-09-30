//! Basic parsing primitives and utility combinators.

use crate::{Input, ParseError, ParseResult, Parser};
use std::marker::PhantomData;

/// Consumes any single item from the input
pub fn item<I: Input>() -> Item<I> {
    Item {
        _phantom: PhantomData,
    }
}

pub struct Item<I> {
    _phantom: PhantomData<I>,
}

impl<I: Input> Parser<I, I::Item> for Item<I> {
    fn parse(&self, input: I) -> ParseResult<I, I::Item> {
        match input.uncons() {
            Some((item, remaining)) => Ok((item, remaining)),
            None => Err(ParseError::UnexpectedEof),
        }
    }
}

/// Parses an item that satisfies the given predicate
pub fn satisfy<I, F>(predicate: F) -> Satisfy<I, F>
where
    I: Input,
    F: Fn(&I::Item) -> bool,
{
    Satisfy {
        predicate,
        _phantom: PhantomData,
    }
}

pub struct Satisfy<I, F> {
    predicate: F,
    _phantom: PhantomData<I>,
}

impl<I, F> Parser<I, I::Item> for Satisfy<I, F>
where
    I: Input,
    F: Fn(&I::Item) -> bool,
{
    fn parse(&self, input: I) -> ParseResult<I, I::Item> {
        match input.uncons() {
            Some((item, remaining)) => {
                if (self.predicate)(&item) {
                    Ok((item, remaining))
                } else {
                    Err(ParseError::expected(
                        "item satisfying predicate",
                        Some("different item"),
                        input,
                    ))
                }
            }
            None => Err(ParseError::UnexpectedEof),
        }
    }
}

/// Parses a specific item
pub fn token<I: Input>(expected: I::Item) -> Token<I> {
    Token { expected }
}

pub struct Token<I: Input> {
    expected: I::Item,
}

impl<I: Input> Parser<I, I::Item> for Token<I> {
    fn parse(&self, input: I) -> ParseResult<I, I::Item> {
        match input.uncons() {
            Some((item, remaining)) => {
                if item == self.expected {
                    Ok((item, remaining))
                } else {
                    Err(ParseError::expected(
                        format!("{:?}", self.expected),
                        Some(format!("{:?}", item)),
                        input,
                    ))
                }
            }
            None => Err(ParseError::UnexpectedEof),
        }
    }
}

/// Succeeds without consuming input (empty parser)
/// This is an alias for `pure` from the parser module
pub fn empty<I: Input, T: Clone>(value: T) -> crate::parser::Pure<I, T> {
    crate::parser::pure(value)
}

/// Parses between two delimiters
/// Composed using preceded_by and skip combinators
pub fn between<I, L, R, P, T, U, V>(left: L, parser: P, right: R) -> impl Parser<I, U>
where
    I: Input,
    L: Parser<I, T>,
    P: Parser<I, U>,
    R: Parser<I, V>,
{
    parser.preceded_by(left).skip(right)
}

/// Choice between multiple parsers (tries each in order)
pub fn choice<I: Input, T, P: Parser<I, T>>(parsers: Vec<P>) -> Choice<I, T, P> {
    Choice {
        parsers,
        _phantom: PhantomData,
    }
}

pub struct Choice<I, T, P> {
    parsers: Vec<P>,
    _phantom: PhantomData<(I, T)>,
}

impl<I, T, P> Parser<I, T> for Choice<I, T, P>
where
    I: Input,
    P: Parser<I, T>,
{
    fn parse(&self, input: I) -> ParseResult<I, T> {
        let mut errors = Vec::new();

        for parser in &self.parsers {
            match parser.parse(input.clone()) {
                Ok(result) => return Ok(result),
                Err(err) => errors.push(err),
            }
        }

        Err(ParseError::many(errors))
    }
}

/// Helper function for parsing separated items (shared logic)
fn parse_sep_by_impl<I, P, S, T, U>(
    parser: &P,
    separator: &S,
    first: T,
    mut remaining: I,
) -> ParseResult<I, Vec<T>>
where
    I: Input,
    P: Parser<I, T>,
    S: Parser<I, U>,
    T: Clone,
{
    let mut results = vec![first];

    // Parse separator followed by element, repeatedly
    loop {
        let input_before_sep = remaining.clone();
        match separator.parse(remaining.clone()) {
            Ok((_, after_sep)) => match parser.parse(after_sep) {
                Ok((element, after_element)) => {
                    results.push(element);
                    remaining = after_element;
                }
                Err(_) => {
                    // Separator without following element - backtrack
                    remaining = input_before_sep;
                    break;
                }
            },
            Err(_) => break, // No more separators
        }
    }

    Ok((results, remaining))
}

/// Parses items separated by a delimiter
pub fn sep_by<I, P, S, T, U>(parser: P, separator: S) -> SepBy<P, S, T, U>
where
    I: Input,
    P: Parser<I, T>,
    S: Parser<I, U>,
    T: Clone,
{
    SepBy {
        parser,
        separator,
        _phantom: PhantomData,
    }
}

pub struct SepBy<P, S, T, U> {
    parser: P,
    separator: S,
    _phantom: PhantomData<(T, U)>,
}

impl<I, P, S, T, U> Parser<I, Vec<T>> for SepBy<P, S, T, U>
where
    I: Input,
    P: Parser<I, T>,
    S: Parser<I, U>,
    T: Clone,
{
    fn parse(&self, input: I) -> ParseResult<I, Vec<T>> {
        // Try to parse the first element
        match self.parser.parse(input.clone()) {
            Ok((first, remaining)) => {
                parse_sep_by_impl(&self.parser, &self.separator, first, remaining)
            }
            Err(_) => Ok((Vec::new(), input)), // Empty list is valid
        }
    }
}

/// Parse one or more items separated by a delimiter
pub fn sep_by1<I, P, S, T, U>(parser: P, separator: S) -> SepBy1<P, S, T, U>
where
    I: Input,
    P: Parser<I, T>,
    S: Parser<I, U>,
    T: Clone,
{
    SepBy1 {
        parser,
        separator,
        _phantom: PhantomData,
    }
}

pub struct SepBy1<P, S, T, U> {
    parser: P,
    separator: S,
    _phantom: PhantomData<(T, U)>,
}

impl<I, P, S, T, U> Parser<I, Vec<T>> for SepBy1<P, S, T, U>
where
    I: Input,
    P: Parser<I, T>,
    S: Parser<I, U>,
    T: Clone,
{
    fn parse(&self, input: I) -> ParseResult<I, Vec<T>> {
        let (first, remaining) = self.parser.parse(input)?;
        parse_sep_by_impl(&self.parser, &self.separator, first, remaining)
    }
}

/// Parses end of input
pub fn eof<I: Input>() -> Eof<I> {
    Eof {
        _phantom: PhantomData,
    }
}

pub struct Eof<I> {
    _phantom: PhantomData<I>,
}

impl<I: Input> Parser<I, ()> for Eof<I> {
    fn parse(&self, input: I) -> ParseResult<I, ()> {
        if input.is_empty() {
            Ok(((), input))
        } else {
            Err(ParseError::expected(
                "end of input",
                Some("more input"),
                input,
            ))
        }
    }
}
