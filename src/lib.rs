//! A binary encoding optimized for UTF-32/UCS-4 encoded text and Twitter.
//!
//! This is a Rust reimplementation of [qntm]'s original [base65536].
//!
//! # Examples
//!
//! Decoding:
//!
//! ```rust
//! # fn test() -> Result<(), Box<std::error::Error>> {
//! use base65536::decode;
//!
//! // don't ignore garbage - note that this means that word wrapping doesn't work
//! assert_eq!(vec![1, 2, 3], decode("㘁ᔃ", false)?);
//! assert_eq!("hello world", String::from_utf8(decode("驨ꍬ啯𒁷ꍲᕤ", false)?)?);
//!
//! // ignore garbage
//! assert_eq!(vec![1, 2, 3], decode("㘁asdfghjklᔃ", true)?);
//! assert_eq!("hello world", String::from_utf8(decode("驨ꍬ啯𒁷ꍲᕤ\n", true)?)?);
//! # Ok(()) }
//! # test().unwrap();
//! ```
//!
//! Encoding:
//!
//! ```rust
//! use base65536::{WrapOptions, encode};
//!
//! // no word wrapping
//! assert_eq!("㘁ᔃ", encode(&[1, 2, 3], None));
//! assert_eq!("驨ꍬ啯𒁷ꍲᕤ", encode("hello world", None));
//!
//! // word wrapping
//! assert_eq!("㘁\nᔃ", encode(&[1, 2, 3], 1));
//! assert_eq!("驨ꍬ啯\n𒁷ꍲᕤ", encode("hello world", 3));
//!
//! // word wrapping with a custom line ending
//! assert_eq!("㘁\r\nᔃ", encode(&[1, 2, 3], WrapOptions::WrapAtWith(1, "\r\n")));
//! assert_eq!("驨ꍬ啯\r\n𒁷ꍲᕤ", encode("hello world", WrapOptions::WrapAtWith(3, "\r\n")));
//! ```
//!
//! [qntm]: https://qntm.org/
//! [base65536]: https://github.com/qntm/base65536
#![doc(html_root_url = "https://docs.rs/base65536/0.4.0")]
#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![cfg_attr(feature = "nightly", feature(test))]

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
const BLOCK_STARTS: &[u32] = &[
    0x03400, 0x03500, 0x03600, 0x03700, 0x03800, 0x03900, 0x03A00, 0x03B00, 0x03C00, 0x03D00,
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
    0x28000, 0x28100, 0x28200, 0x28300, 0x28400, 0x28500,
];
lazy_static! {
    static ref BLOCK_START_TO_INDEX: HashMap<u32, u8, Hasher> =
        (0..BLOCK_STARTS.len()).map(|b| (BLOCK_STARTS[b], b as u8)).collect();
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
/// Represents an error while decoding.
///
/// Used with [`decode`] and [`decode_buf`]. See them for examples.
///
/// [`decode`]: fn.decode.html
/// [`decode_buf`]: fn.decode_buf.html
pub enum Error {
    /// A code point not valid in base65536 was found in the input stream.
    /// Consider using the `ignore_garbage` option.
    ///
    /// Contains the offset from the beginning of the stream at which the
    /// invalid code point was found, and the actual code point.
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

/// A specialized [`Result`] type for decoding operations.
///
/// [`Result`]: https://doc.rust-lang.org/std/result/enum.Result.html
pub type DecodeResult<T> = ::std::result::Result<T, Error>;

#[inline]
fn inner_decode<F>(input: &str, ignore_garbage: bool, mut out: F) -> DecodeResult<()>
where
    F: FnMut(u8, Option<u8>),
{
    let mut done = false;
    for (index, code_point) in input.char_indices() {
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
                out(byte1, None);
                done = true;
            }
        } else if let Some(byte2) = BLOCK_START_TO_INDEX.get(&block_start) {
            if done {
                return Err(Error::InvalidLength);
            } else {
                out(byte1, Some(*byte2));
            }
        } else if !ignore_garbage {
            return Err(Error::InvalidCodePoint(index, code_point));
        }
    }

    Ok(())
}

