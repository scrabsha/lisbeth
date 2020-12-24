//! Some tools to manipulate tuples.
//!
//! Most traits in this crate are implemented for tuples with an arity inferior
//! or equal to eight.
//!
//! # `TupleAppend`
//!
//! There is no simple way to append a value of type `C` to a tuple of type
//! `(A, B)` in rust. This is permitted by [`TupleAppend`].

mod append;

pub use append::TupleAppend;
