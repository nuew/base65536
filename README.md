# base65536
[![Build Status](https://travis-ci.org/nuew/base65536.svg?branch=master)][travis]
[![Docs.rs](https://docs.rs/base65536/badge.svg)][docs]
[![Crates.io](https://img.shields.io/crates/v/base65536.svg)][cargo]
[![License](https://img.shields.io/github/license/nuew/base65536.svg)][license]

An implementation of [qntm]'s [base65536][qntmbase65536] for Rust.

Base65536 is a binary encoding optimized for UTF-32/UCS-4 encoded text and Twitter.
See the original implementation's [README] for more information.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
base65536 = "1.0"
```

### FNV

By default, base65536 uses the [Fowler–Noll–Vo hash function][fnv], from an
external crate, for a static internal [`HashMap`].
This has no security implications.

You can disable this, and use the standard library's default hash function:

```toml
[dependencies]
base65536 = { version = "1.0", default-features = false }
```

## Testing
Testing requires that submodules be downloaded. Before testing, run:

```sh
git submodule update --init
```

Benchmarks are available on nightly rust with the `nightly` feature.

[`HashMap`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html
[cargo]: https://crates.io/crates/base65536
[docs]: https://docs.rs/base65536/
[fnv]: https://crates.io/crates/fnv
[license]: https://github.com/nuew/base65536/blob/master/LICENSE
[README]: https://github.com/qntm/base65536/blob/master/README.md
[travis]: https://travis-ci.org/nuew/base65536
[qntm]: https://qntm.org/
[qntmbase65536]: https://github.com/qntm/base65536
