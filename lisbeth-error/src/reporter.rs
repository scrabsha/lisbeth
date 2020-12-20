//! Allows to report errors to the user.
//!
//! This module contains the [`ErrorReporter`] structure, which holds metadata
//! about the input. This structure provides the metadata that are needed to
//! properly report the error to the user.

use std::{fs, io::Error as IOError};

use crate::span::{Span, SpannedStr};

/// Holds metadata about the input, allows to report errors to the user.
///
/// This structure should be created before the parsing process and will provide
/// metadata required for the [`AnnotatedReport`].
///
/// If the reporter contains a file path, then this path must be valid UTF-8.
/// This is needed because the file path is printed on the console when an error
/// is reported.
///
/// [`AnnotatedReport`]: [super::error::AnnotatedError]
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
}
