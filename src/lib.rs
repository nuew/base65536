//! An implementation of [base65536][1] in Rust.
//!
//! [1]: https://github.com/qntm/base65536
//! [2]: https://crates.io/crates/base64
#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![cfg_attr(feature = "nightly" , feature(test))]

#[macro_use]
extern crate lazy_static;

#[cfg(feature = "fnv")]
extern crate fnv;

#[cfg(feature = "nightly")]
extern crate test as test_crate;

#[cfg(test)]
mod test;

use std::collections::HashMap;
use std::{error, fmt};

#[cfg(feature = "fnv")]
use fnv::FnvBuildHasher as Hasher;

#[cfg(not(feature = "fnv"))]
use std::collections::hash_map::RandomState as Hasher;

const PADDING_BLOCK_START: u32 = 0x1500;
#[cfg_attr(feature = "clippy", allow(unreadable_literal))]
const BLOCK_STARTS: &[u32] =
    &[0x03400, 0x03500, 0x03600, 0x03700, 0x03800, 0x03900, 0x03A00, 0x03B00, 0x03C00, 0x03D00,
      0x03E00, 0x03F00, 0x04000, 0x04100, 0x04200, 0x04300, 0x04400, 0x04500, 0x04600, 0x04700,
      0x04800, 0x04900, 0x04A00, 0x04B00, 0x04C00, 0x04E00, 0x04F00, 0x05000, 0x05100, 0x05200,
      0x05300, 0x05400, 0x05500, 0x05600, 0x05700, 0x05800, 0x05900, 0x05A00, 0x05B00, 0x05C00,
      0x05D00, 0x05E00, 0x05F00, 0x06000, 0x06100, 0x06200, 0x06300, 0x06400, 0x06500, 0x06600,
      0x06700, 0x06800, 0x06900, 0x06A00, 0x06B00, 0x06C00, 0x06D00, 0x06E00, 0x06F00, 0x07000,
      0x07100, 0x07200, 0x07300, 0x07400, 0x07500, 0x07600, 0x07700, 0x07800, 0x07900, 0x07A00,
      0x07B00, 0x07C00, 0x07D00, 0x07E00, 0x07F00, 0x08000, 0x08100, 0x08200, 0x08300, 0x08400,
      0x08500, 0x08600, 0x08700, 0x08800, 0x08900, 0x08A00, 0x08B00, 0x08C00, 0x08D00, 0x08E00,
      0x08F00, 0x09000, 0x09100, 0x09200, 0x09300, 0x09400, 0x09500, 0x09600, 0x09700, 0x09800,
      0x09900, 0x09A00, 0x09B00, 0x09C00, 0x09D00, 0x09E00, 0x0A100, 0x0A200, 0x0A300, 0x0A500,
      0x10600, 0x12000, 0x12100, 0x12200, 0x13000, 0x13100, 0x13200, 0x13300, 0x14400, 0x14500,
      0x16800, 0x16900, 0x20000, 0x20100, 0x20200, 0x20300, 0x20400, 0x20500, 0x20600, 0x20700,
      0x20800, 0x20900, 0x20A00, 0x20B00, 0x20C00, 0x20D00, 0x20E00, 0x20F00, 0x21000, 0x21100,
      0x21200, 0x21300, 0x21400, 0x21500, 0x21600, 0x21700, 0x21800, 0x21900, 0x21A00, 0x21B00,
      0x21C00, 0x21D00, 0x21E00, 0x21F00, 0x22000, 0x22100, 0x22200, 0x22300, 0x22400, 0x22500,
      0x22600, 0x22700, 0x22800, 0x22900, 0x22A00, 0x22B00, 0x22C00, 0x22D00, 0x22E00, 0x22F00,
      0x23000, 0x23100, 0x23200, 0x23300, 0x23400, 0x23500, 0x23600, 0x23700, 0x23800, 0x23900,
      0x23A00, 0x23B00, 0x23C00, 0x23D00, 0x23E00, 0x23F00, 0x24000, 0x24100, 0x24200, 0x24300,
      0x24400, 0x24500, 0x24600, 0x24700, 0x24800, 0x24900, 0x24A00, 0x24B00, 0x24C00, 0x24D00,
      0x24E00, 0x24F00, 0x25000, 0x25100, 0x25200, 0x25300, 0x25400, 0x25500, 0x25600, 0x25700,
      0x25800, 0x25900, 0x25A00, 0x25B00, 0x25C00, 0x25D00, 0x25E00, 0x25F00, 0x26000, 0x26100,
      0x26200, 0x26300, 0x26400, 0x26500, 0x26600, 0x26700, 0x26800, 0x26900, 0x26A00, 0x26B00,
      0x26C00, 0x26D00, 0x26E00, 0x26F00, 0x27000, 0x27100, 0x27200, 0x27300, 0x27400, 0x27500,
      0x27600, 0x27700, 0x27800, 0x27900, 0x27A00, 0x27B00, 0x27C00, 0x27D00, 0x27E00, 0x27F00,
      0x28000, 0x28100, 0x28200, 0x28300, 0x28400, 0x28500];
