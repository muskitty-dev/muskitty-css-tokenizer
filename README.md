# muskitty-css-tokenizer

[English](README.md) | [简体中文](README.zh-CN.md)

[![crates.io](https://img.shields.io/crates/v/muskitty-css-tokenizer.svg)](https://crates.io/crates/muskitty-css-tokenizer)
[![Documentation](https://docs.rs/muskitty-css-tokenizer/badge.svg)](https://docs.rs/muskitty-css-tokenizer)
[![License](https://img.shields.io/crates/l/muskitty-css-tokenizer.svg)](https://github.com/muskitty-dev/muskitty-css-tokenizer/blob/main/LICENSE)
[![CI](https://github.com/muskitty-dev/muskitty-css-tokenizer/actions/workflows/ci.yml/badge.svg)](https://github.com/muskitty-dev/muskitty-css-tokenizer/actions/workflows/ci.yml)

A from-scratch CSS tokenizer written in pure Rust, implementing the
[CSS Syntax Module Level 3 §4.3](https://drafts.csswg.org/css-syntax-3/#tokenization)
with zero runtime dependencies.

Part of the [MusKitty](https://github.com/muskitty-dev) browser engine project.

## Status

| Component | Spec Coverage | Test Pass Rate |
|-----------|---------------|----------------|
| **Tokenizer** (§4.3) | 15/15 sub-algorithms | 71/71 unit tests |

- Zero `unsafe` code
- Zero C/C++ dependencies
- Zero runtime dependencies
- Rust stable toolchain only
- MSRV 1.82

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
muskitty-css-tokenizer = "0.1.0"
```

Or run:

```bash
cargo add muskitty-css-tokenizer
```

## Quick Start

```rust
use muskitty_css_tokenizer::{CssTokenizer, Token, Tokenizer};

let mut t = CssTokenizer::new("color: red; ");
while let Some(token) = t.next_token() {
    // process token
}
```

## Architecture

```
muskitty-css-tokenizer/
  src/
    types.rs          Token, HashType, Numeric, State definitions
    trait_def.rs      Tokenizer trait
    impls.rs          CssTokenizer — recursive-descent tokenizer (~1800 lines)
    lib.rs            Public API: CssTokenizer + Tokenizer trait + types
```

### What is a CSS Tokenizer?

The CSS tokenizer is a recursive-descent algorithm (§4.3) that consumes
a stream of Unicode code points (after §5.3 preprocessing — CR/LF/FF
normalized to LF) and emits tokens. The main entry point
`consume_a_token` (§4.3.1) dispatches to sub-algorithms based on the
current input code point.

### Spec Coverage

All 15 §4.3 sub-algorithms are implemented:

- §4.3.1 Consume a token (full dispatch, incl. `unicode_ranges_allowed`)
- §4.3.2 Consume comments
- §4.3.3 Consume a numeric token
- §4.3.4 Consume an ident-like token (incl. `url(` special case)
- §4.3.5 Consume a string token
- §4.3.6 Consume a url token
- §4.3.7 Consume an escaped code point
- §4.3.8 Check if two code points are a valid escape
- §4.3.9 Check if three code points would start an ident sequence
- §4.3.10 Check if three code points would start a number
- §4.3.11 Check if three code points would start a unicode-range
- §4.3.12 Consume an ident sequence
- §4.3.13 Consume a number
- §4.3.14 Consume a unicode-range token
- §4.3.15 Consume the remnants of a bad url

## Building

```bash
cargo check
cargo build
```

## Testing

```bash
# Unit tests (71 tests)
cargo test --lib

# All tests
cargo test
```

## Design Principles

1. **CSSWG is ground truth** — Implementation follows the spec exactly.
2. **Spec-compliant, not test-compliant** — Tests verify the code; code is never modified to pass a test unless the spec proves the test is wrong.
3. **Zero runtime dependencies** — Pure safe Rust.
4. **Zero unsafe** — Pure safe Rust.
5. **Surgical changes** — Every diff is as small as the task requires.

## Spec Reference

This implementation references:

- [CSS Syntax Module Level 3](https://drafts.csswg.org/css-syntax-3/) — Primary authority
  - §4.1: Token Railroad Diagrams
  - §4.3: Tokenizer Algorithms
  - §5.3: Input Stream Preprocessing

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for details.

Copyright 2026 MusCat / MusKitty Bit-Torch Community