/// Decode from a reference to a base65536-encoded string as octets.
///
/// # Errors
///
/// If the input string contains a character not inside of a base65536 block,
/// [`Error::InvalidCodePoint`] will be retuned, along with the bad character,
/// and it's position in the input.
///
/// Note that [`decode`], [`decode_buf`], and [`decode_slice`] are *very*
/// strict by default, even failing on line breaks (such as those generated by
/// [`encode`], [`encode_buf`], and [`encode_slice`] when wrapping is enabled),
/// as to match behaviour with the [original implementation]. To prevent this,
/// use with the `ignore_garbage` option.
///
/// If the base65536 stream continues after a terminating padding character,
/// [`Error::InvalidLength`] is returned.
///
/// # Examples
///
/// ```rust
/// # fn test() -> Result<(), Box<std::error::Error>> {
/// use base65536::decode;
///
/// assert_eq!(vec![1, 2, 3], decode("㘁ᔃ", false)?);
/// assert_eq!("hello world", String::from_utf8(decode("驨ꍬ啯𒁷ꍲᕤ", false)?)?);
/// # Ok(()) }
/// # test().unwrap();
/// ```
///
/// [`decode`]: fn.decode.html
/// [`decode_buf`]: fn.decode_buf.html
/// [`decode_slice`]: fn.decode_slice.html
/// [`encode`]: fn.encode.html
/// [`encode_buf`]: fn.encode_buf.html
/// [`encode_slice`]: fn.encode_slice.html
/// [original implementation]: https://github.com/qntm/base65536
/// [`Error::InvalidCodePoint`]: enum.Error.html#variant.InvalidCodePoint
/// [`Error::InvalidLength`]: enum.Error.html#variant.InvalidLength
pub fn decode<T>(input: &T, ignore_garbage: bool) -> DecodeResult<Vec<u8>>
where
    T: ?Sized + AsRef<str>,
{
    let mut buf = Vec::with_capacity(input.as_ref().len());
    decode_buf(input, &mut buf, ignore_garbage).map(|_| buf)
}

/// Decode from a reference to a base65536-encoded string as octets.
/// Writes into the supplied output buffer, growing it if needed.
///
/// # Errors
///
/// If the input string contains a character not inside of a base65536 block,
/// [`Error::InvalidCodePoint`] will be retuned, along with the bad character,
/// and it's position in the input.
///
/// Note that [`decode`], [`decode_buf`], and [`decode_slice`] are *very*
/// strict by default, even failing on line breaks (such as those generated by
/// [`encode`], [`encode_buf`], and [`encode_slice`] when wrapping is enabled),
/// as to match behaviour with the [original implementation]. To prevent this,
/// use with the `ignore_garbage` option.
///
/// If the base65536 stream continues after a terminating padding character,
/// [`Error::InvalidLength`] is returned.
///
/// # Examples
///
/// ```rust
/// # fn test() -> Result<(), Box<std::error::Error>> {
/// use base65536::decode_buf;
///
/// let mut buf = Vec::new();
/// decode_buf("㘁ᔃ", &mut buf, false)?;
/// assert_eq!(vec![1, 2, 3], buf);
///
/// let mut buf = Vec::new();
/// decode_buf("驨ꍬ啯𒁷ꍲᕤ", &mut buf, false)?;
/// assert_eq!("hello world", String::from_utf8(buf)?);
/// # Ok(()) }
/// # test().unwrap();
/// ```
///
/// [`decode`]: fn.decode.html
/// [`decode_buf`]: fn.decode_buf.html
/// [`decode_slice`]: fn.decode_slice.html
/// [`encode`]: fn.encode.html
/// [`encode_buf`]: fn.encode_buf.html
/// [`encode_slice`]: fn.encode_slice.html
/// [original implementation]: https://github.com/qntm/base65536
/// [`Error::InvalidCodePoint`]: enum.Error.html#variant.InvalidCodePoint
/// [`Error::InvalidLength`]: enum.Error.html#variant.InvalidLength
pub fn decode_buf<T>(input: &T, buf: &mut Vec<u8>, ignore_garbage: bool) -> DecodeResult<()>
where
    T: ?Sized + AsRef<str>,
{
    inner_decode(input.as_ref(), ignore_garbage, |a, b| {
        buf.push(a);
        if let Some(b) = b {
            buf.push(b)
        }
    })
}

