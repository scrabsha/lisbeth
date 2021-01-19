//! A simple lexing system.
//!
//! This module contains traits that allow to define the various terminals of
//! the grammar.
//!
//! # Terminal definition
//!
//! There should be one user-defined type for each terminal available in the
//! grammar. These terminal types must implement the `Terminal` trait. It allows
//! to create the said terminal from a [`SpannedStr`] and contains some
//! description system that allows to automatically generate errors.

use lisbeth_error::{
    error::AnnotatedError,
    span::{Span, SpannedStr},
};

/// The result returned when lexing is done.
///
/// If lexing went successfully, then the `Ok` variant is returned. It must
/// contain the token which has been parsed, its associated span and the rest
/// of the input, in that order.
///
/// In the event of an error, the `Err` variant is returned. It is composed of
/// vector containing all the errors that happened, and if possible, the rest of
/// the input, in that order. Returning the input tail allows the lexer to
/// recover when an error is encountered.
pub type LexingResult<'a, T> =
    Result<(T, Span, SpannedStr<'a>), (Vec<AnnotatedError>, Option<SpannedStr<'a>>)>;

/// Represents a terminal in the grammar.
///
/// The grammar includes one type per terminal. Each of these type must implement
/// the `Terminal` trait.
///
/// The terminal trait is composed of one lexing function, which creates the
/// terminal from the input string, a function that describe the terminal in
/// general and a function that describe a specific terminal.
///
/// # General and specific description
///
/// Let's use the char literal example. The general description for a char
/// literal is "`a char literal`". The specific description depends on the
/// content of the char literal. For instance, it can be "`'a'`".
pub trait Terminal: Sized {
    /// Attempts to lex the terminal from an input string.
    ///
    /// If the input does not start with the said terminal, then `None` should
    /// be returned.
    fn lex(i: SpannedStr) -> Option<LexingResult<Self>>;

    /// The general description for the terminal.
    const DESCRIPTION: &'static str;

    /// Describes a specific terminal.
    fn specific_description(&self) -> String;
}
