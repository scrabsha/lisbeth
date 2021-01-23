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
//!
//! Terminals should not contain the span at which it was lexed.
//!
//! # Token definition
//!
//! A token represents one of the terminal of the grammar. It is created with
//! the [`token`] macro, and automatically implements the [`Token`] trait, which
//! allows the lexer to correctly handle it. It also holds the span at which the
//! terminal was encountered.

use std::convert::TryFrom;

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

fn incorrect_terminal_error(span: Span, expected: &str, got: String) -> AnnotatedError {
    AnnotatedError::new(span, format!("Expected {}, found {}", expected, got))
}

// Allows Token -> Terminal conversion.
//
// This trait should be implemented by the token macro.
#[doc(hidden)]
pub trait Tokenizeable<T: Token>: Sized + Terminal {
    fn from_token(tok: &T) -> Option<Self>;

    fn from_token_or_error(tok: &T) -> Result<Self, AnnotatedError> {
        match Self::from_token(tok) {
            Some(t) => Ok(t),
            None => {
                let report =
                    incorrect_terminal_error(tok.span(), Self::DESCRIPTION, tok.describe());
                Err(report)
            }
        }
    }
}

/// Represents a token (eg: a combinaison of terminal).
///
/// This trait should not be implemented manually. The [`token`] macro should be
/// used instead.
///
/// A token is composed of a [`Span`] and one of the terminal defined in the
/// grammar.
pub trait Token: Sized {
    /// Lexes a single token from the input.
    fn from_str(
        input: SpannedStr,
    ) -> Result<(Self, SpannedStr), (Vec<AnnotatedError>, Option<SpannedStr>)>;

    /// Returns the token span.
    fn span(&self) -> Span;

    /// Returns a description of the terminal stored in the token.
    ///
    /// This description corresponds to the [`specific_description`] from the
    /// [`Terminal`] trait.
    ///
    /// [`specific_description`]: Terminal::specific_description
    fn describe(&self) -> String;
}

