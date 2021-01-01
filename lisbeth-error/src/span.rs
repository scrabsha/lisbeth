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

/// Represents a position in the input data.
///
/// Positions are 0-indexed, meaning that the first character of each line has
/// 0 as column number. The same goes for the line number.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Position {
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

    /// Returns the position's line.
    #[inline]
    pub const fn line(self) -> u32 {
        self.line
    }

    /// Returns the position's column.
    #[inline]
    pub const fn col(self) -> u32 {
        self.col
    }

    /// Returns the position's offset from the beginning of the file.
    #[inline]
    pub const fn offset(self) -> u32 {
        self.offset
    }
}

// Note: when the following documentation is modified, remember to update the
// doc for Position::Ord accordingly.
/// # Warning
///
/// Positions can be compared toghether only if they come from the same input
/// unit. If they do not, then inconsistencies may occur.
///
/// # Panics
///
/// In debug mode, this function may panic if the two positions are not from the
/// same input unit. In release mode, this function does not panic.
impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Position) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Note: when the following documentation is modified, remember to update the
// doc for Position::PartialOrd accordingly.
/// # Warning
///
/// Positions can be compared toghether only if they come from the same input
/// unit. If they do not, then inconsistencies may occur.
///
/// # Panics
///
/// In debug mode, this function may panic if the two positions are not from the
/// same input unit. In release mode, this function does not panic.
impl Ord for Position {
    #[cfg(debug)]
    fn cmp(&self, other: &Position) -> Ordering {
        let offset_provided = self.offset.cmp(&other.offset);

        let lc_provided = match self.line.cmp(&other.line) {
            Ordering::Equal => self.col.cmp(&other.col),
            any => any,
        };

        assert!(
            offset_provided != lc_provided,
            "Attempt to perform an inconsistent span comparaison",
        );

        offset_provided
    }

    #[cfg(not(debug))]
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

impl Span {
    /// Returns the span's starting position.
    #[inline]
    pub const fn start(self) -> Position {
        self.start
    }

    /// Returns the span's ending position.
    ///
    /// The position ends on the next non-spanned part:
    ///
    /// ```rust
    /// use lisbeth_error::span::SpannedStr;
    ///
    /// let s = SpannedStr::input_file("hello");
    /// assert_eq!(s.span().end().col(), 5);
    /// ```
    #[inline]
    pub const fn end(self) -> Position {
        self.end
    }

    #[inline]
    const fn split_with(self, mid: Position) -> (Span, Span) {
        let Span { start, end } = self;

        let left = Span { start, end: mid };
        let right = Span { start: mid, end };

        (left, right)
    }

    pub(crate) fn of_file(input: &str) -> Span {
        let start = Position::BEGINNING;
        let end = start.advance_with(input);

        Span { start, end }
    }

    /// Returns the span of the character following the current span, on the
    /// same line.
    ///
    /// This function can be used when an unexpected EOF happens.
    ///
    /// ```rust
    /// use lisbeth_error::span::{Span, SpannedStr};
    ///
    /// let input = SpannedStr::input_file("foo");
    /// let after_input = input.span().next_char();
    ///
    /// assert_eq!(after_input.start().line(), 0);
    /// assert_eq!(after_input.start().col(), 3);
    ///
    /// assert_eq!(after_input.end().line(), 0);
    /// assert_eq!(after_input.end().col(), 4);
    /// ```
    pub fn next_char(self) -> Span {
        let start = self.end;
        let end = start.advance_with(" ");

        Span { start, end }
    }
}

/// Represents a portion of input file.
///
/// This is represented the same way as [`Span`], but with an additionnal
/// content field.
///
/// It is initially created with the [`input_file`] function, and can then be
/// splitted at desired index. Its content and span can be accessed with the
/// [`content`] and [`span`] methods.
///
/// # Example
///
/// The following code shows how to extract a sequence of numbers separated by
/// non-digit characters.
///
/// ```rust
/// use lisbeth_error::span::{Span, SpannedStr};
///
/// #[derive(Debug)]
/// struct Number(u32, Span);
///
/// // Parses a number from input, if any failure occurs, returns None
/// fn extract_number<'a>(input: SpannedStr<'a>) -> (Number, SpannedStr<'a>) {
///     let (matched, tail) = input.take_while(char::is_numeric);
///
///     let value = matched.content().parse().unwrap();
///     let number = Number(value, matched.span());
///     (number, tail)
/// }
///
/// let input = SpannedStr::input_file("42 or nothing");
/// let (number, tail) = extract_number(input);
///
/// assert_eq!(number.0, 42);
/// assert_eq!(tail.content(), " or nothing");
/// ```
///
/// [`input_file`]: SpannedStr::input_file
/// [`content`]: SpannedStr::content
/// [`span`]: SpannedStr::span
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
        let span = Span::of_file(content);

