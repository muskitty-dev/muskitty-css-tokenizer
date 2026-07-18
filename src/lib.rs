//! MusKitty CSS Tokenizer
//!
//! Implements the tokenization stage of the CSS Syntax Module Level 3
//! (§4.3 "Tokenizer Algorithms"). Extracted from muskitty-css as a
//! standalone crate for independent versioning and publication.
//!
//! The tokenizer consumes a stream of Unicode code points (after §5.3
//! preprocessing) and emits [`Token`]s. These tokens are consumed by
//! downstream parsers (e.g. `muskitty-css`'s parser, `muskitty-selectors`)
//! to build CSS objects or selector ASTs.
//!
//! # References
//!
//! - CSS Syntax Module Level 3: <https://drafts.csswg.org/css-syntax-3/>
//! - Spec source (Markdown): `D:\CSSWG\css-syntax-3\Overview.md`

mod impls;
mod trait_def;
mod types;

pub use impls::CssTokenizer;
pub use trait_def::Tokenizer;
pub use types::{HashType, Numeric, State, Token};
