//! Lisbeth-error
//!
//! A dead-simple error type for the lisbeth parser infrastructure.
//!
//! The types in this crate *should* be used in that order:
//!   - an [`ErrorReporter`] is created from input,
//!   - a [`SpannedStr`] is created from the [`ErrorReporter`],
//!   - parsing happens on that [`SpannedStr`],
//!   - tokens are produced, they store their position with a [`Span`],
//!   - when an error occurs, an error is reported with an [`AnnotatedError`],
//!   - this error is formatted by the [`ErrorReporter`] declared previously,
//!   which returns a [`FormattedError`],
//!   - the [`FormattedError`] is printed on the console.
//!
//! An example of usage can be found in the [handbook] module.
//!
//! [`ErrorReporter`]: reporter::ErrorReporter
//! [`SpannedStr`]: span::SpannedStr
//! [`Span`]: span::Span
//! [`AnnotatedError`]: error::AnnotatedError
//! [`FormattedError`]: reporter::FormattedError

#![deny(missing_docs, warnings)]

pub mod error;
pub mod handbook;
pub mod reporter;
pub mod span;
