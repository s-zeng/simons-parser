//! Core Parser trait and Applicative/Monadic combinators.

use crate::{Input, ParseError, ParseResult};
use std::marker::PhantomData;

/// A parser that consumes input of type `I` and produces a value of type `T`.
///
/// Parsers are designed to be composable using Applicative and Monadic combinators,
/// with emphasis on Applicative style for maximum parallelization opportunities.
pub trait Parser<I: Input, T> {
    /// Run the parser on the given input
    fn parse(&self, input: I) -> ParseResult<I, T>;

    /// Applicative map: transform the result of a successful parse
    fn map<U, F>(self, f: F) -> Map<Self, F, T, U>
    where
        Self: Sized,
        F: Fn(T) -> U,
    {
        Map { parser: self, f, _phantom: PhantomData }
    }

    /// Applicative sequence: parse two things in sequence, keeping both results
    fn and<U, P>(self, other: P) -> And<Self, P>
    where
        Self: Sized,
        P: Parser<I, U>,
    {
        And { left: self, right: other }
    }

    /// Parse this, then that, keeping only the result of this
    fn skip<U, P>(self, other: P) -> Skip<Self, P, T, U>
    where
        Self: Sized,
        P: Parser<I, U>,
    {
        Skip { left: self, right: other, _phantom: PhantomData }
    }

    /// Parse that, then this, keeping only the result of this
    fn preceded_by<U, P>(self, other: P) -> PrecededBy<P, Self, T, U>
    where
        Self: Sized,
        P: Parser<I, U>,
    {
        PrecededBy { first: other, second: self, _phantom: PhantomData }
    }

    /// Monadic bind: parse this, then use the result to determine the next parser
    fn bind<U, F, P>(self, f: F) -> Bind<Self, F, T, U>
    where
        Self: Sized,
        F: Fn(T) -> P,
        P: Parser<I, U>,
    {
        Bind { parser: self, f, _phantom: PhantomData }
    }

    /// Alternative: try this parser, if it fails try the other
    fn or<P>(self, other: P) -> Or<Self, P>
    where
        Self: Sized,
        P: Parser<I, T>,
    {
        Or { left: self, right: other }
    }

    /// Make this parser optional (returns Some(result) or None)
    fn optional(self) -> Optional<Self>
    where
        Self: Sized,
    {
        Optional { parser: self }
    }

    /// Parse zero or more occurrences
    fn many(self) -> Many<Self>
    where
        Self: Sized,
        T: Clone,
    {
        Many { parser: self }
    }

    /// Parse one or more occurrences
    fn many1(self) -> Many1<Self>
    where
        Self: Sized,
        T: Clone,
    {
        Many1 { parser: self }
    }
}

// Applicative combinators

/// Map combinator - transforms parser output
pub struct Map<P, F, T, U> {
    parser: P,
    f: F,
    _phantom: PhantomData<(T, U)>,
}

impl<I, T, U, P, F> Parser<I, U> for Map<P, F, T, U>
where
    I: Input,
    P: Parser<I, T>,
    F: Fn(T) -> U,
{
    fn parse(&self, input: I) -> ParseResult<I, U> {
        self.parser.parse(input).map(|(result, remaining)| ((self.f)(result), remaining))
    }
}

/// And combinator - parses two things in sequence, keeping both
pub struct And<L, R> {
    left: L,
    right: R,
}

impl<I, T, U, L, R> Parser<I, (T, U)> for And<L, R>
where
    I: Input,
    L: Parser<I, T>,
    R: Parser<I, U>,
{
    fn parse(&self, input: I) -> ParseResult<I, (T, U)> {
        let (left_result, input1) = self.left.parse(input)?;
        let (right_result, input2) = self.right.parse(input1)?;
        Ok(((left_result, right_result), input2))
    }
}

/// Skip combinator - parse left then right, keep only left result
pub struct Skip<L, R, T, U> {
    left: L,
    right: R,
    _phantom: PhantomData<(T, U)>,
}

impl<I, T, U, L, R> Parser<I, T> for Skip<L, R, T, U>
where
    I: Input,
    L: Parser<I, T>,
    R: Parser<I, U>,
{
    fn parse(&self, input: I) -> ParseResult<I, T> {
        let (left_result, input1) = self.left.parse(input)?;
        let (_, input2) = self.right.parse(input1)?;
        Ok((left_result, input2))
    }
}

/// PrecededBy combinator - parse first then second, keep only second result
pub struct PrecededBy<F, S, T, U> {
    first: F,
    second: S,
    _phantom: PhantomData<(T, U)>,
}

