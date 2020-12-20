//! Lisbeth-error
//!
//! A dead-simple error type for the lisbeth parser infrastructure.
//!
//! This type allows provides the `ErrorReport` type, that allows to annotate
//! input code in order to explain what happened to the user. Annotations are
//! added to the report by providing a span and an associated message.
//!
//! Spans are conveyed thanks to the `SpannedStr` type, which holds the a string
//! slice and its position in the input stream.

#![forbid(missing_docs, warnings)]

pub mod error;
pub mod reporter;
pub mod span;