        SpannedStr { span, content }
    }

    // Note: span must represent the same source as content, otherwise
    // inconsistent results may occur.
    //
    // In debug mode, it is ensured that:
    //   - span.start == Position::BEGINNING,
    //   - span.end.offset == content.len().
    pub(crate) fn assemble(content: &'a str, span: Span) -> SpannedStr<'a> {
        debug_assert_eq!(
            span.start,
            Position::BEGINNING,
            "Attempt to create a SpannedStr that does not start at the beginning of the file",
        );
        debug_assert_eq!(
            span.end.offset as usize,
            content.len(),
            "Attempt to create a SpannedStr with an incorrect length",
        );

        SpannedStr { content, span }
    }

    /// Returns the contained [`Span`].
    ///
    /// The span contains the position at which the content is located in the
    /// input data.
    ///
    /// # Example
    ///
    /// ```rust
    /// use lisbeth_error::span::SpannedStr;
    ///
    /// let a = SpannedStr::input_file("foo bar");
    /// let b = SpannedStr::input_file("baz qux");
    ///
    /// // a and b have the same length and the same starting point, so they
    /// // have the same span.
    /// assert_eq!(a.span(), b.span());
    /// ```
    pub const fn span(self) -> Span {
        self.span
    }

    /// Returns the span content.
    ///
    /// # Example
    ///
    /// ```rust
    /// use lisbeth_error::span:: SpannedStr;
    ///
    /// let a = SpannedStr::input_file("hello");
    /// assert_eq!(a.content(), "hello");
    /// ```
    pub const fn content(self) -> &'a str {
        self.content
    }

    /// Splits the spanned string at a given byte index.
    ///
    /// This method works the same way as [str::split_at], but updates the span
    /// so that it is still correct.
    ///
    /// # Panics
    ///
    /// This method panics when one of the condition listed in [`str::split_at`]
    /// is met.
    ///
    /// # Example
    ///
    /// ```rust
    /// use lisbeth_error::span::SpannedStr;
    ///
    /// let input = SpannedStr::input_file("helloworld");
    /// let (left, right) = input.split_at(5);
    ///
    /// assert_eq!(left.content(), "hello");
    /// assert_eq!(right.content(), "world");
    /// ```
    pub fn split_at(self, idx: usize) -> (SpannedStr<'a>, SpannedStr<'a>) {
        let (left_content, right_content) = self.content.split_at(idx);

        let mid = self.span.start.advance_with(left_content);
        let (left_span, right_span) = self.span.split_with(mid);

        let left_sstr = SpannedStr {
            span: left_span,
            content: left_content,
        };

        let right_sstr = SpannedStr {
            span: right_span,
            content: right_content,
        };

        (left_sstr, right_sstr)
    }

    /// Returns the longest prefix of input that match a given a condition.
    ///
    /// # Example
    ///
    /// ```rust
    /// use lisbeth_error::span::SpannedStr;
    ///
    /// let i = SpannedStr::input_file("42 101");
    /// let (number, tail) = i.take_while(char::is_numeric);
    ///
    /// assert_eq!(number.content(), "42");
    /// assert_eq!(tail.content(), " 101");
    /// ```
    pub fn take_while<F>(self, mut f: F) -> (SpannedStr<'a>, SpannedStr<'a>)
    where
        F: FnMut(char) -> bool,
    {
        let idx = self
            .content
            .char_indices()
            .find(|(_, chr)| !f(*chr))
            .map(|(idx, _)| idx)
            .unwrap_or_else(|| self.content.len());

        self.split_at(idx)
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

    mod span {
        use super::*;

        #[test]
        fn of_file() {
            let i = "hello, world";
            let left = Span::of_file("hello, world");

            let start = Position::BEGINNING;
            let end = start.advance_with(i);
            let right = Span { start, end };

            assert_eq!(left, right);
        }

        #[test]
        fn next_char() {
            let s = Span {
                start: Position {
                    line: 1,
                    col: 41,
                    offset: 50,
                },
                end: Position {
                    line: 1,
                    col: 50,
                    offset: 59,
                },
            };

            let left = s.next_char();

            let right = Span {
                start: Position {
                    line: 1,
                    col: 50,
                    offset: 59,
                },
                end: Position {
                    line: 1,
                    col: 51,
                    offset: 60,
                },
            };

            assert_eq!(left, right);
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

        #[test]
        fn span_and_content() {
            let span = Span {
                start: Position {
                    line: 10,
                    col: 0,
                    offset: 100,
                },
                end: Position {
                    line: 15,
                    col: 10,
                    offset: 150,
                },
            };

            let content = "hello, world";

            let sstr = SpannedStr { content, span };

            assert_eq!(sstr.span(), span);
            assert_eq!(sstr.content(), content);
        }

        #[test]
        fn split_at_working() {
            let input = SpannedStr::input_file("foobar");
            let (left, right) = input.split_at(3);

            assert_eq!(left.content, "foo");
            assert_eq!(right.content, "bar");

            let left_span = Span {
                start: Position {
                    line: 0,
                    col: 0,
                    offset: 0,
                },
                end: Position {
                    line: 0,
                    col: 3,
                    offset: 3,
                },
            };

            let right_span = Span {
                start: Position {
                    line: 0,
                    col: 3,
                    offset: 3,
                },
                end: Position {
                    line: 0,
                    col: 6,
                    offset: 6,
                },
            };

            assert_eq!(left.span, left_span);
            assert_eq!(right.span, right_span);
        }

        #[test]
        #[should_panic(expected = "byte index 15 is out of bounds of `hello, world`")]
        fn split_at_out_of_bounds() {
            let f = SpannedStr::input_file("hello, world");
            f.split_at(15);
        }

        #[test]
        #[should_panic(
            expected = "byte index 2 is not a char boundary; it is inside \'é\' (bytes 1..3) of `Vélo`"
        )]
        fn split_at_non_boundary() {
            let f = SpannedStr::input_file("Vélo");
            f.split_at(2);
        }

        #[test]
        fn take_while() {
            let (left, right) = SpannedStr::input_file("foo bar").take_while(|c| c != ' ');

            assert_eq!(left.content, "foo");
            assert_eq!(right.content, " bar");
        }

        #[test]
        fn take_while_empty_string() {
            let input = SpannedStr::input_file("");
            let (left, right) = input.take_while(|_| true);

            assert_eq!(left, input);
            assert_eq!(right, input);
        }

        #[test]
        fn take_while_non_ascii() {
            let (left, right) = SpannedStr::input_file("éêè").take_while(|c| c != 'è');

            assert_eq!(left.content, "éê");
            assert_eq!(right.content, "è");
        }
    }
}
