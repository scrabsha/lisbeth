//! Some span data structures
//!
//! # Description
//!
//! This module contains the [`Span`] and [`SpannedStr`] data structures. The
//! difference between them is that [`SpannedStr`] contains the inner text while
//! [`Span`] contains only its position. Consequently, [`SpannedStr`] is used
//! during the lexing and parsing steps, but the AST generated *should* contain
//! only [`Span`].
//!
//! # A consistency note
//!
//! Inconsistent results may occur when [`Span`] and [`SpannedStr`] coming from
//! different places are used toghether. This is fine for most use-cases, in
//! which a single process in invoked for a single input unit.

use std::cmp::{Ord, Ordering};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Position {
    line: u32,
    col: u32,
    offset: u32,
}

impl Position {
    const BEGINNING: Position = Position {
        line: 0,
        col: 0,
        offset: 0,
    };

    fn advance_with(self, s: &str) -> Position {
        let Position {
            mut line,
            mut col,
            mut offset,
        } = self;

        s.chars().for_each(|c| {
            if c == '\n' {
                line += 1;
                col = 0
            } else {
                col += 1;
            }
        });

        offset += s.len() as u32;

        Position { line, col, offset }
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Position) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Position {
    // This compares the position according to the offset_from_beginning field.
    fn cmp(&self, other: &Position) -> Ordering {
        self.offset.cmp(&other.offset)
    }
}

/// Represents the position of a piece of code in the input file.
///
/// A `Span` is represented as the start and end position. Every character that
/// is between these two position is considered as *inside* the span.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Span {
    start: Position,
    end: Position,
}

/// Represents a portion of input file.
///
/// This is represented the same way as [`Span`], but with an additionnal
/// content field.
///
/// It is initially created with the [`input_file`][SpannedStr::input_file]
/// function, and *will* then be splitted at desired index.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SpannedStr<'a> {
    span: Span,
    content: &'a str,
}

impl<'a> SpannedStr<'a> {
    /// Creates a new [`SpannedStr`] from an input file.
    ///
    /// This returned spanned string can then be splitted at various places
    /// during the parsing step.
    ///
    /// # Example
    ///
    /// ```rust
    /// use lisbeth_error::span::SpannedStr;
    ///
    /// let file_content = "fn main() { println!(\"Hello, world!\"); }";
    ///
    /// let whole_file = SpannedStr::input_file(file_content);
    /// ```
    pub fn input_file(content: &'a str) -> SpannedStr<'a> {
        let start = Position::BEGINNING;
        let end = start.advance_with(content);

        SpannedStr {
            span: Span { start, end },
            content,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod position {
        use super::*;

        #[test]
        fn advance_with_no_line_return() {
            let p = Position::BEGINNING.advance_with("hello, world");

            assert_eq!(p.line, 0);
            assert_eq!(p.col, 12);
            assert_eq!(p.offset, 12);
        }

        #[test]
        fn advance_with_line_return() {
            let p = Position::BEGINNING.advance_with("\n\n\n");

            assert_eq!(p.line, 3);
            assert_eq!(p.col, 0);
            assert_eq!(p.offset, 3);
        }

        #[test]
        fn advance_with_mixed() {
            let p = Position::BEGINNING.advance_with("Hello,\nworld");

            assert_eq!(p.line, 1);
            assert_eq!(p.col, 5);
            assert_eq!(p.offset, 12);
        }

        #[test]
        fn advance_with_empty() {
            let p = Position::BEGINNING.advance_with("");
            assert_eq!(p, Position::BEGINNING);
        }

        #[test]
        fn advance_with_two_times() {
            let p = Position::BEGINNING.advance_with("foo bar");
            let p = p.advance_with(" baz");

            assert_eq!(p.line, 0);
            assert_eq!(p.col, 11);
            assert_eq!(p.offset, 11);
        }

        #[test]
        fn ord_simple() {
            let p = Position::BEGINNING.advance_with("hello, world!");
            let q = p.advance_with(" How are you?");

            assert!(p < q);
        }

        #[test]
        fn ord_only_cares_about_offset() {
            // This is part of the inconsistency paragraph in the module documentation
            let p = Position {
                line: 10,
                col: 20,
                offset: 1000,
            };

            let q = Position {
                line: 100,
                col: 25,
                offset: 10,
            };

            assert!(p > q);
        }
    }

    mod spanned_str {
        use super::*;

        #[test]
        fn input_file_simple() {
            let sstr = SpannedStr::input_file("hello\nworld");

            assert_eq!(sstr.span.start, Position::BEGINNING);
            assert_eq!(sstr.span.end.line, 1);
            assert_eq!(sstr.span.end.col, 5);
        }
    }
}
