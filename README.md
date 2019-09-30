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
base65536 = "0.4"
```

## Testing
Testing requires that submodules be downloaded. Before testing, run:

```sh
git submodule update --init
```

Benchmarks are available on nightly rust with the `nightly` feature.

[cargo]: https://crates.io/crates/base64
[docs]: https://docs.rs/base65536/
[license]: https://github.com/nuew/base65536/blob/master/LICENSE
[README]: https://github.com/qntm/base65536/blob/master/README.md
[travis]: https://travis-ci.org/nuew/base65536
[qntm]: https://qntm.org/
[qntmbase65536]: https://github.com/qntm/base65536
