//! An error report with annotations
//!
//! The [`AnnotatedError`] type allows to construct error with annotations on it.

use crate::span::Span;

/// An error report with annotations.
///
/// This error report is created with the precise span at which the error occurs
/// and a simple message explaining the situation. It can then be improved by
/// adding more information.
///
/// # Example
///
/// The following code shows how to create a simple error report.
///
/// ```rust
/// use lisbeth_error::{
///     error::AnnotatedError,
///     span::{Span, SpannedStr},
/// };
///
/// let file = SpannedStr::input_file("The cat are on the table.");
///
/// let cat = file.split_at(4).1.split_at(3).0;
/// let are = file.split_at(8).1.split_at(3).0;
///
/// let report = AnnotatedError::new(are.span(), "Conjugation error")
///     .with_annotation(cat.span(), "`cat` is singular,")
///     .with_annotation(are.span(), "but `are` is used only for plural subject");
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct AnnotatedError {
    span: Span,
    msg: String,
    annotations: Vec<Annotation>,
}

impl AnnotatedError {
    /// Constructs a new report.
    ///
    /// `span` represents the precise location at which the error is
    /// encountered, `msg` describes the issue. `msg` can be either a static
    /// string slice or a `String`.
    pub fn new<Msg>(span: Span, msg: Msg) -> AnnotatedError
    where
        Msg: ToString,
    {
        let msg = msg.to_string();
        AnnotatedError {
            annotations: Vec::new(),
            span,
            msg,
        }
    }

    /// Adds a new annotation at a given span to the report.
    pub fn with_annotation<Msg>(mut self, span: Span, msg: Msg) -> AnnotatedError
    where
        Msg: ToString,
    {
        let content = msg.to_string();
        let ann = Annotation { span, content };
        self.annotations.push(ann);
        self
    }

    /// Returns the span at which the error is encountered.
    pub fn span(&self) -> Span {
        self.span
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Annotation {
    span: Span,
    content: String,
}