lazy_static! {
    static ref BLOCK_START_TO_INDEX: HashMap<u32, u8, Hasher> =
        (0..BLOCK_STARTS.len()).map(|b| (BLOCK_STARTS[b], b as u8)).collect();
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
/// Represents an error in the decoding process.
pub enum Error {
    /// A code point not valid in base65536 was found in the input stream.
    /// Consider using the `ignore_garbage` option.
    ///
    /// The usize is the offset from the begining of the input stream where the invalid code point
    /// was found.
    ///
    /// The char is the invalid code point.
    InvalidCodePoint(usize, char),
    /// The base65536 stream continued after a terminating padding byte.
    InvalidLength,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::InvalidCodePoint(offset, ch) => {
                write!(f, "invalid code point '{}' at offset {}", ch, offset)
            }
            Error::InvalidLength => write!(f, "sequence continued after final byte"),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::InvalidCodePoint(_, _) => "invalid code point",
            Error::InvalidLength => "invalid length",
        }
    }
}

pub type DecodeResult<T> = ::std::result::Result<T, Error>;

/// Decode from string reference as octets.
///
/// Note that `decode` and [`decode_buf`] are *very* strict by default, even
/// failing on line breaks, as to match behaviour with the original
/// implementation. To prevent this, use with the `ignore_garbage`
/// option.
///
/// [`decode_buf`]: fn.decode_buf.html
pub fn decode<T: ?Sized + AsRef<str>>(input: &T, ignore_garbage: bool) -> DecodeResult<Vec<u8>> {
    // The default capacity was determined by benchmarking instead of math because I got tired.
    // This should be (and this might actually be) 2x the number of characters.
    let mut buf = Vec::with_capacity(input.as_ref().len());
    decode_buf(input, &mut buf, ignore_garbage).map(|_| buf)
}

