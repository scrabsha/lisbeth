# Lisbeth-error


[![Build Status][actions-badge]][actions-url]
[![Latest Version][version-badge]][version-url]
[![Rust Documentation][docs-badge]][docs-url]

[actions-badge]: https://github.com/scrabsha/lisbeth/workflows/Continuous%20integration/badge.svg
[actions-url]: https://github.com/scrabsha/lisbeth/actions?query=workflow%3A%22Continuous+integration%22
[version-badge]: https://img.shields.io/crates/v/lisbeth-error.svg
[version-url]: https://crates.io/crates/lisbeth-error
[docs-badge]: https://img.shields.io/badge/docs-latest-blue.svg
[docs-url]: https://docs.rs/lisbeth-error

Span manipulation and error reporting made easy.

This crate provide datatypes that aim to make compilation and parsing easier. It
contains data structures that store text and carry with them its span and an
error reporting system.

**Note**: while the code is stored in the
[lisbeth parsing framework repository][lisbeth-github], this crate contains.
Only span- and error reporting-related code. As such, it can be used in any
project requiring span management and proper error reporting.

[lisbeth-github]: https://github.com/scrabsha/lisbeth
