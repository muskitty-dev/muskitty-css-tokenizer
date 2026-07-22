//! Tokenizer trait definition.
//!
//! CSS Syntax Module Level 3 §4.3 "Tokenizer Algorithms".

use crate::types::{State, Token};

/// The CSS tokenizer.
///
/// Implements the tokenization algorithm of CSS Syntax §4.3. The tokenizer
/// consumes Unicode code points from an input stream (preprocessed per
/// §5.3 to normalize newlines) and emits [`Token`]s.
///
/// # Algorithm structure
///
/// Unlike the HTML tokenizer's explicit state machine, the CSS tokenizer
/// is recursive-descent: a single entry point `consume_a_token` (§4.3.1)
/// dispatches to sub-algorithms (`consume_an_ident_like_token` §4.3.4,
/// `consume_a_numeric_token` §4.3.3, `consume_a_string_token` §4.3.5,
/// `consume_a_url_token` §4.3.6) which in turn call primitives
/// (`consume_an_escaped_code_point` §4.3.7, `consume_an_ident_sequence`
/// §4.3.12, `consume_a_number` §4.3.13).
///
/// # Reentrancy
///
/// The CSS tokenizer is *not* reentrant in the same way the HTML
/// tokenizer is: CSS has no analogue of HTML's content-model switching
/// (RCDATA / RAWTEXT / ScriptData). The state is therefore minimal —
/// just "consuming" vs. "EOF emitted". Future CSSOM incremental-parsing
/// use cases may require reentrancy; the trait exposes
/// [`state`](Tokenizer::state) and [`set_state`](Tokenizer::set_state)
/// to support that future need.
pub trait Tokenizer {
    /// Consume and return the next token from the input stream.
    ///
    /// Implements the §4.3.1 "Consume a token" algorithm at the top level.
    /// Returns `Some(token)` for each token, ending with
    /// `Some(Token::Eof)` for `<EOF-token>` (§5.3). After `<EOF-token>`
    /// has been emitted, returns `None`.
    fn next_token(&mut self) -> Option<Token>;

    /// Return the current tokenizer state.
    fn state(&self) -> State;

    /// Set the current tokenizer state.
    ///
    /// Currently only useful for resetting to [`State::Data`] after EOF,
    /// which re-enables token production (though the input stream is
    /// already exhausted, so this is a no-op in practice).
    fn set_state(&mut self, state: State);

    /// Reset the tokenizer to its initial state over the same input.
    ///
    /// Clears any partial token state and resets position to 0. Used by
    /// future CSSOM incremental parsing and by test harnesses.
    fn reset(&mut self);

    /// §4.3.1 L782-783: Set the `unicode_ranges_allowed` flag.
    ///
    /// Defaults to `false`. The `U+`/`u+` branch of §4.3.1 only produces a
    /// `<unicode-range-token>` (§4.3.14) when this flag is `true`; otherwise
    /// `U`/`u` is tokenized as an ident-like token. Per §4.3.14 L1500-1506,
    /// unicode-range tokens are not produced by the top-level tokenizer under
    /// normal circumstances — the flag is set to `true` only when parsing the
    /// value of the `@font-face/unicode-range` descriptor.
    fn set_unicode_ranges_allowed(&mut self, allowed: bool);

    /// 当前 position（tokenizer 内部的字符索引，0-based）。
    ///
    /// 用于 [`next_token_with_span`](Self::next_token_with_span) 追踪 token
    /// 的 source range。默认实现返回 0；具体 tokenizer 应覆盖。
    ///
    /// 注意：对 `CssTokenizer` 而言，这是 `Vec<char>` 上的 **char 索引**，
    /// 不是 byte offset。调用方需要自行将 char 索引映射到 byte offset。
    fn position(&self) -> usize {
        0
    }

    /// 返回下一个 token 及其在输入流中的字符范围 `[start, end)`。
    ///
    /// 默认实现基于 [`position`](Self::position) + [`next_token`](Self::next_token)：
    /// 记录消费前的 position 作为 `start`，消费后的 position 作为 `end`。
    /// 具体 tokenizer 可覆盖以获得更精确的 span。
    ///
    /// 返回的 range 是 **char 索引**（不是 byte offset），与
    /// [`position`](Self::position) 一致。EOF token 的 span 是空 range
    /// `pos..pos`（`next_token` 在 EOF 后不推进 position）。
    fn next_token_with_span(&mut self) -> Option<(Token, std::ops::Range<usize>)> {
        let start = self.position();
        let token = self.next_token()?;
        let end = self.position();
        Some((token, start..end))
    }
}
