//! Span 追踪测试 — 验证 `next_token_with_span` 返回的 char range 精确覆盖每个 token。

use muskitty_css_tokenizer::{CssTokenizer, Token, Tokenizer};

#[test]
fn span_covers_exact_token_chars() {
    let mut tz = CssTokenizer::new("10px");
    let (token, range) = tz.next_token_with_span().unwrap();
    assert!(matches!(token, Token::Dimension(_, _)));
    assert_eq!(range, 0..4, "span should cover \"10px\" (4 chars)");
}

#[test]
fn span_tracks_position_across_tokens() {
    let mut tz = CssTokenizer::new("a b");
    let (_, r1) = tz.next_token_with_span().unwrap(); // "a"
    let (_, r2) = tz.next_token_with_span().unwrap(); // " "
    let (_, r3) = tz.next_token_with_span().unwrap(); // "b"
    assert_eq!(r1, 0..1, "ident \"a\"");
    assert_eq!(r2, 1..2, "whitespace");
    assert_eq!(r3, 2..3, "ident \"b\"");
}

#[test]
fn eof_span_is_empty_at_end() {
    let mut tz = CssTokenizer::new("a");
    tz.next_token_with_span(); // "a"
    let (token, range) = tz.next_token_with_span().unwrap(); // EOF
    assert!(matches!(token, Token::Eof));
    assert_eq!(range, 1..1, "EOF span is empty");
}

#[test]
fn span_after_eof_returns_none() {
    let mut tz = CssTokenizer::new("a");
    tz.next_token_with_span(); // "a"
    tz.next_token_with_span(); // EOF
    assert!(tz.next_token_with_span().is_none(), "after EOF → None");
}

#[test]
fn span_covers_multichar_token() {
    // "color" → single Ident token, span 0..5
    let mut tz = CssTokenizer::new("color");
    let (token, range) = tz.next_token_with_span().unwrap();
    assert!(matches!(token, Token::Ident(_)));
    assert_eq!(range, 0..5);
}

#[test]
fn span_skips_comments() {
    // "/* c */ x" → comment skipped inside consume_a_token's loop, then
    // Whitespace consumed. The span of the Whitespace token starts from
    // pos 0 (before the comment) because next_token_with_span records
    // start before next_token() runs, and comment-skipping happens inside
    // next_token(). This is acceptable for original_text: the source slice
    // faithfully includes the comment.
    let mut tz = CssTokenizer::new("/* c */ x");
    let (ws, r_ws) = tz.next_token_with_span().unwrap();
    assert!(matches!(ws, Token::Whitespace));
    // span 0..8: covers the comment (0..7) + the whitespace char (7)
    assert_eq!(r_ws, 0..8, "span includes skipped comment + whitespace");
    let (token, range) = tz.next_token_with_span().unwrap();
    assert!(matches!(token, Token::Ident(_)));
    assert_eq!(range, 8..9, "ident \"x\"");
}

#[test]
fn span_tracks_string_token() {
    // "\"hi\"" → String("hi"), span covers all 4 chars
    let mut tz = CssTokenizer::new("\"hi\"");
    let (token, range) = tz.next_token_with_span().unwrap();
    assert!(matches!(token, Token::String(_)));
    assert_eq!(range, 0..4);
}

#[test]
fn span_tracks_function_token() {
    // "foo(" → Function("foo"), span 0..4
    let mut tz = CssTokenizer::new("foo(");
    let (token, range) = tz.next_token_with_span().unwrap();
    assert!(matches!(token, Token::Function(_)));
    assert_eq!(range, 0..4);
}

#[test]
fn span_tracks_dimension_with_decimal() {
    // "1.5em" → Dimension, span 0..5
    let mut tz = CssTokenizer::new("1.5em");
    let (token, range) = tz.next_token_with_span().unwrap();
    assert!(matches!(token, Token::Dimension(_, _)));
    assert_eq!(range, 0..5);
}

#[test]
fn span_tracks_percentage() {
    // "50%" → Percentage, span 0..3
    let mut tz = CssTokenizer::new("50%");
    let (token, range) = tz.next_token_with_span().unwrap();
    assert!(matches!(token, Token::Percentage(_)));
    assert_eq!(range, 0..3);
}

#[test]
fn span_handles_non_ascii() {
    // "café" → Ident("café"), é is one char but 2 bytes
    let mut tz = CssTokenizer::new("café");
    let (token, range) = tz.next_token_with_span().unwrap();
    assert!(matches!(token, Token::Ident(_)));
    assert_eq!(range, 0..4, "4 chars (not bytes)");
}
