# base65536
[![Build Status](https://travis-ci.org/nuew/base65536.svg?branch=master)](https://travis-ci.org/nuew/base65536)
[![Docs.rs](https://docs.rs/base65536/badge.svg)](https://docs.rs/base65536/)
[![Crates.io](https://img.shields.io/crates/v/base65536.svg)](https://crates.io/crates/base65536)
[![License](https://img.shields.io/github/license/nuew/base65536.svg)](https://github.com/nuew/base65536/blob/master/LICENSE)

An implementation of [base65536][1] for Rust.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
base65536 = "0.3"
```

## Testing
Testing requires that submodules be downloaded. Before testing, run:

```bash
git submodule update --init
```

Benchmarks are available on nightly rust with the `nightly` feature.

[1]: https://github.com/qntm/base65536
[2]: https://crates.io/crates/base64
