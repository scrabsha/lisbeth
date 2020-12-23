//! Allows to report errors to the user.
//!
//! This module contains the [`ErrorReporter`] structure, which holds metadata
//! about the input. This structure provides the metadata that are needed to
//! properly report the error to the user.
//!
//! # Example
//!
//! The following code snippet shows how objects defined in the `lisbeth_error`
//! crate interact with each other:
//!
//! ```rust
//! use lisbeth_error::{
//!     error::AnnotatedError,
//!     reporter::ErrorReporter,
//! };
//!
//!
//! let reporter = ErrorReporter::input_file(
//!     "docs.txt".to_string(),
//!     "The cat are on the table.".to_string(),
//! );
//! let file = reporter.spanned_str();
//!
//! let cat = file.split_at(4).1.split_at(3).0;
//! let are = file.split_at(8).1.split_at(3).0;
//!
//! let report = AnnotatedError::new(are.span(), "Conjugation error")
//!     .with_annotation(cat.span(), "`cat` is singular,")
//!     .with_annotation(are.span(), "but `are` is used only for plural subject");
//!
//! println!("{}", reporter.format_error(&report));
//! ```
//!
//! This will print to STDOUT:
//!
//! ```none
//! Error: Conjugation error
//! --> docs.txt:1:9
//!     |
//!   1 |                                           The cat are on the table.
//!     |                                               ^^^ ^^^
//!     | `cat` is singular,----------------------------'   |
//!     | but `are` is used only for plural subject---------'
//!     |
//! ```

use std::{
    fmt::{self, Display},
    fs,
    io::Error as IOError,
};

use crate::{
    error::AnnotatedError,
    span::{Position, Span, SpannedStr},
};

/// Holds metadata about the input, allows to report errors to the user.
///
/// This structure should be created before the parsing process and will provide
/// metadata required for the [`AnnotatedError`].
///
/// If the reporter contains a file path, then this path must be valid UTF-8.
/// This is needed because the file path is printed on the console when an error
/// is reported.
///
/// [`AnnotatedError`]: [super::error::AnnotatedError]
pub struct ErrorReporter {
    path: Option<String>,
    content: String,
    span: Span,
}

impl ErrorReporter {
    /// Given a file path and its content, creates a new [`ErrorReporter`].
    ///
    /// `path` is not checked to be a valid path.
    pub fn input_file(path: String, content: String) -> ErrorReporter {
        let path = Some(path);
        let span = Span::of_file(content.as_str());
        ErrorReporter {
            content,
            path,
            span,
        }
    }

    /// Creates an [`ErrorReporter`] with no file path, just its content.
    ///
    /// This can be usefull in situations in which non-file inputs such as STDIN
    /// are processed.
    pub fn non_file_input(content: String) -> ErrorReporter {
        let path = None;
        let span = Span::of_file(content.as_str());
        ErrorReporter {
            content,
            path,
            span,
        }
    }

    /// Reads the content of `path`, and creates an [`ErrorReporter`] with it.
    pub fn from_path(path: String) -> Result<ErrorReporter, IOError> {
        fs::read_to_string(path.as_str())
            .map(|content| (Span::of_file(content.as_str()), content, Some(path)))
            .map(|(span, content, path)| ErrorReporter {
                content,
                path,
                span,
            })
    }

    /// Returns the file path, if it exists.
    pub fn path(&self) -> Option<&str> {
        self.path.as_deref()
    }

    /// Returns the [`SpannedStr`] associated to the whole input.
    ///
    /// # Example
    ///
    /// ```rust
    /// use lisbeth_error::reporter::ErrorReporter;
    ///
    /// let file = ErrorReporter::non_file_input("Hello, world".to_string());
    /// assert_eq!(file.spanned_str().content(), "Hello, world");
    /// ```
    pub fn spanned_str(&self) -> SpannedStr {
        // self.span has been built from self.content, so this call is fine.
        SpannedStr::assemble(self.content.as_str(), self.span)
    }

