//! WPT `css/css-syntax` tokenizer-level test suite harness.
//!
//! Drives the MusKitty CSS tokenizer through fixtures extracted from
//! upstream WPT `css/css-syntax/*.html` (`tests/data/wpt/*.json`). The
//! upstream tests assert through the CSSOM (`getPropertyValue`,
//! `selectorText` round-trips); the portable essence is tokenized
//! token identity, which is what is asserted here. The JSON `note`
//! fields document the exact mapping per file.
//!
//! Not ported from css/css-syntax (upstream asserts things this crate
//! cannot express yet):
//! - serialize-consecutive-tokens / serialize-escape-identifiers /
//!   anb-serialization: no token or selector serializer implemented.
//! - urange-parsing / unicode-range-selector: property-grammar level.
//! - surrogate halves of input-preprocessing: lone surrogates are
//!   unrepresentable in Rust `str`.
//!
//! Run with:
//!   cargo test --test wpt_css_syntax -- --nocapture

use muskitty_css_tokenizer::{CssTokenizer, Token, Tokenizer};
use serde_json::Value;
use std::fs;
use std::path::PathBuf;

struct Case {
    file: String,
    desc: String,
    run: Box<dyn Fn() -> Result<(), String>>,
}

fn tokens(input: &str) -> Vec<Token> {
    let mut t = CssTokenizer::new(input);
    let mut out = Vec::new();
    while let Some(tok) = t.next_token() {
        if matches!(tok, Token::Eof) {
            break;
        }
        out.push(tok);
    }
    out
}

fn build_cases(file: String, root: &Value) -> Vec<Case> {
    let source = root
        .get("source")
        .and_then(|v| v.as_str())
        .unwrap_or("?")
        .to_string();
    let cases = root
        .get("cases")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    let mut out = Vec::new();
    for c in &cases {
        let kind = c
            .get("kind")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let desc = format!(
            "{source} {kind} {:?}",
            c.get("input").and_then(|v| v.as_str()).unwrap_or("")
        );
        let kind_clone = kind.clone();
        let c = c.clone();
        out.push(Case {
            file: file.clone(),
            desc,
            run: Box::new(move || {
                match kind_clone.as_str() {
                    "ident" => {
                        let input = c.get("input").and_then(|v| v.as_str()).unwrap_or("");
                        let want = c.get("value").and_then(|v| v.as_str()).unwrap_or("");
                        match tokens(input).first() {
                            Some(Token::Ident(v)) if v == want => Ok(()),
                            other => Err(format!("expected Ident({want:?}), got {other:?}")),
                        }
                    }
                    "dimension" => {
                        let input = c.get("input").and_then(|v| v.as_str()).unwrap_or("");
                        let unit = c.get("unit").and_then(|v| v.as_str()).unwrap_or("");
                        let want_value: f64 = c
                            .get("value")
                            .and_then(|v| v.as_str())
                            .and_then(|s| s.parse().ok())
                            .unwrap_or(f64::NAN);
                        match tokens(input).first() {
                            Some(Token::Dimension(n, u))
                                if (n.value - want_value).abs() < f64::EPSILON && u == unit =>
                            {
                                Ok(())
                            }
                            other => Err(format!(
                                "expected Dimension({want_value}, {unit:?}), got {other:?}"
                            )),
                        }
                    }
                    "url" | "string" => {
                        let input = c.get("input").and_then(|v| v.as_str()).unwrap_or("");
                        let want = c.get("value").and_then(|v| v.as_str()).unwrap_or("");
                        let expected = if kind_clone == "url" {
                            Token::Url(want.to_string())
                        } else {
                            Token::String(want.to_string())
                        };
                        match tokens(input).first() {
                            Some(t) if *t == expected => Ok(()),
                            other => Err(format!("expected {expected:?}, got {other:?}")),
                        }
                    }
                    "cdc" => match tokens("-->").first() {
                        Some(Token::Cdc) => Ok(()),
                        other => Err(format!("expected Cdc, got {other:?}")),
                    },
                    "is-whitespace" | "not-whitespace" => {
                        let cp_val =
                            c.get("codepoint").and_then(|v| v.as_u64()).unwrap_or(0);
                        let cp = char::from_u32(cp_val as u32).unwrap_or('\u{FFFD}');
                        let toks = tokens(&format!(".a{cp}b"));
                        let ws_count = toks
                            .iter()
                            .filter(|t| matches!(t, Token::Whitespace))
                            .count();
                        if kind_clone == "is-whitespace" {
                            if ws_count == 1 {
                                Ok(())
                            } else {
                                Err(format!(
                                    "expected exactly 1 Whitespace token, got {ws_count} in {toks:?}"
                                ))
                            }
                        } else if ws_count == 0 {
                            Ok(())
                        } else {
                            Err(format!("expected no Whitespace token, got {toks:?}"))
                        }
                    }
                    "valid-ident-char" | "not-ident-char" => {
                        let cp_val =
                            c.get("codepoint").and_then(|v| v.as_u64()).unwrap_or(0);
                        let cp = char::from_u32(cp_val as u32).unwrap_or('\u{FFFD}');
                        match tokens(&format!("f{cp}oo")).first() {
                            Some(Token::Ident(v)) => {
                                if kind_clone == "valid-ident-char" {
                                    if *v == format!("f{cp}oo") {
                                        Ok(())
                                    } else {
                                        Err(format!(
                                            "ident split at U+{cp_val:04X}: {v:?}"
                                        ))
                                    }
                                } else if *v == "f" {
                                    // The ident stops before the non-ident
                                    // code point — correct.
                                    Ok(())
                                } else {
                                    Err(format!(
                                        "U+{cp_val:04X} unexpectedly consumed into ident"
                                    ))
                                }
                            }
                            other => Err(format!(
                                "expected Ident first token, got {other:?} (U+{cp_val:04X})"
                            )),
                        }
                    }
                    other => Err(format!("unknown case kind {other:?}")),
                }
            }),
        });
    }
    out
}

