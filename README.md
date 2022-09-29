# mdbook-chapter-number

![CI](https://github.com/Mura-Mi/mdbook-chapter-number/actions/workflows/ci.yml/badge.svg)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![crates.io](https://img.shields.io/crates/v/mdbook-chapter-number.svg)](https://crates.io/crates/mdbook-chapter-number)

This is a [mdbook](https://rust-lang.github.io/mdBook/) preprocessor that adds chapter numbers to the each page header.

## Usage
You can install this preprocessor with `cargo install mdbook-chapter-number`.

Then, add the following to your `book.toml`:

```toml
[book]
title = "Example book"

[preprocessor.chapter-number]
renderer = ["html"]
```