    fn code_snippet_for(&self, start_pos: Position, end_pos: Position) -> &str {
        let (start_offset, end_offset) = (start_pos.offset() as usize, end_pos.offset() as usize);

        let before_start = self.content.split_at(start_offset).0;
        let after_end = self.content.split_at(end_offset).1;

        let end_idx = end_offset as usize
            + after_end
                .char_indices()
                .find(|(_, c)| *c == '\n')
                .map(|(idx, _)| idx)
                .unwrap_or_else(|| after_end.len());

        let start_idx = before_start
            .char_indices()
            .rev()
            .take_while(|(_, c)| *c != '\n')
            .last()
            .map(|(idx, _)| idx)
            .unwrap_or_else(|| before_start.len());

        self.content.split_at(end_idx).0.split_at(start_idx).1
    }

    /// Constructs a [`FormattedError`] from an [`AnnotatedError`].
    ///
    /// The returned value can finally be printed to the user.
    pub fn format_error<'a>(&'a self, err: &'a AnnotatedError) -> FormattedError<'a> {
        let (start_pos, end_pos) = err.bounds();
        let stream_name = self.path();
        let text = self.code_snippet_for(start_pos, end_pos);

        let pos = err.span.start();
        let general_msg = err.msg.as_str();

        let errors = err.error_matrix();

        let first_line_number = start_pos.line() as usize;

        FormattedError {
            pos,
            first_line_number,
            general_msg,
            stream_name,
            text,
            errors,
        }
    }
}

/// An error object that can finally be displayed.
///
/// This structure is created by [`ErrorReporter::format_error`], and
/// implements the [`Display`] trait.
#[derive(Clone, Debug, PartialEq)]
pub struct FormattedError<'a> {
    pos: Position,
    general_msg: &'a str,
    stream_name: Option<&'a str>,
    first_line_number: usize,
    // Invariant: text.lines().count() == errors.len()
    text: &'a str,
    errors: Vec<Vec<Annotation<'a>>>,
}

impl<'a> FormattedError<'a> {
    fn write_general_message(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Error: {}", self.general_msg)
    }

    fn write_position(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (line, col) = (self.pos.line() + 1, self.pos.col() + 1);
        match self.stream_name {
            Some(name) => writeln!(f, " --> {}:{}:{}", name, line, col),
            None => writeln!(f, " --> {}:{}", line, col),
        }
    }

    fn write_header(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.write_general_message(f)?;
        self.write_position(f)
    }

    fn spacing(&self) -> usize {
        self.errors
            .iter()
            .flatten()
            .map(|ann| ann.text.len())
            .max()
            .unwrap_or(0)
    }

    fn write_line(
        content: &str,
        spacing: usize,
        number: usize,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        writeln!(f, " {:>3} | {} {}", number, " ".repeat(spacing), content)
    }

