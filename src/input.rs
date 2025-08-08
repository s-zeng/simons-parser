//! Input stream trait for general input types.

/// Trait for types that can be used as input to parsers.
/// 
/// This allows parsing over any type that can provide iterator-like access,
/// not just strings or byte slices. Examples include HTML DOM trees, JSON values,
/// or custom data structures.
pub trait Input: Clone + PartialEq {
    /// The type of individual items in the input stream
    type Item: Clone + PartialEq + std::fmt::Debug;

    /// Returns the next item from the input stream, along with the remaining input.
    /// Returns None if the input is empty.
    fn uncons(&self) -> Option<(Self::Item, Self)>;

    /// Returns true if the input stream is empty
    fn is_empty(&self) -> bool {
        self.uncons().is_none()
    }

    /// Returns the length of remaining input, if known
    fn len(&self) -> Option<usize> {
        None
    }
}

/// Implementation for string slices - the most common case
impl<'a> Input for &'a str {
    type Item = char;

    fn uncons(&self) -> Option<(Self::Item, Self)> {
        let mut chars = self.chars();
        chars.next().map(|c| (c, &self[c.len_utf8()..]))
    }

    fn len(&self) -> Option<usize> {
        Some(str::len(self))
    }
}

/// Implementation for byte slices
impl<'a> Input for &'a [u8] {
    type Item = u8;

    fn uncons(&self) -> Option<(Self::Item, Self)> {
        if self.is_empty() {
            None
        } else {
            Some((self[0], &self[1..]))
        }
    }

    fn len(&self) -> Option<usize> {
        Some(<[u8]>::len(self))
    }
}

/// Implementation for Vecs
impl<T: Clone + PartialEq + std::fmt::Debug> Input for Vec<T> {
    type Item = T;

    fn uncons(&self) -> Option<(Self::Item, Self)> {
        if self.is_empty() {
            None
        } else {
            Some((self[0].clone(), self[1..].to_vec()))
        }
    }

    fn len(&self) -> Option<usize> {
        Some(Vec::len(self))
    }
}

