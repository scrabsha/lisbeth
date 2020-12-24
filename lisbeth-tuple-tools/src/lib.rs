//! Some tools to manipulate tuples.
//!
//! Most traits in this crate are implemented for tuples with an arity inferior
//! or equal to eight.
//!
//! # `TupleAppend`
//!
//! There is no simple way to append a value of type `C` to a tuple of type
//! `(A, B)` in rust. This is permitted by [`TupleAppend`].
//!
//! ## Example
//!
//! Here, we append a [`char`] to a `(char, u32)`:
//!
//! ```rust
//! use lisbeth_tuple_tools::TupleAppend;
//!
//! let tup = ('l', 42).append('s');
//!
//! assert_eq!(tup, ('l', 42, 's'));
//! ```
//!
//! # `TupleMap*`
//!
//! This crate contains [`TupleMap1`], [`TupleMap2`], and so on. These traits
//! provide functions that allow to map a single element of a tuple from one
//! type to an other.

mod append;

pub use append::TupleAppend;