impl<I, T, U, F, S> Parser<I, T> for PrecededBy<F, S, T, U>
where
    I: Input,
    F: Parser<I, U>,
    S: Parser<I, T>,
{
    fn parse(&self, input: I) -> ParseResult<I, T> {
        let (_, input1) = self.first.parse(input)?;
        self.second.parse(input1)
    }
}

// Monadic combinators

/// Bind combinator - monadic sequencing
pub struct Bind<P, F, T, U> {
    parser: P,
    f: F,
    _phantom: PhantomData<(T, U)>,
}

impl<I, T, U, P, F, Q> Parser<I, U> for Bind<P, F, T, U>
where
    I: Input,
    P: Parser<I, T>,
    F: Fn(T) -> Q,
    Q: Parser<I, U>,
{
    fn parse(&self, input: I) -> ParseResult<I, U> {
        let (result, input1) = self.parser.parse(input)?;
        (self.f)(result).parse(input1)
    }
}

// Choice combinators

/// Or combinator - try left, if it fails try right
pub struct Or<L, R> {
    left: L,
    right: R,
}

impl<I, T, L, R> Parser<I, T> for Or<L, R>
where
    I: Input,
    L: Parser<I, T>,
    R: Parser<I, T>,
{
    fn parse(&self, input: I) -> ParseResult<I, T> {
        match self.left.parse(input.clone()) {
            Ok(result) => Ok(result),
            Err(left_err) => match self.right.parse(input) {
                Ok(result) => Ok(result),
                Err(right_err) => Err(ParseError::many(vec![left_err, right_err])),
            },
        }
    }
}

/// Optional combinator - makes a parser optional
pub struct Optional<P> {
    parser: P,
}

impl<I, T, P> Parser<I, Option<T>> for Optional<P>
where
    I: Input,
    P: Parser<I, T>,
{
    fn parse(&self, input: I) -> ParseResult<I, Option<T>> {
        match self.parser.parse(input.clone()) {
            Ok((result, remaining)) => Ok((Some(result), remaining)),
            Err(_) => Ok((None, input)),
        }
    }
}

// Repetition combinators

/// Many combinator - zero or more occurrences
pub struct Many<P> {
    parser: P,
}

impl<I, T, P> Parser<I, Vec<T>> for Many<P>
where
    I: Input,
    P: Parser<I, T>,
    T: Clone,
{
    fn parse(&self, mut input: I) -> ParseResult<I, Vec<T>> {
        let mut results = Vec::new();
        
        loop {
            match self.parser.parse(input.clone()) {
                Ok((result, remaining)) => {
                    results.push(result);
                    input = remaining;
                }
                Err(_) => break,
            }
        }
        
        Ok((results, input))
    }
}

/// Many1 combinator - one or more occurrences
pub struct Many1<P> {
    parser: P,
}

impl<I, T, P> Parser<I, Vec<T>> for Many1<P>
where
    I: Input,
    P: Parser<I, T>,
    T: Clone,
{
    fn parse(&self, input: I) -> ParseResult<I, Vec<T>> {
        let (first, mut remaining) = self.parser.parse(input)?;
        let mut results = vec![first];
        
        loop {
            match self.parser.parse(remaining.clone()) {
                Ok((result, new_remaining)) => {
                    results.push(result);
                    remaining = new_remaining;
                }
                Err(_) => break,
            }
        }
        
        Ok((results, remaining))
    }
}

// Pure/Return functions for Applicative

/// Pure - lifts a value into the parser context (always succeeds)
pub fn pure<I: Input, T: Clone>(value: T) -> Pure<I, T> {
    Pure { value, _phantom: PhantomData }
}

pub struct Pure<I, T> {
    value: T,
    _phantom: PhantomData<I>,
}

impl<I: Input, T: Clone> Parser<I, T> for Pure<I, T> {
    fn parse(&self, input: I) -> ParseResult<I, T> {
        Ok((self.value.clone(), input))
    }
}

/// Fail - always fails with the given error message
pub fn fail<I: Input, T>(message: impl Into<String>) -> Fail<I, T> {
    Fail { 
        message: message.into(),
        _phantom: PhantomData,
    }
}

pub struct Fail<I, T> {
    message: String,
    _phantom: PhantomData<(I, T)>,
}

impl<I: Input, T> Parser<I, T> for Fail<I, T> {
    fn parse(&self, input: I) -> ParseResult<I, T> {
        Err(ParseError::message(self.message.clone(), input))
    }
}