    fn write_underlines(
        errs: &[Annotation<'_>],
        spacing: usize,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        write!(f, "     | {} ", " ".repeat(spacing))?;

        let mut current_col_number = 0;
        for annotation in errs {
            let delta = annotation.col_number - current_col_number;
            let length = usize::max(1, annotation.length);
            let chr = if length == 1 { "|" } else { "^" };

            write!(f, "{}{}", " ".repeat(delta), chr.repeat(length))?;

            current_col_number += delta + length;
        }

        writeln!(f)
    }

    fn write_error_line(
        annotation: &Annotation,
        spacing: usize,
        other_annotations: &[Annotation],
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        let pipe_len = spacing - annotation.text.len() + annotation.col_number + 1;

        write!(f, "     | {}{}'", annotation.text, "-".repeat(pipe_len))?;

        let mut current_col_number = annotation.col_number;

        for annotation in other_annotations {
            let delta = annotation.col_number - current_col_number - 1;
            write!(f, "{}|", " ".repeat(delta))?;

            current_col_number = annotation.col_number;
        }

        writeln!(f)
    }

    fn write_errors(
        annotations: &[Annotation<'_>],
        spacing: usize,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        Self::write_underlines(annotations, spacing, f)?;

        for idx in 0..annotations.len() {
            let annotation = &annotations[idx];
            let annotations = &annotations[idx + 1..];

            Self::write_error_line(annotation, spacing, annotations, f)?;
        }

        Ok(())
    }
}

impl<'a> Display for FormattedError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.write_header(f)?;

        let spacing = self.spacing();

        writeln!(f, "     |")?;

        for (idx, (line, errs)) in self.text.lines().zip(self.errors.iter()).enumerate() {
            Self::write_line(line, spacing, idx + self.first_line_number + 1, f)?;
            Self::write_errors(errs, spacing, f)?;

            writeln!(f, "     |")?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Annotation<'a> {
    pub(crate) col_number: usize,
    pub(crate) length: usize,
    pub(crate) text: &'a str,
}

#[cfg(test)]
mod tests {
    use super::*;

    mod reporting {
        // In this module, a set of "correct reports" are checked.
        use super::*;

        #[test]
        fn reporting_simple() {
            let input_file = ErrorReporter::non_file_input("hello, world".to_string());

            let hello = input_file.spanned_str().split_at(5).0;
            let comma = input_file.spanned_str().split_at(5).1.split_at(1).0;
            let world = input_file.spanned_str().split_at(7).1;

            let report = AnnotatedError::new(comma.span(), "Don't you recognize me?")
                .with_annotation(hello.span(), "Hi sweetie")
                .with_annotation(world.span(), "I am not a world!")
                .with_annotation(comma.span(), "Such cute, very comma");

            let left = input_file.format_error(&report).to_string();

            let right = "\
            Error: Don't you recognize me?\n \
             --> 1:6\n     \
                 |\n   \
               1 |                       hello, world\n     \
                 |                       ^^^^^| ^^^^^\n     \
                 | Hi sweetie------------'    | |\n     \
                 | Such cute, very comma------' |\n     \
                 | I am not a world!------------'\n     \
                 |\n\
            ";

            assert_eq!(left, right);
        }

        #[test]
        fn conjugaison_error() {
            let reporter = ErrorReporter::input_file(
                "docs.txt".to_string(),
                "The cat are on the table.".to_string(),
            );
            let file = reporter.spanned_str();

            let cat = file.split_at(4).1.split_at(3).0;
            let are = file.split_at(8).1.split_at(3).0;

            let report = AnnotatedError::new(are.span(), "Conjugation error")
                .with_annotation(cat.span(), "`cat` is singular,")
                .with_annotation(are.span(), "but `are` is used only for plural subject");

            let left = reporter.format_error(&report).to_string();

            let right = "\
            Error: Conjugation error\n \
             --> docs.txt:1:9\n     \
                 |\n   \
               1 |                                           The cat are on the table.\n     \
                 |                                               ^^^ ^^^\n     \
                 | `cat` is singular,----------------------------'   |\n     \
                 | but `are` is used only for plural subject---------'\n     \
                 |\n\
            ";

            assert_eq!(left, right);
        }

        #[test]
        fn multiline_simple() {
            let reporter = ErrorReporter::non_file_input("Hello\nWorld".into());
            let content = reporter.spanned_str();

            let hello = content.split_at(5).0;
            let world = content.split_at(6).1;

            let report = AnnotatedError::new(hello.span(), "Foo")
                .with_annotation(hello.span(), "bar")
                .with_annotation(world.span(), "baz");

            let left = reporter.format_error(&report).to_string();

            let right = "\
            Error: Foo\n \
             --> 1:1\n     \
                 |\n   \
               1 |     Hello\n     \
                 |     ^^^^^\n     \
                 | bar-'\n     \
                 |\n   \
               2 |     World\n     \
                 |     ^^^^^\n     \
                 | baz-'\n     \
                 |\n\
            ";

            assert_eq!(left, right);
        }
    }

    mod error_reporter {
        use super::*;

        #[test]
        fn code_snippet_for_single_line() {
            let foobar = "foo bar";
            let input_file = ErrorReporter::non_file_input(foobar.to_string());

            let foo = input_file.spanned_str().split_at(3).0;
            let bar = input_file.spanned_str().split_at(4).1;

            let report = AnnotatedError::new(foo.span(), "Common word found")
                .with_annotation(foo.span(), "This happens to be a common word")
                .with_annotation(bar.span(), "This too by the way");

            let (start, end) = report.bounds();

            let selected_text = input_file.code_snippet_for(start, end);

            assert_eq!(selected_text, "foo bar");
        }

        #[test]
        fn code_snippet_for_select_specific() {
            let input_text = "foo bar\nbarbar\nbazbaz";
            let input_file = ErrorReporter::non_file_input(input_text.to_string());

            let barbar = input_file.spanned_str().split_at(8).1.split_at(6).0;
            assert_eq!(barbar.content(), "barbar");
            let report = AnnotatedError::new(barbar.span(), "Found a non-existant word");

            let (start, end) = report.bounds();

            let selected_text = input_file.code_snippet_for(start, end);

            assert_eq!(selected_text, "barbar");
        }
    }
}
