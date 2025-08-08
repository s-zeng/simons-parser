//! Text and string parsing utilities.

use crate::{Input, ParseError, ParseResult, Parser, combinators::*};

/// Parse a specific character
pub fn char(c: char) -> Char {
    Char { expected: c }
}

pub struct Char {
    expected: char,
}

impl<'a> Parser<&'a str, char> for Char {
    fn parse(&self, input: &'a str) -> ParseResult<&'a str, char> {
        token(self.expected).parse(input)
    }
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
pub fn alpha() -> Alpha {
    Alpha
}

pub struct Alpha;

impl<'a> Parser<&'a str, char> for Alpha {
    fn parse(&self, input: &'a str) -> ParseResult<&'a str, char> {
        satisfy(|c: &char| c.is_alphabetic()).parse(input)
    }
}

/// Parse any numeric digit
pub fn digit() -> Digit {
    Digit
}

pub struct Digit;

impl<'a> Parser<&'a str, char> for Digit {
    fn parse(&self, input: &'a str) -> ParseResult<&'a str, char> {
        satisfy(|c: &char| c.is_ascii_digit()).parse(input)
    }
}

/// Parse any alphanumeric character
pub fn alphanumeric() -> Alphanumeric {
    Alphanumeric
}

pub struct Alphanumeric;

impl<'a> Parser<&'a str, char> for Alphanumeric {
    fn parse(&self, input: &'a str) -> ParseResult<&'a str, char> {
        satisfy(|c: &char| c.is_alphanumeric()).parse(input)
    }
}

/// Parse any whitespace character
pub fn space() -> Space {
    Space
}

pub struct Space;

impl<'a> Parser<&'a str, char> for Space {
    fn parse(&self, input: &'a str) -> ParseResult<&'a str, char> {
        satisfy(|c: &char| c.is_whitespace()).parse(input)
    }
}

/// Parse zero or more whitespace characters
pub fn spaces() -> Spaces {
    Spaces
}

pub struct Spaces;

impl<'a> Parser<&'a str, String> for Spaces {
    fn parse(&self, input: &'a str) -> ParseResult<&'a str, String> {
        space()
            .many()
            .map(|chars| chars.into_iter().collect())
            .parse(input)
    }
}

/// Parse one or more whitespace characters
pub fn spaces1() -> Spaces1 {
    Spaces1
}

pub struct Spaces1;

impl<'a> Parser<&'a str, String> for Spaces1 {
    fn parse(&self, input: &'a str) -> ParseResult<&'a str, String> {
        space()
            .many1()
            .map(|chars| chars.into_iter().collect())
            .parse(input)
    }
}

/// Parse a newline character
pub fn newline() -> Newline {
    Newline
}

pub struct Newline;

impl<'a> Parser<&'a str, char> for Newline {
    fn parse(&self, input: &'a str) -> ParseResult<&'a str, char> {
        char('\n').parse(input)
    }
}

/// Parse a tab character  
pub fn tab() -> Tab {
    Tab
}

pub struct Tab;

impl<'a> Parser<&'a str, char> for Tab {
    fn parse(&self, input: &'a str) -> ParseResult<&'a str, char> {
        char('\t').parse(input)
    }
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
pub fn not_char(c: char) -> NotChar {
    NotChar { forbidden: c }
}

pub struct NotChar {
    forbidden: char,
}

impl<'a> Parser<&'a str, char> for NotChar {
    fn parse(&self, input: &'a str) -> ParseResult<&'a str, char> {
        satisfy(|ch: &char| *ch != self.forbidden).parse(input)
    }
}

/// Parse any character from a given set
pub fn one_of(chars: &str) -> OneOf {
    OneOf {
        chars: chars.to_string(),
    }
}

pub struct OneOf {
    chars: String,
}

impl<'a> Parser<&'a str, char> for OneOf {
    fn parse(&self, input: &'a str) -> ParseResult<&'a str, char> {
        satisfy(|c: &char| self.chars.contains(*c)).parse(input)
    }
}

/// Parse any character not in the given set
pub fn none_of(chars: &str) -> NoneOf {
    NoneOf {
        chars: chars.to_string(),
    }
}

pub struct NoneOf {
    chars: String,
}

impl<'a> Parser<&'a str, char> for NoneOf {
    fn parse(&self, input: &'a str) -> ParseResult<&'a str, char> {
        satisfy(|c: &char| !self.chars.contains(*c)).parse(input)
    }
}