/// Creates a token type and implements [`Token`] for it.
///
/// This macro generates most of the boilerplate required so that the token
/// defined can be used in the [`Lexer`].
///
/// Documentation and `#[derive(...)]` macros can be added on the token by
/// passing them before the token name.
///
/// # Example
///
/// The following example shows how to define a simple token representing the
/// [morse code][morse_wikipedia] token.
///
/// [morse_wikipedia]: https://en.wikipedia.org/wiki/Morse_code
///
/// ```rust
/// use lisbeth_error::span::{Span, SpannedStr};
///
/// use lisbeth_parser::lexer::{LexingResult, Terminal};
/// use lisbeth_parser::token;
/// #
/// # fn lex_single_char(i: SpannedStr, chr: char) -> Option<(Span, SpannedStr)> {
/// #     if i.content().starts_with(chr) {
/// #         let (matched, tail) = i.split_at(chr.len_utf8());
/// #         Some((matched.span(), tail))
/// #     } else {
/// #         None
/// #     }
/// # }
///
/// #[derive(Clone, Debug, PartialEq)]
/// struct Dot;
///
/// impl Terminal for Dot {
/// #     const DESCRIPTION: &'static str = "`.`";
/// #
/// #     fn specific_description(&self) -> String {
/// #         Self::DESCRIPTION.to_string()
/// #     }
/// #
/// #     fn lex(i: SpannedStr) -> Option<LexingResult<Self>> {
/// #         let (span, tail) = lex_single_char(i, '.')?;
/// #         let d = Dot;
/// #
/// #         Some(Ok((d, span, tail)))
/// #     }
///     // Implementation details hidden.
/// }
///
/// #[derive(Clone, Debug, PartialEq)]
/// struct Dash;
///
/// impl Terminal for Dash {
/// #     const DESCRIPTION: &'static str = "`-`";
/// #
/// #     fn specific_description(&self) -> String {
/// #         Self::DESCRIPTION.to_string()
/// #     }
/// #
/// #     fn lex(i: SpannedStr) -> Option<LexingResult<Self>> {
/// #         let (span, tail) = lex_single_char(i, '-')?;
/// #         let d = Dash;
/// #
/// #         Some(Ok((d, span, tail)))
/// #     }
///     // Implementation details hidden.
/// }
///
/// token! {
///     /// A token for the morse language.
///     #[derive(Clone, Debug, PartialEq)]
///     Token = Dot | Dash
/// }
/// ```
#[macro_export]
macro_rules! token {
    (
        $( #[$m:meta] )*
        $token_name: ident =
            $( $term: ident )|* $(,)?
    ) => {
        ::paste::paste! {
            // Token type generation
            $( #[$m] )*
            struct $token_name {
                kind: [<$token_name Kind>],
                span: ::lisbeth_error::span::Span,
            }

            // Token kind type generation
            $( #[$m] )*
            enum [<$token_name Kind>] {
                $( $term($term), )*
            }

            // Faillible Token -> Terminal conversion
            $(
                impl $crate::lexer::Tokenizeable<$token_name> for $term {
                    fn from_token(tok: &$token_name) -> Option<Self> {
                        match &tok.kind {
                            [<$token_name Kind>]::$term(t) => Some(t.clone()),
                            _ => None,
                        }
                    }
                }
             )*

            impl $crate::lexer::Token for $token_name {
                fn from_str(
                    input: ::lisbeth_error::span::SpannedStr,
                ) -> Result<
                    (Self, ::lisbeth_error::span::SpannedStr),
                    (Vec<::lisbeth_error::error::AnnotatedError>, Option<::lisbeth_error::span::SpannedStr>)
                > {
                    // Trying to parse with every terminal until one of them
                    // succeed.
                    $(
                        if let Some(rslt) = $term::lex(input) {
                            let (term, span, tail) = rslt?;
                            let kind = [<$token_name Kind>] ::$term(term);
                            let tok = $token_name { kind, span };
                            return Ok((tok, tail));
                        }
                     )*

                    // If no token matched, then a failure is emitted.
                    let mut first = true;
                    let (chr, _) = input.take_while(|_| if first { first = false; true } else { false });

                    let report = ::lisbeth_error::error::AnnotatedError::new(chr.span(), format!("Unknown start of token: `{}`", chr.content()))
                        .with_annotation(chr.span(), "Unknown start of token");
                    let reports = vec![report];

                    Err((reports, None))
                }

                #[inline]
                fn span(&self) -> Span {
                    self.span
                }

                fn describe(&self) -> String {
                    match &self.kind {
                        $(
                            [<$token_name Kind>] ::$term(t) => t.specific_description(),
                        )*
                    }
                }
            }
        }
    };
}

/// Represents a sequence of tokens.
///
/// This structure is created thanks to a [SpannedStr].
#[derive(Clone, Debug, PartialEq)]
pub struct Lexer<Tok>(Vec<Tok>);

impl<Tok: Token> Lexer<Tok> {
    fn from_spanned_str(mut input: SpannedStr) -> Result<Lexer<Tok>, Vec<AnnotatedError>> {
        let mut toks = Vec::<Tok>::new();
        let mut errs = Vec::<AnnotatedError>::new();

        while !input.content().is_empty() {
            match Tok::from_str(input) {
                Ok((tok, tail)) => {
                    // Everything went correctly
                    toks.push(tok);
                    input = tail;
                }
                Err((es, Some(tail))) => {
                    // Some errors, but we still know where to resume
                    errs.extend(es);
                    input = tail;
                }
                Err((es, None)) => {
                    // Some errors, but we don't know where to resume
                    errs.extend(es);
                    return Err(errs);
                }
            }
        }

        if errs.is_empty() {
            Ok(Lexer(toks))
        } else {
            Err(errs)
        }
    }
}

impl<'a, Tok> TryFrom<SpannedStr<'a>> for Lexer<Tok>
where
    Tok: Token + 'a,
{
    type Error = Vec<AnnotatedError>;

    fn try_from(input: SpannedStr) -> Result<Self, Self::Error> {
        Self::from_spanned_str(input)
    }
}

#[cfg(test)]
mod lexer {
    use super::*;

    mod from_spanned_str {
        use super::*;

        #[derive(Clone, Debug, PartialEq)]
        struct Dash;
        #[derive(Clone, Debug, PartialEq)]
        struct Dot;

        impl Terminal for Dash {
            fn lex(i: SpannedStr) -> Option<LexingResult<Self>> {
                if i.content().starts_with('-') {
                    let (matched, tail) = i.split_at(1);
                    let d = Dash;
                    let s = matched.span();
                    return Some(Ok((d, s, tail)));
                }

                if i.content().starts_with('_') {
                    let (matched, tail) = i.split_at(1);
                    let report = AnnotatedError::new(matched.span(), "Expected `-`, found `_`");
                    return Some(Err((vec![report], Some(tail))));
                }

                return None;
            }

            fn specific_description(&self) -> String {
                "`-`".to_string()
            }

            const DESCRIPTION: &'static str = "`-`";
        }

        impl Terminal for Dot {
            fn lex(i: SpannedStr) -> Option<LexingResult<Self>> {
                if i.content().starts_with('.') {
                    let (matched, tail) = i.split_at(1);
                    let d = Dot;
                    let s = matched.span();
                    return Some(Ok((d, s, tail)));
                }

                return None;
            }

            fn specific_description(&self) -> String {
                "`.`".to_string()
            }

            const DESCRIPTION: &'static str = "`.`";
        }

        token! {
            #[derive(Debug, PartialEq)]
            MorseToken = Dash | Dot
        }

        #[test]
        fn working_case() {
            let input = SpannedStr::input_file("...---...");
            let l = Lexer::<MorseToken>::from_spanned_str(input);

            assert!(l.is_ok());

            let left_parsed_tokens = l.unwrap().0.into_iter().map(|t| t.kind).collect::<Vec<_>>();

            let right_parsed_tokens = vec![
                MorseTokenKind::Dot(Dot),
                MorseTokenKind::Dot(Dot),
                MorseTokenKind::Dot(Dot),
                MorseTokenKind::Dash(Dash),
                MorseTokenKind::Dash(Dash),
                MorseTokenKind::Dash(Dash),
                MorseTokenKind::Dot(Dot),
                MorseTokenKind::Dot(Dot),
                MorseTokenKind::Dot(Dot),
            ];

            assert_eq!(left_parsed_tokens, right_parsed_tokens);
        }

        #[test]
        fn error_with_recovery() {
            let input = SpannedStr::input_file("__");
            let l = Lexer::<MorseToken>::from_spanned_str(input);

            // We ensure that the recovery actually happened because there
            // are two errors.
            assert_eq!(l.unwrap_err().len(), 2);
        }

        #[test]
        fn fatal_error() {
            let input = SpannedStr::input_file("||");
            let l = Lexer::<MorseToken>::from_spanned_str(input);

            // | can not be recovered from, so there should be a single error
            assert_eq!(l.unwrap_err().len(), 1);
        }
    }
}
