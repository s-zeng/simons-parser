//! Basic parsing primitives and utility combinators.

use crate::{Input, ParseError, ParseResult, Parser};
use std::marker::PhantomData;

/// Consumes any single item from the input
pub fn item<I: Input>() -> Item<I> {
    Item { _phantom: PhantomData }
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
    Satisfy { predicate, _phantom: PhantomData }
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
pub fn empty<I: Input, T: Clone>(value: T) -> Empty<I, T> {
    Empty { value, _phantom: PhantomData }
}

pub struct Empty<I, T> {
    value: T,
    _phantom: PhantomData<I>,
}

impl<I: Input, T: Clone> Parser<I, T> for Empty<I, T> {
    fn parse(&self, input: I) -> ParseResult<I, T> {
        Ok((self.value.clone(), input))
    }
}

/// Parses between two delimiters
pub fn between<I, L, R, P, T, U, V>(left: L, parser: P, right: R) -> Between<L, P, R, T, U, V>
where
    I: Input,
    L: Parser<I, T>,
    P: Parser<I, U>,
    R: Parser<I, V>,
{
    Between { left, parser, right, _phantom: PhantomData }
}

pub struct Between<L, P, R, T, U, V> {
    left: L,
    parser: P,
    right: R,
    _phantom: PhantomData<(T, U, V)>,
}

impl<I, L, P, R, T, U, V> Parser<I, U> for Between<L, P, R, T, U, V>
where
    I: Input,
    L: Parser<I, T>,
    P: Parser<I, U>,
    R: Parser<I, V>,
{
    fn parse(&self, input: I) -> ParseResult<I, U> {
        let (_, input1) = self.left.parse(input)?;
        let (result, input2) = self.parser.parse(input1)?;
        let (_, input3) = self.right.parse(input2)?;
        Ok((result, input3))
    }
}

/// Choice between multiple parsers (tries each in order)
pub fn choice<I: Input, T, P: Parser<I, T>>(parsers: Vec<P>) -> Choice<I, T, P> {
    Choice { parsers, _phantom: PhantomData }
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

/// Parses items separated by a delimiter
pub fn sep_by<I, P, S, T, U>(parser: P, separator: S) -> SepBy<P, S, T, U>
where
    I: Input,
    P: Parser<I, T>,
    S: Parser<I, U>,
    T: Clone,
{
    SepBy { parser, separator, _phantom: PhantomData }
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
        let (first, mut remaining) = match self.parser.parse(input.clone()) {
            Ok(result) => result,
            Err(_) => return Ok((Vec::new(), input)), // Empty list is valid
        };
        
        let mut results = vec![first];
        
        // Parse separator followed by element, repeatedly
        loop {
            let input_before_sep = remaining.clone();
            match self.separator.parse(remaining.clone()) {
                Ok((_, after_sep)) => {
                    match self.parser.parse(after_sep) {
                        Ok((element, after_element)) => {
                            results.push(element);
                            remaining = after_element;
                        }
                        Err(_) => {
                            // Separator without following element - backtrack
                            remaining = input_before_sep;
                            break;
                        }
                    }
                }
                Err(_) => break, // No more separators
            }
        }
        
        Ok((results, remaining))
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
    SepBy1 { parser, separator, _phantom: PhantomData }
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
        let (first, mut remaining) = self.parser.parse(input)?;
        let mut results = vec![first];
        
        // Parse separator followed by element, repeatedly
        loop {
            let input_before_sep = remaining.clone();
            match self.separator.parse(remaining.clone()) {
                Ok((_, after_sep)) => {
                    match self.parser.parse(after_sep) {
                        Ok((element, after_element)) => {
                            results.push(element);
                            remaining = after_element;
                        }
                        Err(_) => {
                            // Separator without following element - backtrack
                            remaining = input_before_sep;
                            break;
                        }
                    }
                }
                Err(_) => break, // No more separators
            }
        }
        
        Ok((results, remaining))
    }
}

/// Parses end of input
pub fn eof<I: Input>() -> Eof<I> {
    Eof { _phantom: PhantomData }
}

pub struct Eof<I> {
    _phantom: PhantomData<I>,
}

impl<I: Input> Parser<I, ()> for Eof<I> {
    fn parse(&self, input: I) -> ParseResult<I, ()> {
        if input.is_empty() {
            Ok(((), input))
        } else {
            Err(ParseError::expected("end of input", Some("more input"), input))
        }
    }
}