#[test]
fn wpt_css_syntax_tokenizer_suite() {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join("wpt");
    let mut entries: Vec<PathBuf> = fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("read wpt dir {dir:?}: {e}"))
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|s| s.to_str()) == Some("json"))
        .collect();
    entries.sort();

    let mut total_pass = 0usize;
    let mut total_fail = 0usize;
    let mut per_file: Vec<(String, usize, usize)> = Vec::new();
    let mut failures: Vec<(String, String, String)> = Vec::new();

    for path in &entries {
        let name = path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("?")
            .to_string();
        let text = fs::read_to_string(path).unwrap_or_else(|e| panic!("read {path:?}: {e}"));
        let root: Value =
            serde_json::from_str(&text).unwrap_or_else(|e| panic!("parse {path:?}: {e}"));
        let cases = build_cases(name.clone(), &root);
        let mut file_pass = 0usize;
        let mut file_fail = 0usize;
        for case in &cases {
            match (case.run)() {
                Ok(()) => file_pass += 1,
                Err(detail) => {
                    file_fail += 1;
                    failures.push((case.file.clone(), case.desc.clone(), detail));
                }
            }
        }
        total_pass += file_pass;
        total_fail += file_fail;
        per_file.push((name, file_pass, file_fail));
    }

    eprintln!("\n═══════════════════════════════════════════════════════════════");
    eprintln!(" WPT css/css-syntax (tokenizer level) — results");
    eprintln!("═══════════════════════════════════════════════════════════════");
    eprintln!(
        " {:<36} {:>8} {:>8} {:>8}",
        "fixture", "pass", "fail", "total"
    );
    eprintln!(" ─────────────────────────────────────────────────────────────────");
    for (name, p, f) in &per_file {
        eprintln!(" {:<36} {:>8} {:>8} {:>8}", name, p, f, p + f);
    }
    eprintln!(" ─────────────────────────────────────────────────────────────────");
    let total = total_pass + total_fail;
    let pct = if total == 0 {
        0.0
    } else {
        100.0 * total_pass as f64 / total as f64
    };
    eprintln!(
        " {:<36} {:>8} {:>8} {:>8}   ({:.1}%)",
        "TOTAL", total_pass, total_fail, total, pct
    );
    eprintln!("\n── failures ──");
    for (file, desc, detail) in &failures {
        eprintln!("\n[{file}] {desc}\n  {detail}");
    }
    eprintln!("═══════════════════════════════════════════════════════════════\n");

    assert!(
        total > 0,
        "no test cases were loaded — fixture data missing?"
    );
    eprintln!(
        "PASS RATE: {:.1}% ({}/{}) — informational; not asserting a hard threshold yet.",
        pct, total_pass, total
    );
}
