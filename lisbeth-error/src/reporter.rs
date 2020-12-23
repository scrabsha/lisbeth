//! Allows to report errors to the user.
//!
//! This module contains the [`ErrorReporter`] structure, which holds metadata
//! about the input. This structure provides the metadata that are needed to
//! properly report the error to the user.

use std::{fs, io::Error as IOError};

use crate::span::{Position, Span, SpannedStr};

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

    #[allow(dead_code)]
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

    mod error_reporter {
        use super::*;

        use crate::error::AnnotatedError;

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
