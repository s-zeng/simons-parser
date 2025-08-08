//! Error types for the parser combinator library.

use std::fmt;

/// Result type used throughout the parser library.
pub type ParseResult<I, T> = Result<(T, I), ParseError<I>>;

/// Error type representing parsing failures.
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError<I> {
    /// Unexpected end of input
    UnexpectedEof,
    /// Expected something but found something else
    Expected {
        expected: String,
        found: Option<String>,
        input: I,
    },
    /// Custom error with message
    Message { message: String, input: I },
    /// Multiple errors (for choice combinators)
    Many(Vec<ParseError<I>>),
}

impl<I> ParseError<I> {
    /// Create a new expected error
    pub fn expected(
        expected: impl Into<String>,
        found: Option<impl Into<String>>,
        input: I,
    ) -> Self {
        ParseError::Expected {
            expected: expected.into(),
            found: found.map(|f| f.into()),
            input,
        }
    }

    /// Create a new message error
    pub fn message(message: impl Into<String>, input: I) -> Self {
        ParseError::Message {
            message: message.into(),
            input,
        }
    }

    /// Create a new many errors
    pub fn many(errors: Vec<ParseError<I>>) -> Self {
        ParseError::Many(errors)
    }
}

impl<I> fmt::Display for ParseError<I>
where
    I: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::UnexpectedEof => write!(f, "unexpected end of input"),
            ParseError::Expected {
                expected,
                found,
                input,
            } => {
                write!(f, "expected {}", expected)?;
                if let Some(found) = found {
                    write!(f, ", found {}", found)?;
                }
                write!(f, " at {:?}", input)
            }
            ParseError::Message { message, input } => {
                write!(f, "{} at {:?}", message, input)
            }
            ParseError::Many(errors) => {
                write!(f, "multiple errors: ")?;
                for (i, error) in errors.iter().enumerate() {
                    if i > 0 {
                        write!(f, "; ")?;
                    }
                    write!(f, "{}", error)?;
                }
                Ok(())
            }
        }
    }
}

impl<I> std::error::Error for ParseError<I> where I: fmt::Debug + Send + Sync + 'static {}
