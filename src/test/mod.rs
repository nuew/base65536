use super::*;

mod bad;
mod doubled_bytes;
mod ignore_garbage;
mod pairs;
mod single_bytes;
mod wrap;

#[cfg(feature = "nightly")]
mod bench;
