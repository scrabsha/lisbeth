//! A simple example explaining how to parse input data and emit errors.
//!
//! This page shows how to parse a sequence of space-separated decimal numbers.
//! Let's first define the `Number` type, and a simple parser for it:
//!
//! ```rust
//! use lisbeth_error::{
//!     span::{Span, SpannedStr},
//!     error::AnnotatedError,
//! };
//!
//! struct Number {
//!     span: Span,
//!     value: u32,
//! }
//!
//! // This will be usefull for our tests
//! impl PartialEq<u32> for Number {
//!     fn eq(&self, other: &u32) -> bool {
//!         self.value == *other
//!     }
//! }
//!
//! fn number<'a>(input: SpannedStr<'a>) -> Result<(Number, SpannedStr<'a>), AnnotatedError> {
//!     let (matched, tail) = input.take_while(char::is_numeric);
//!
//!     if matched.content().is_empty() {
//!         let mut first = true;
//!         let err_span = input.take_while(|c| {
//!             let tmp = !c.is_whitespace() || first;
//!             first = false;
//!             tmp
//!         }).0;
//!         let report = AnnotatedError::new(err_span.span(), "Expected number")
//!             .with_annotation(
//!                 err_span.span(),
//!                 format!("Expected number, found `{}`.", err_span.content()),
//!             );
//!
//!         return Err(report);
//!     }
//!
//!     let value = matched.content().parse().unwrap();
//!     let span = matched.span();
//!
//!     let number = Number { span, value };
//!
//!     Ok((number, tail))
//! }
//! ```
//!
//! Now, we need to parse spaces. Let's do so:
//!
//! ```rust
//! # use lisbeth_error::{
//! #     span::{Span, SpannedStr},
//! #     error::AnnotatedError,
//! # };
//! #
//! fn space<'a>(input: SpannedStr<'a>) -> Result<SpannedStr<'a>, AnnotatedError> {
//!     let first_char = input.content().chars().next();
//!
//!     let first_char = match first_char {
//!         Some(chr) => chr,
//!         None => {
//!             let report = AnnotatedError::new(
//!                 input.span(),
//!                 "Expected ` `, found EOF"
//!             );
//!             return Err(report);
//!         },
//!     };
//!
//!     if first_char != ' ' {
//!         let err_span = input.split_at(first_char.len_utf8()).0;
//!
//!         let report = AnnotatedError::new(err_span.span(), "Expected ` `.")
//!             .with_annotation(
//!                 err_span.span(),
//!                 format!("Expected ` `, found `{}`.", err_span.content()),
//!             );
//!
//!         return Err(report);
//!     }
//!
//!     let (_, tail) = input.split_at(1);
//!     Ok(tail)
//! }
//! ```
//!
//! Once we can parse both a number and a space, let's define `ssn` (short for
//! "space-separated numbers"), that will call our parsers in the correct order:
//!
//! ```rust
//! # use lisbeth_error::{
//! #     span::{Span, SpannedStr},
//! #     error::AnnotatedError,
//! # };
//! #
//! # struct Number;
//! #
//! # fn number<'a>(input: SpannedStr<'a>) -> Result<(Number, SpannedStr<'a>), AnnotatedError> {
//! #     todo!();
//! # }
//! #
//! # fn space<'a>(input: SpannedStr<'a>) -> Result<SpannedStr<'a>, AnnotatedError> {
//! #     todo!();
//! # }
//! fn ssn<'a>(mut input: SpannedStr<'a>) -> Result<Vec<Number>, AnnotatedError> {
//!     let mut nbrs = Vec::new();
//!
//!     while !input.content().is_empty() {
//!         let (nbr, tail) = number(input)?;
//!         nbrs.push(nbr);
//!
//!         // Return if nothing is left
//!         if tail.content().is_empty() {
//!             break;
//!         }
//!
//!         let tail = space(input)?;
//!
//!         input = tail;
//!     }
//!    
//!     Ok(nbrs)
//! }
//! ```
//!
//! Let's test `ssn`:
//!
//! ```rust
//! # use lisbeth_error::{
//! #     span::{Span, SpannedStr},
//! #     error::AnnotatedError,
//! # };
//! #
//! # #[derive(Debug, PartialEq)]
//! # struct Number {
//! #     span: Span,
//! #     value: u32,
//! # }
//! #
//! # impl PartialEq<u32> for Number {
//! #    fn eq(&self, other: &u32) -> bool {
//! #        self.value == *other
//! #    }
//! # }
//! #
//! # fn number<'a>(input: SpannedStr<'a>) -> Result<(Number, SpannedStr<'a>), AnnotatedError> {
//! #     let (matched, tail) = input.take_while(char::is_numeric);
//! #
//! #     if matched.content().is_empty() {
//! #         let mut first = true;
//! #         let err_span = input.take_while(|c| {
//! #             let tmp = !c.is_whitespace() || first;
//! #             first = false;
//! #             tmp
//! #         }).0;
//! #
//! #         let report = AnnotatedError::new(err_span.span(), "Expected number")
//! #             .with_annotation(
//! #                 err_span.span(),
//! #                 format!("Expected number, found `{}`.", err_span.content()),
//! #             );
//! #
//! #         return Err(report);
//! #     }
//! #
//! #     let value = matched.content().parse().unwrap();
//! #     let span = matched.span();
//! #
//! #     let number = Number { span, value };
//! #
//! #     Ok((number, tail))
//! # }
//! #
//! # fn space<'a>(input: SpannedStr<'a>) -> Result<SpannedStr<'a>, AnnotatedError> {
//! #     let first_char = input.content().chars().next();
//! #
//! #     let first_char = match first_char {
//! #         Some(chr) => chr,
//! #         None => {
//! #             let report = AnnotatedError::new(input.span(), "Expected ` `, found EOF.");
//! #             return Err(report);
//! #         },
//! #     };
//! #
//! #     if first_char != ' ' {
//! #         let err_span = input.split_at(first_char.len_utf8()).0;
//! #
//! #         let report = AnnotatedError::new(err_span.span(), "Expected ` `")
//! #             .with_annotation(
//! #                 err_span.span(),
//! #                 format!("Expected ` `, found `{}`.", err_span.content()),
//! #             );
//! #
//! #         return Err(report);
//! #     }
//! #
//! #     let (_, tail) = input.split_at(1);
//! #     Ok(tail)
//! # }
//! # fn ssn<'a>(mut input: SpannedStr<'a>) -> Result<Vec<Number>, AnnotatedError> {
//! #     let mut nbrs = Vec::new();
//! #
//! #     while !input.content().is_empty() {
//! #         let (nbr, tail) = number(input)?;
//! #         nbrs.push(nbr);
//! #
//! #         // Return if nothing is left
//! #         if tail.content().is_empty() {
//! #             break;
//! #         }
//! #
//! #         let tail = space(tail)?;
//! #
//! #         input = tail;
//! #     }
//! #
//! #     Ok(nbrs)
//! # }
//! #
//! let input = SpannedStr::input_file("42 101 13");
//! assert_eq!(ssn(input).unwrap(), [42, 101, 13]);
//! ```
//!
//! Now we need to be able to display errors to stderr:
//! ```rust
//! # use lisbeth_error::{
//! #     span::{Span, SpannedStr},
//! #     error::AnnotatedError,
//! # };
//! #
//! # #[derive(Debug, PartialEq)]
//! # struct Number {
//! #     span: Span,
//! #     value: u32,
//! # }
//! #
//! # impl PartialEq<u32> for Number {
//! #    fn eq(&self, other: &u32) -> bool {
//! #        self.value == *other
//! #    }
//! # }
//! #
//! # fn number<'a>(input: SpannedStr<'a>) -> Result<(Number, SpannedStr<'a>), AnnotatedError> {
//! #     let (matched, tail) = input.take_while(char::is_numeric);
//! #
//! #     if matched.content().is_empty() {
//! #         let mut first = true;
//! #         let err_span = input.take_while(|c| {
//! #             let tmp = !c.is_whitespace() || first;
//! #             first = false;
//! #             tmp
//! #         }).0;
//! #
//! #         let report = AnnotatedError::new(err_span.span(), "Expected number")
//! #             .with_annotation(
//! #                 err_span.span(),
//! #                 format!("Expected number, found `{}`.", err_span.content()),
//! #             );
//! #
//! #         return Err(report);
//! #     }
//! #
//! #     let value = matched.content().parse().unwrap();
//! #     let span = matched.span();
//! #
//! #     let number = Number { span, value };
//! #
//! #     Ok((number, tail))
//! # }
//! #
//! # fn space<'a>(input: SpannedStr<'a>) -> Result<SpannedStr<'a>, AnnotatedError> {
//! #     let first_char = input.content().chars().next();
//! #
//! #     let first_char = match first_char {
//! #         Some(chr) => chr,
//! #         None => {
//! #             let report = AnnotatedError::new(input.span(), "Expected ` `, found EOF.");
//! #             return Err(report);
//! #         },
//! #     };
//! #
//! #     if first_char != ' ' {
//! #         let err_span = input.split_at(first_char.len_utf8()).0;
//! #
//! #         let report = AnnotatedError::new(err_span.span(), "Expected ` `")
//! #             .with_annotation(
//! #                 err_span.span(),
//! #                 format!("Expected ` `, found `{}`.", err_span.content()),
//! #             );
//! #
//! #         return Err(report);
//! #     }
//! #
//! #     let (_, tail) = input.split_at(1);
//! #     Ok(tail)
//! # }
//! # fn ssn<'a>(mut input: SpannedStr<'a>) -> Result<Vec<Number>, AnnotatedError> {
//! #     let mut nbrs = Vec::new();
//! #
//! #     while !input.content().is_empty() {
//! #         let (nbr, tail) = number(input)?;
//! #         nbrs.push(nbr);
//! #
//! #         // Return if nothing is left
//! #         if tail.content().is_empty() {
//! #             break;
//! #         }
//! #
//! #         let tail = space(tail)?;
//! #
//! #         input = tail;
//! #     }
//! #
//! #     Ok(nbrs)
//! # }
//! use lisbeth_error::reporter::ErrorReporter;
//!
//! // We intentionnaly don't return anything, so that the code stays as short
//! // as possible.
//! fn parse(file_name: String, content: String) {
//!     let file = ErrorReporter::input_file(
//!         file_name.to_string(),
//!         content.to_string(),
//!     );
//!
//!     match ssn(file.spanned_str()) {
//!         Ok(numbers) => println!("Parsing successfull"),
//!         Err(e) => eprintln!("{}", file.format_error(&e)),
//!
//!     }
//! }
//! ```
//!
//! # Error produced
//!
//! When the input file contains a word:
//!
//! ```none
//! Error: Expected number
//!  --> numbers.ssn:1:7
//!      |
//!    1 |                               42 31 abc 101
//!      |                                     ^^^
//!      | Expected number, found `abc`.-------'
//!      |
//! ```
//!
//! When the input file contains too much space:
//!
//! ```none
//! Error: Expected number
//!  --> numbers.ssn:1:4
//!      |
//!    1 |                               42  31 101
//!      |                                  ^^^
//!      | Expected number, found ` 31`.----'
//!      |
//! ```
//!
//! The `found ' 31'` message can be improved by determining a better
//! `err_span` in the `number` function.
