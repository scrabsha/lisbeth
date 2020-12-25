# Lisbeth-tuple-tools


[![Build Status][actions-badge]][actions-url]
[![Latest Version][version-badge]][version-url]
[![Rust Documentation][docs-badge]][docs-url]

[actions-badge]: https://github.com/scrabsha/lisbeth/workflows/Continuous%20integration/badge.svg
[actions-url]: https://github.com/scrabsha/lisbeth/actions?query=workflow%3A%22Continuous+integration%22
[version-badge]: https://img.shields.io/crates/v/lisbeth-tuple-tools.svg
[version-url]: https://crates.io/crates/lisbeth-tuple-tools
[docs-badge]: https://img.shields.io/badge/docs-latest-blue.svg
[docs-url]: https://docs.rs/lisbeth-tuple-tools

A set of tools for tuples.

All the described tools are implemented for tuples with arity smaller or equal
to 8.

The `TupleAppend` trait allows to append a value at the end of a tuple of any
arity.

The `TupleMap*` traits allow to map the nth element of a tuple from one type
to an other.

**Note**: while the code is stored in the
[lisbeth parsing framework repository][lisbeth-github], this crate does not
contain any parsing-related code. As such, it may be used in any project not
linked to parsing.

[lisbeth-github]: https://github.com/scrabsha/lisbeth