/// Decode from a reference to a base65536-encoded string as octets.
/// Writes into the supplied slice, returning how many bytes were written.
///
/// # Panics
///
/// Panics if the slice is not long enough.
///
/// # Errors
///
/// If the input string contains a character not inside of a base65536 block,
/// [`Error::InvalidCodePoint`] will be retuned, along with the bad character,
/// and it's position in the input.
///
/// Note that [`decode`], [`decode_buf`], and [`decode_slice`] are *very*
/// strict by default, even failing on line breaks (such as those generated by
/// [`encode`], [`encode_buf`], and [`encode_slice`] when wrapping is enabled),
/// as to match behaviour with the [original implementation]. To prevent this,
/// use with the `ignore_garbage` option.
///
/// If the base65536 stream continues after a terminating padding character,
/// [`Error::InvalidLength`] is returned.
///
/// # Examples
///
/// ```rust
/// # fn test() -> Result<(), Box<std::error::Error>> {
/// use base65536::decode_slice;
///
/// let mut buf = [0; 3];
/// decode_slice("㘁ᔃ", &mut buf, false)?;
/// assert_eq!([1, 2, 3], buf);
///
/// let mut buf = [0; 11];
/// decode_slice("驨ꍬ啯𒁷ꍲᕤ", &mut buf, false)?;
/// assert_eq!(b"hello world", &buf);
/// # Ok(()) }
/// # test().unwrap();
/// ```
///
/// [`decode`]: fn.decode.html
/// [`decode_buf`]: fn.decode_buf.html
/// [`decode_slice`]: fn.decode_slice.html
/// [`encode`]: fn.encode.html
/// [`encode_buf`]: fn.encode_buf.html
/// [`encode_slice`]: fn.encode_slice.html
/// [original implementation]: https://github.com/qntm/base65536
/// [`Error::InvalidCodePoint`]: enum.Error.html#variant.InvalidCodePoint
/// [`Error::InvalidLength`]: enum.Error.html#variant.InvalidLength
pub fn decode_slice<T>(input: &T, buf: &mut [u8], ignore_garbage: bool) -> DecodeResult<usize>
where
    T: ?Sized + AsRef<str>,
{
    let mut pos = 0;
    inner_decode(input.as_ref(), ignore_garbage, |a, b| {
        buf[pos] = a;
        pos += 1;
        if let Some(b) = b {
            buf[pos] = b;
            pos += 1;
        }
    }).map(|_| pos)
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
/// Line Wrapping Options.
///
/// Used with [`encode`] and [`encode_buf`]. See them for examples.
///
/// Unless you want to specify a custom end-of-line string, use an
/// [`Option::None`] instead for no wrapping or an [`usize`] instead for
/// wrapping at a column boundary, and everything will magically work.
///
/// [`encode`]: fn.encode.html
/// [`encode_buf`]: fn.encode_buf.html
/// [`Option::None`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.None
/// [`usize`]: https://doc.rust-lang.org/std/primitive.usize.html
pub enum WrapOptions<'a> {
    /// Don't wrap lines at all
    NoWrap,
    /// Wrap every so many columns with '\n'. The length must be > 0.
    WrapAt(usize),
    /// Wrap every so many columns with a specified string. The length must be > 0.
    WrapAtWith(usize, &'a str),
}

impl<'a, T> From<T> for WrapOptions<'a>
where
    T: Into<Option<usize>>,
{
    fn from(from: T) -> Self {
        match from.into() {
            Some(columns) => WrapOptions::WrapAt(columns),
            None => WrapOptions::NoWrap,
        }
    }
}

/// Encode arbitrary octets as base65536.
///
/// The `wrap` option allows wrapping the output every so many characters,
/// optionally with a supplied string using [`WrapOptions`].
///
/// Unless called with `ignore_garbage`, [`decode`] and [`decode_buf`] will
/// fail on output generated with a wrap. This is to match behaviour with the
/// original implementation.
///
/// # Panics
///
/// Panics if set to wrap every 0 columns.
///
/// # Examples
///
/// Without word wrapping:
///
/// ```rust
/// use base65536::encode;
///
/// assert_eq!("驨ꍬ啯𒁷ꍲᕤ", encode("hello world", None));
/// ```
///
/// With word wrapping:
///
/// ```rust
/// # fn test() -> Option<()> {
/// use base65536::encode;
///
/// assert_eq!(base65536::encode(
///     "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed \
///      do eiusmod tempor incididunt ut labore et dolore magna \
///      aliqua. Ut enim ad minim veniam, quis nostrud exercitation \
///      ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis \
///      aute irure dolor in reprehenderit in voluptate velit esse \
///      cillum dolore eu fugiat nulla pariatur. Excepteur sint \
///      occaecat cupidatat non proident, sunt in culpa qui officia \
///      deserunt mollit anim id est laborum.", 80)
///     .lines().next()?.chars().count(), 80);
/// # Some(()) }
/// # test().unwrap()
/// ```
///
/// With word wrapping using a custom line ending:
///
/// ```rust
/// # fn test() -> Option<()> {
/// use base65536::{WrapOptions, encode};
///
/// assert_eq!(base65536::encode(
///     "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed \
///      do eiusmod tempor incididunt ut labore et dolore magna \
///      aliqua. Ut enim ad minim veniam, quis nostrud exercitation \
///      ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis \
///      aute irure dolor in reprehenderit in voluptate velit esse \
///      cillum dolore eu fugiat nulla pariatur. Excepteur sint \
///      occaecat cupidatat non proident, sunt in culpa qui officia \
///      deserunt mollit anim id est laborum.",
///     WrapOptions::WrapAtWith(80, "\r\n"))
///     .lines().next()?.chars().count(), 80);
/// # Some(()) }
/// # test().unwrap()
/// ```
///
/// [`WrapOptions`]: enum.WrapOptions.html
/// [`decode`]: fn.decode.html
/// [`decode_buf`]: fn.decode_buf.html
pub fn encode<'a, T: ?Sized, W>(input: &T, wrap: W) -> String
where
    T: AsRef<[u8]>,
    W: Into<WrapOptions<'a>>,
{
    // Some output code points are three bytes, while others are four. As every two bytes of input
    // results in one character of output, this allocates the necessary space for the maximum
    // possible output length, assuming that `wrap` is set to None.
    // A reallocation may be necessary if the `wrap` option is used.
    let mut output = String::with_capacity(input.as_ref().len() * 2);
    encode_buf(input, &mut output, wrap);
    output
}

/// Encode arbitrary octets as base65536. Writes into the supplied output
/// buffer, growing it if needed.
///
/// The `wrap` option allows wrapping the output every so many characters,
/// optionally with a supplied string using [`WrapOptions`].
///
/// Unless called with `ignore_garbage`, [`decode`] and [`decode_buf`] will
/// fail on output generated with a wrap. This is to match behaviour with the
/// original implementation.
///
/// # Panics
///
/// Panics if set to wrap every 0 columns.
///
/// # Examples
///
/// Without word wrapping:
///
/// ```rust
/// use base65536::encode_buf;
///
/// let mut buf = String::new();
/// encode_buf("hello world", &mut buf, None);
///
/// assert_eq!("驨ꍬ啯𒁷ꍲᕤ", buf);
/// ```
///
/// With word wrapping:
///
/// ```rust
/// # fn test() -> Option<()> {
/// use base65536::encode_buf;
///
/// let mut buf = String::new();
/// base65536::encode_buf(
///     "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed \
///      do eiusmod tempor incididunt ut labore et dolore magna \
///      aliqua. Ut enim ad minim veniam, quis nostrud exercitation \
///      ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis \
///      aute irure dolor in reprehenderit in voluptate velit esse \
///      cillum dolore eu fugiat nulla pariatur. Excepteur sint \
///      occaecat cupidatat non proident, sunt in culpa qui officia \
///      deserunt mollit anim id est laborum.", &mut buf, 80);
///
/// assert_eq!(buf.lines().next()?.chars().count(), 80);
/// # Some(()) }
/// # test().unwrap()
/// ```
///
/// With word wrapping using a custom line ending:
///
/// ```rust
/// # fn test() -> Option<()> {
/// use base65536::{WrapOptions, encode_buf};
///
/// let mut buf = String::new();
/// base65536::encode_buf(
///     "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed \
///      do eiusmod tempor incididunt ut labore et dolore magna \
///      aliqua. Ut enim ad minim veniam, quis nostrud exercitation \
///      ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis \
///      aute irure dolor in reprehenderit in voluptate velit esse \
///      cillum dolore eu fugiat nulla pariatur. Excepteur sint \
///      occaecat cupidatat non proident, sunt in culpa qui officia \
///      deserunt mollit anim id est laborum.",
///     &mut buf,
///     WrapOptions::WrapAtWith(80, "\r\n"));
///
/// assert_eq!(buf.lines().next()?.chars().count(), 80);
/// # Some(()) }
/// # test().unwrap()
/// ```
///
/// [`WrapOptions`]: enum.WrapOptions.html
/// [`decode`]: fn.decode.html
/// [`decode_buf`]: fn.decode_buf.html
pub fn encode_buf<'a, T: ?Sized, W>(input: &T, buf: &mut String, wrap: W)
where
    T: AsRef<[u8]>,
    W: Into<WrapOptions<'a>>,
{
    use std::char::from_u32_unchecked;

    let wrap = wrap.into();
    for (count, bytes) in input.as_ref().chunks(2).enumerate() {
        match wrap {
            WrapOptions::NoWrap => {}
            WrapOptions::WrapAt(columns) => {
                if count % columns == 0 && count != 0 {
                    buf.push('\n');
                }
            }
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
