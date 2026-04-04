# broomfilter

[![CI](https://github.com/ae2rs/broomfilter/actions/workflows/ci.yml/badge.svg)](https://github.com/ae2rs/broomfilter/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/broomfilter.svg)](https://crates.io/crates/broomfilter)
[![docs.rs](https://docs.rs/broomfilter/badge.svg)](https://docs.rs/broomfilter)
[![Rust](https://img.shields.io/badge/rust-1.85%2B-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Downloads](https://img.shields.io/crates/d/broomfilter.svg)](https://crates.io/crates/broomfilter)

A (pretty fast) bloom filter implementation in Rust.

---

## 🤔 What is this?

A bloom filter is a probabilistic data structure that can tell you:
- ❌ **"Definitely not in the set"** — 100% accurate
- 🤷 **"Probably in the set"** — maybe, who knows

Think of it as that friend who says "I'm *pretty sure* I locked the door."
You can trust them when they say they didn't, but when they say they did... you might want to go check.

### 🧹 Theory of broom sweeping (simplified)

```
    item ──→ [ hash hash hash ] ──→ flip bits in the array
                  🧹💨

    query ──→ [ hash hash hash ] ──→ check bits
                                      ├── all set?   → "probably yes" 🤷
                                      └── any unset? → "definitely no" ❌
```

---

## 💡 Why does this exist?

Because I wanted to learn how a bloom filter actually worked instead of just reciting the definition in tech interviews.
Turns out its super fun to optimize!

---

## 🚀 Usage

```rust
use broomfilter::Filter;

// Create a filter with 2^14 bits, optimized for 10,000 items
let mut filter = Filter::new(14, 10_000).unwrap();

// Insert some things 🧹
filter.insert("hello");
filter.insert("world");

// Check for things
assert!(filter.contains("hello"));    // true  — probably  🤷
assert!(!filter.contains("goodbye")); // false — definitely ❌
```

---

## 🏗️ Status

> ⚠️ **Work in progress** — handle with care (or don't, I'm a dev, not a cop)

It works, it has benchmarks, it *probably* won't break. But then again,
the whole point of a bloom filter is that you can never be 100% sure about anything.

| Feature | Status |
|---------|--------|
| Insert items | ✅ Swept in |
| Check membership | ✅ Probably |
| False positives | ✅ It's a feature |
| False negatives | 🚫 Impossible (finally, some certainty) |
| Delete items | 🧹 Nope, the broom only sweeps one way |

---

## 📜 License

MIT — do whatever you want with it, just don't blame me when it breaks.