/// Decode from string reference as octets.
/// Writes into the supplied buffer as to avoid unnecessary allocation.
///
/// Note that [`decode`] and `decode_buf` are *very* strict by default, even
/// failing on line breaks, as to match behaviour with the original
/// implementation. To prevent this, use with the `ignore_garbage`
/// option.
///
/// [`decode`]: fn.decode.html
pub fn decode_buf<T: ?Sized + AsRef<str>>(input: &T,
                                          buf: &mut Vec<u8>,
                                          ignore_garbage: bool)
                                          -> DecodeResult<()> {
    let mut done = false;
    for (index, code_point) in input.as_ref().char_indices() {
        let (byte1, block_start) = {
            const BLOCK_MASK: u32 = (1 << 8) - 1;
            let code_point = code_point as u32;

            let byte1 = code_point & BLOCK_MASK;
            (byte1 as u8, code_point - byte1)
        };

        if block_start == PADDING_BLOCK_START {
            if done {
                return Err(Error::InvalidLength);
            } else {
                buf.push(byte1);
                done = true;
            }
        } else if let Some(byte2) = BLOCK_START_TO_INDEX.get(&block_start) {
            if done {
                return Err(Error::InvalidLength);
            } else {
                buf.push(byte1);
                buf.push(*byte2);
            }
        } else if !ignore_garbage {
            return Err(Error::InvalidCodePoint(index, code_point));
        }
    }

    Ok(())
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
/// Line Wrapping Options
pub enum WrapOptions<'a> {
    /// Don't wrap lines at all
    NoWrap,
    /// Wrap every so many columns with '\n'. The length must be > 0.
    WrapAt(usize),
    /// Wrap every so many columns with a specified string. The length must be > 0.
    WrapAtWith(usize, &'a str),
}

impl<'a, T> From<T> for WrapOptions<'a>
    where T: Into<Option<usize>>
{
    fn from(from: T) -> Self {
        match from.into() {
            Some(columns) => WrapOptions::WrapAt(columns),
            None => WrapOptions::NoWrap
        }
    }
}

/// Encode arbitrary octets as base65536.
///
/// The `wrap` option allows wrapping the output every so many characters with
/// a supplied string. You should generally use "\n", though you may want to
/// use "\r\n" on Windows.
///
/// Unless called with `ignore_garbage` on, [`decode`] and [`decode_buf`] will
/// fail on output generated with a wrap. This is to match behaivour with the
/// original implementation.
///
/// # Panics
/// Panics if the wrap is set to every 0 columns.
///
/// [`decode`]: fn.decode.html
/// [`decode_buf`]: fn.decode_buf.html
pub fn encode<'a, T, W>(input: &T, wrap: W) -> String
    where T: ?Sized + AsRef<[u8]>, W: Into<WrapOptions<'a>>
{
    // Some output code points are three bytes, while others are four. As every two bytes of input
    // results in one character of output, this allocates the necessary space for the maximum
    // possible output length, assuming that `wrap` is set to None.
    // A reallocation may be necessary if the `wrap` option is used.
    let mut output = String::with_capacity(input.as_ref().len() * 2);
    encode_buf(input, &mut output, wrap);
    output
}

/// Encode arbitrary octets as base65536.
/// Writes into the supplied buffer as to avoid unnecessary allocation.
///
/// The `wrap` option allows wrapping the output every so many characters with
/// a supplied string. You should generally use "\n", though you may want to
/// use "\r\n" on Windows.
///
/// Unless called with `ignore_garbage` on, [`decode`] and [`decode_buf`] will
/// fail on output generated with a wrap. This is to match behaivour with the
/// original implementation.
///
/// # Panics
/// Panics if the wrap is set to every 0 columns.
///
/// [`decode`]: fn.decode.html
/// [`decode_buf`]: fn.decode_buf.html
pub fn encode_buf<'a, T, W>(input: &T, buf: &mut String, wrap: W)
    where T: ?Sized + AsRef<[u8]>, W: Into<WrapOptions<'a>>
{
    use std::char::from_u32_unchecked;

    let wrap = wrap.into();
    for (count, bytes) in input.as_ref().chunks(2).enumerate() {
        match wrap {
            WrapOptions::NoWrap => {},
            WrapOptions::WrapAt(columns) => {
                if count % columns == 0 && count != 0 {
                    buf.push('\n');
                }
            },
            WrapOptions::WrapAtWith(columns, eol) => {
                if count % columns == 0 && count != 0 {
                    buf.push_str(eol);
                }
            }
        }

        let block_start = match bytes.len() {
            1 => PADDING_BLOCK_START,
            2 => BLOCK_STARTS[bytes[1] as usize],
            _ => unreachable!(),
        };

        // It is safe to use this because we know that all code points within
        // 0x100 of any possible block_start are defined, and that's the
        // largest possible addition to block_start.
        let code_point = block_start + u32::from(bytes[0]);
        buf.push(unsafe { from_u32_unchecked(code_point) });
    }
}
