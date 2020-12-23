//! An error report with annotations
//!
//! The [`AnnotatedError`] type allows to construct error with annotations on it.

use std::iter;

use crate::{
    reporter::Annotation as ReportedAnnotation,
    span::{Position, Span},
};

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
    pub(crate) span: Span,
    pub(crate) msg: String,
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

    fn all_spans<'a>(&'a self) -> impl Iterator<Item = Span> + 'a {
        self.annotations
            .iter()
            .map(|a| a.span)
            .chain(iter::once(self.span))
    }

    pub(crate) fn bounds(&self) -> (Position, Position) {
        // These two unwraps won't panic because all_spans returns an iterator
        // that contains at least the span at which the error happened.
        let min = self.all_spans().map(Span::start).min().unwrap();
        let max = self.all_spans().map(Span::end).max().unwrap();

        (min, max)
    }

    pub(crate) fn error_matrix<'a>(
        &'a self,
    ) -> Vec<Vec<ReportedAnnotation<'a>>> {
        let (start_pos, end_pos) = self.bounds();

        let (first_line_number, last_line_number) = (start_pos.line() as usize, end_pos.line() as usize);
        let total_line_number = last_line_number - first_line_number + 1;

        let mut matrix = (0..total_line_number)
            .map(|_| Vec::new())
            .collect::<Vec<_>>();

        for annotation in self.annotations.iter() {
            let line_idx = annotation.span.start().line() as usize - first_line_number;

            assert_eq!(
                annotation.span.start().line(),
                annotation.span.end().line(),
                "Multiline spans are not supported",
            );

            let col_number = annotation.span.start().col() as usize;
            let length = annotation.span.end().col() as usize - col_number;
            let text = annotation.content.as_str();

            let ann = ReportedAnnotation {
                col_number,
                length,
                text,
            };
            matrix[line_idx].push(ann);
        }

        matrix
            .iter_mut()
            .for_each(|anns| anns.sort_by_key(|a| a.col_number));

        matrix
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Annotation {
    span: Span,
    content: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    mod annotated_error {
        use super::*;

        use crate::span::SpannedStr;

        #[test]
        fn bounds_double() {
            let input = SpannedStr::input_file("ab");
            let (a, b) = input.split_at(1);

            let report = AnnotatedError::new(a.span(), "Some generic message")
                .with_annotation(a.span(), "ann1")
                .with_annotation(b.span(), "ann2");

            let (start, end) = report.bounds();

            assert_eq!(start.offset(), 0);
            assert_eq!(end.offset(), 2);

            assert_eq!(start.col(), 0);
            assert_eq!(end.col(), 2);

            assert_eq!(start.line(), 0);
            assert_eq!(end.line(), 0);
        }

        #[test]
        fn bounds_single() {
            let input = SpannedStr::input_file("ab");

            let report = AnnotatedError::new(input.span(), "Some generic message");

            let (start, end) = report.bounds();

            assert_eq!(start.offset(), 0);
            assert_eq!(end.offset(), 2);

            assert_eq!(start.col(), 0);
            assert_eq!(end.col(), 2);

            assert_eq!(start.line(), 0);
            assert_eq!(end.line(), 0);
        }

        #[test]
        fn error_matrix_for() {
            // In this text, there is a line that gets ignored because it has
            // no annotation in it, and two lines that contain one and two
            // annotations respectively.

            let input_file = SpannedStr::input_file("line 1\nline 2\nline3");

            let l1 = input_file.split_at(6).0;
            assert_eq!(l1.content(), "line 1");
            let l2 = input_file.split_at(7).1.split_at(6).0;
            assert_eq!(l2.content(), "line 2");
            let num = input_file.split_at(12).1.split_at(1).0;
            assert_eq!(num.content(), "2");

            let report = AnnotatedError::new(l1.span(), "<insert general message here>")
                .with_annotation(l1.span(), "first line")
                .with_annotation(l2.span(), "second line")
                .with_annotation(num.span(), "second line, but better");

            let matrix = report.error_matrix();

            assert_eq!(matrix.len(), 2);
            assert_eq!(matrix[0].len(), 1);
            assert_eq!(matrix[1].len(), 2);

            assert!(matrix[1][0].col_number < matrix[1][1].col_number);
        }
    }
}
