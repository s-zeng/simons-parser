//! Text and string parsing utilities.

use crate::{Input, ParseError, ParseResult, Parser, combinators::*};

/// Parse a specific character
/// Composed using the token combinator
pub fn char<'a>(c: char) -> impl Parser<&'a str, char> {
    token(c)
}

/// Parse a specific string
pub fn string(s: &str) -> String_ {
    String_ {
        expected: s.to_string(),
    }
}

pub struct String_ {
    expected: String,
}

impl<'a> Parser<&'a str, String> for String_ {
    fn parse(&self, mut input: &'a str) -> ParseResult<&'a str, String> {
        let original_input = input;
        let mut matched = String::new();

        for expected_char in self.expected.chars() {
            match input.uncons() {
                Some((c, remaining)) if c == expected_char => {
                    matched.push(c);
                    input = remaining;
                }
                Some((c, _)) => {
                    return Err(ParseError::expected(
                        format!("string '{}'", self.expected),
                        Some(format!("character '{}'", c)),
                        original_input,
                    ));
                }
                None => {
                    return Err(ParseError::expected(
                        format!("string '{}'", self.expected),
                        Some("end of input"),
                        original_input,
                    ));
                }
            }
        }

        Ok((matched, input))
    }
}

/// Parse any alphabetic character
/// Composed using the satisfy combinator
pub fn alpha<'a>() -> impl Parser<&'a str, char> {
    satisfy(|c: &char| c.is_alphabetic())
}

/// Parse any numeric digit
/// Composed using the satisfy combinator
pub fn digit<'a>() -> impl Parser<&'a str, char> {
    satisfy(|c: &char| c.is_ascii_digit())
}

/// Parse any alphanumeric character
/// Composed using the satisfy combinator
pub fn alphanumeric<'a>() -> impl Parser<&'a str, char> {
    satisfy(|c: &char| c.is_alphanumeric())
}

/// Parse any whitespace character
/// Composed using the satisfy combinator
pub fn space<'a>() -> impl Parser<&'a str, char> {
    satisfy(|c: &char| c.is_whitespace())
}

/// Parse zero or more whitespace characters
/// Composed using space(), many(), and map()
pub fn spaces<'a>() -> impl Parser<&'a str, String> {
    space().many().map(|chars| chars.into_iter().collect())
}

/// Parse one or more whitespace characters
/// Composed using space(), many1(), and map()
pub fn spaces1<'a>() -> impl Parser<&'a str, String> {
    space().many1().map(|chars| chars.into_iter().collect())
}

/// Parse a newline character
/// Composed using the char combinator
pub fn newline<'a>() -> impl Parser<&'a str, char> {
    char('\n')
}

/// Parse a tab character
/// Composed using the char combinator
pub fn tab<'a>() -> impl Parser<&'a str, char> {
    char('\t')
}

/// Parse an unsigned integer
pub fn unsigned() -> Unsigned {
    Unsigned
}

pub struct Unsigned;

impl<'a> Parser<&'a str, u32> for Unsigned {
    fn parse(&self, input: &'a str) -> ParseResult<&'a str, u32> {
        let (digits, remaining) = digit().many1().parse(input)?;
        let digits_str: String = digits.into_iter().collect();
        match digits_str.parse::<u32>() {
            Ok(n) => Ok((n, remaining)),
            Err(_) => Err(ParseError::message("invalid number", input)),
        }
    }
}

/// Parse a signed integer
pub fn integer() -> Integer {
    Integer
}

pub struct Integer;

impl<'a> Parser<&'a str, i32> for Integer {
    fn parse(&self, input: &'a str) -> ParseResult<&'a str, i32> {
        char('-')
            .optional()
            .and(unsigned())
            .map(|(sign, num)| match sign {
                Some(_) => -(num as i32),
                None => num as i32,
            })
            .parse(input)
    }
}

/// Parse any character except the given one
/// Composed using the satisfy combinator
pub fn not_char<'a>(c: char) -> impl Parser<&'a str, char> {
    satisfy(move |ch: &char| *ch != c)
}

/// Parse any character from a given set
/// Composed using the satisfy combinator
pub fn one_of<'a>(chars: &str) -> impl Parser<&'a str, char> {
    let chars = chars.to_string();
    satisfy(move |c: &char| chars.contains(*c))
}

/// Parse any character not in the given set
/// Composed using the satisfy combinator
pub fn none_of<'a>(chars: &str) -> impl Parser<&'a str, char> {
    let chars = chars.to_string();
    satisfy(move |c: &char| !chars.contains(*c))
}
