// Copyright 2017 Ethan Welker (nuew)
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//  http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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
#![cfg_attr(feature = "nightly", feature(test))]

#[cfg(feature = "nightly")]
extern crate test as test_crate;

#[cfg(test)]
mod test;

use std::{error, fmt};

mod block_starts;

const BLOCK_START_TO_INDEX: phf::Map<u32, u8> = include!(concat!(env!("OUT_DIR"), "/phf.rs"));

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

        if block_start == block_starts::PADDING_BLOCK_START {
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
/// [`encode`] and [`encode_buf`] when wrapping is enabled),
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
/// [`encode`] and [`encode_buf`] when wrapping is enabled),
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
/// [`encode`] and [`encode_buf`] when wrapping is enabled),
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
    })
    .map(|_| pos)
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
pub fn encode<'a, T, W>(input: &T, wrap: W) -> String
where
    T: ?Sized + AsRef<[u8]>,
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
pub fn encode_buf<'a, T, W>(input: &T, buf: &mut String, wrap: W)
where
    T: ?Sized + AsRef<[u8]>,
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
            1 => block_starts::PADDING_BLOCK_START,
            2 => block_starts::BLOCK_STARTS[bytes[1] as usize],
            _ => unreachable!(),
        };

        // It is safe to use this because we know that all code points within
        // 0x100 of any possible block_start are defined, and that's the
        // largest possible addition to block_start.
        let code_point = block_start + u32::from(bytes[0]);
        buf.push(unsafe { from_u32_unchecked(code_point) });
    }
}
