# muskitty-css-tokenizer

[English](README.md) | [简体中文](README.zh-CN.md)

[![crates.io](https://img.shields.io/crates/v/muskitty-css-tokenizer.svg)](https://crates.io/crates/muskitty-css-tokenizer)
[![Documentation](https://docs.rs/muskitty-css-tokenizer/badge.svg)](https://docs.rs/muskitty-css-tokenizer)
[![License](https://img.shields.io/crates/l/muskitty-css-tokenizer.svg)](https://github.com/muskitty-dev/muskitty-css-tokenizer/blob/main/LICENSE)
[![CI](https://github.com/muskitty-dev/muskitty-css-tokenizer/actions/workflows/ci.yml/badge.svg)](https://github.com/muskitty-dev/muskitty-css-tokenizer/actions/workflows/ci.yml)

一个从零开始用纯 Rust 编写的 CSS 分词器，实现了
[CSS Syntax Module Level 3 §4.3](https://drafts.csswg.org/css-syntax-3/#tokenization)
规范，且无任何运行时依赖。

它是 [MusKitty](https://github.com/muskitty-dev) 浏览器引擎项目的组成部分。

## 状态

| 组件 | 规范覆盖率 | 测试通过率 |
|-----------|---------------|----------------|
| **Tokenizer** (§4.3) | 15/15 个子算法 | 71/71 个单元测试 |

- 零 `unsafe` 代码
- 零 C/C++ 依赖
- 零运行时依赖
- 仅使用 Rust 稳定版工具链
- MSRV 1.82

## 安装

在你的 `Cargo.toml` 中添加：

```toml
[dependencies]
muskitty-css-tokenizer = "0.1.0"
```

或运行：

```bash
cargo add muskitty-css-tokenizer
```

## 快速上手

```rust
use muskitty_css_tokenizer::{CssTokenizer, Token, Tokenizer};

let mut t = CssTokenizer::new("color: red; ");
while let Some(token) = t.next_token() {
    // 处理 token
}
```

## 架构

```
muskitty-css-tokenizer/
  src/
    types.rs          Token, HashType, Numeric, State 定义
    trait_def.rs      Tokenizer trait
    impls.rs          CssTokenizer — 递归下降分词器（约 1800 行）
    lib.rs            公共 API：CssTokenizer + Tokenizer trait + 类型定义
```

### 什么是 CSS 分词器？

CSS 分词器是一种递归下降算法（§4.3），它消费
Unicode 码点流（经过 §5.3 预处理 —— CR/LF/FF 已归一化为 LF），
并产出 token。主入口
`consume_a_token`（§4.3.1）根据当前输入码点
分派到相应的子算法。

### 规范覆盖率

已实现全部 15 个 §4.3 子算法：

- §4.3.1 Consume a token（完整分派，包含 `unicode_ranges_allowed`）
- §4.3.2 Consume comments
- §4.3.3 Consume a numeric token
- §4.3.4 Consume an ident-like token（包含 `url(` 特例）
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

## 构建

```bash
cargo check
cargo build
```

## 测试

```bash
# 单元测试（71 个测试）
cargo test --lib

# 全部测试
cargo test
```

## 设计原则

1. **CSSWG 即真理** —— 实现严格遵循规范。
2. **对齐规范，而非对齐测试** —— 测试用于验证代码；除非规范证明测试有误，否则绝不为了通过测试而修改代码。
3. **零运行时依赖** —— 纯安全 Rust。
4. **零 unsafe** —— 纯安全 Rust。
5. **精准修改** —— 每次 diff 都只做到任务所需的最小范围。

## 规范参考

本实现参考了以下规范：

- [CSS Syntax Module Level 3](https://drafts.csswg.org/css-syntax-3/) —— 主要权威依据
  - §4.1: Token Railroad Diagrams
  - §4.3: Tokenizer Algorithms
  - §5.3: Input Stream Preprocessing

## 许可证

基于 Apache License, Version 2.0 授权。详见 [LICENSE](LICENSE)。

Copyright 2026 MusCat / MusKitty Bit-Torch Community
