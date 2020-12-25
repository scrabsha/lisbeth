<div class="title-block" style="text-align: center;" align="center">

# Lisbeth

A parsing framework written in rust\*.<br>
<sub>\*Some assembly required.</sub>

</div>

This repository contains traits and data structures that ease the creation of
parsers with good ability to recover and good error reporting.

## Content

This repository contains several crates. The goal is to maximize modularity.
The ultimate goal would be to propose additional backends for other components,
such as the [annotate-snippet] crate.

### Lisbeth-error

[![Latest Version][error-version-badge]][error-version-url]
[![Rust Documentation][error-docs-badge]][error-docs-url]

This crate provides span management primitives. These spans allow, if a failure
occurs, to generate a comprehensible error report which is displayed to the
user. Its code can be found in the [lisbeth-error subfolder][error-subrepo].

### Lisbeth-tuple-tools

[![Latest Version][tuple-tools-version-badge]][tuple-tools-version-url]
[![Rust Documentation][tuple-tools-docs-badge]][tuple-tools-docs-url]

This crate provides tools to manipulate tuples, such as appending an element at
the end of a tuple and mapping a specific element of a tuple. Its code can be
found in the [lisbeth-tuple-tools subfolder][tuple-tools-subrepo].

It does not contain any parsing-related code. As such, it can be used
independently from the rest of the code in this repository.

## License

The code under this repository is licensed under the
<a href="LICENSE">MIT license</a>.

[annotate-snippet]: https://crates.io/crates/annotate-snippets

[error-version-badge]: https://img.shields.io/crates/v/lisbeth-error.svg
[error-version-url]: https://crates.io/crates/lisbeth-error
[error-docs-badge]: https://img.shields.io/badge/docs-latest-blue.svg
[error-docs-url]: https://docs.rs/lisbeth-error
[error-subrepo]: lisbeth-error

[tuple-tools-version-badge]: https://img.shields.io/crates/v/lisbeth-tuple-tools.svg
[tuple-tools-version-url]: https://crates.io/crates/lisbeth-tuple-tools
[tuple-tools-docs-badge]: https://img.shields.io/badge/docs-latest-blue.svg
[tuple-tools-docs-url]: https://docs.rs/lisbeth-tuple-tools
[tuple-tools-subrepo]: lisbeth-tuple-tools
