//! An implementation of [base65536][1] in Rust.
//! Usage is similar to that of [`base64`][2].
//!
//! [1]: https://github.com/qntm/base65536
//! [2]: https://crates.io/crates/base64

#[macro_use]
extern crate lazy_static;

#[cfg(feature = "fnv")]
extern crate fnv;

#[cfg(test)]
mod test;

use std::{error, fmt};

const PADDING_BLOCK_START: u32 = 0x1500;
const BLOCK_STARTS: &'static [u32] =
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

#[cfg(feature = "fnv")]
use fnv::FnvHashMap;

#[cfg(feature = "fnv")]
lazy_static! {
    static ref B2S: FnvHashMap<u32, u8> = {
        let mut b2s = FnvHashMap::with_capacity_and_hasher(BLOCK_STARTS.len(), Default::default());

        for b in 0..BLOCK_STARTS.len() {
           b2s.insert(BLOCK_STARTS[b], b as u8);
        }

        b2s
    };
}

#[cfg(not(feature = "fnv"))]
use std::collections::HashMap;

#[cfg(not(feature = "fnv"))]
lazy_static! {
    static ref B2S: HashMap<u32, u8> = {
        let mut b2s = HashMap::with_capacity(BLOCK_STARTS.len());

        for b in 0..BLOCK_STARTS.len() {
           b2s.insert(BLOCK_STARTS[b], b as u8);
        }

        b2s
    };
}

#[derive(Clone, Copy, Debug)]
/// Contains configuration parameters for base65536 encoding/decoding
pub struct Config {
    ignore_garbage: bool,
    line_wrap: Option<(usize, &'static str)>,
}

impl Config {
    #[inline]
    pub fn new() -> Config {
        Config::default()
    }

    #[inline]
    /// Ignore non-base65536 code points in text.
    ///
    /// Without this set, [`decode`] and friends are *very* strict,
    /// even failing on line breaks. This is to match behaviour with
    /// the original implementation.
    ///
    /// [`decode`]: fn.decode.html
    pub fn ignore_garbage(&mut self, ignore_garbage: bool) -> Config {
        self.ignore_garbage = ignore_garbage;
        *self
    }

    #[inline]
    /// Wrap output at a column with a custom string. You should generally use
    /// "\n", except on Windows, where you might want to use "\r\n".
    ///
    /// Unless called with [`ignore_garbage`] set to `true`, a decoder will fail
    /// on output created with this enabled. This is to match behaviour with the
    /// original implementation.
    ///
    /// [`ignore_garbage`]: #method.ignore_garbage
    pub fn wrap(&mut self, wrap: Option<(usize, &'static str)>) -> Config {
        self.line_wrap = wrap;
        *self
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            ignore_garbage: false,
            line_wrap: None,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
/// Represents an error in the decoding process
pub enum Error {
    InvalidCodePoint(usize, char),
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

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

pub type Result<T> = ::std::result::Result<T, Error>;

#[inline]
/// Decode from string reference as octets.
/// Convenience method for `decode_config(input, Config::default())`
///
/// Note that `decode` and friends are *very* strict by default, even failing
/// on line breaks, as to match behaviour with the original implementation.
/// To prevent this, instead use with the [`ignore_garbage`]
/// option, as such:
///
/// ```rust
/// # extern crate base65536;
/// # use base65536::{Config, decode_config};
/// # fn main() {
/// # let input = "";
/// decode_config(input, Config::new().ignore_garbage(true))
/// # .unwrap();
/// # }
/// ```
///
/// [`ignore_garbage`]: struct.Config.html#method.ignore_garbage
pub fn decode<T: ?Sized + AsRef<str>>(input: &T) -> self::Result<Vec<u8>> {
    decode_config(input, Config::default())
}

#[inline]
/// Decode from string reference as octets.
///
/// Note that `decode_config` and friends are *very* strict by default, even
/// failing on line breaks, as to match behaviour with the original
/// implementation. To prevent this, instead use with the [`ignore_garbage`]
/// option, as such:
///
/// ```rust
/// # extern crate base65536;
/// # use base65536::{Config, decode_config};
/// # fn main() {
/// # let input = "";
/// decode_config(input, Config::new().ignore_garbage(true))
/// # .unwrap();
/// # }
///
/// [`ignore_garbage`]: struct.Config.html#method.ignore_garbage
pub fn decode_config<T: ?Sized + AsRef<str>>(input: &T, config: Config) -> self::Result<Vec<u8>> {
    let mut buf = Vec::new();
    decode_config_buf(input, config, &mut buf).map(|_| buf)
}

/// Decode from string reference as octets.
/// Writes into supplied buffer to avoid allocation.
///
/// Note that `decode_config_buf` and friends are *very* strict by default,
/// even failing on line breaks, as to match behaviour with the original
/// implementation. To prevent this, instead use with the [`ignore_garbage`]
/// option, as such:
///
/// ```rust
/// # extern crate base65536;
/// # use base65536::{Config, decode_config_buf};
/// # fn main() {
/// # let input = "";
/// let mut result = Vec::new();
///
/// decode_config_buf(input, Config::new().ignore_garbage(true), &mut result)
/// # .unwrap();
/// # }
///
/// [`ignore_garbage`]: struct.Config.html#method.ignore_garbage
pub fn decode_config_buf<T: ?Sized + AsRef<str>>(input: &T,
                                                 config: Config,
                                                 buf: &mut Vec<u8>)
                                                 -> self::Result<()> {
    use std::char::from_u32_unchecked;
    let input = input.as_ref();

    let mut done = false;
    for (index, code_point) in input.char_indices() {
        let code_point = code_point as u32;

        let byte1 = code_point & ((1 << 8) - 1);
        let block_start = code_point - byte1;

        if block_start == PADDING_BLOCK_START {
            if done {
                return Err(Error::InvalidLength);
            }

            buf.push(byte1 as u8);
            done = true;
        } else {
            let byte2 = B2S.get(&block_start);

            if let Some(byte2) = byte2 {
                if done {
                    return Err(Error::InvalidLength);
                }

                buf.push(byte1 as u8);
                buf.push(*byte2);
            } else if !config.ignore_garbage {
                // safe because the input is a char, which is guaranteed valid
                return Err(Error::InvalidCodePoint(index,
                                                   unsafe { from_u32_unchecked(code_point) }));
            }
        }
    }

    Ok(())
}

#[inline]
/// Encode arbitrary octets as base65536.
/// Convienence method for `encode_config(input, Config::default())`
pub fn encode<T: ?Sized + AsRef<[u8]>>(input: &T) -> String {
    encode_config(input, Config::default())
}

#[inline]
/// Encode arbitrary octets as base65536.
pub fn encode_config<T: ?Sized + AsRef<[u8]>>(input: &T, config: Config) -> String {
    let mut output = String::new();
    encode_config_buf(input, config, &mut output);
    output
}

/// Encode arbitrary octets as base65536.
/// Writes into supplied buffer to avoid allocation.
pub fn encode_config_buf<T: ?Sized + AsRef<[u8]>>(input: &T, config: Config, buf: &mut String) {
    use std::char::from_u32_unchecked;
    let input = input.as_ref();

    let mut i = 0;
    while i < input.len() {
        // calculate the to-be-output code point
        let byte1 = input[i];
        let block_start = if i + 1 < input.len() {
            BLOCK_STARTS[input[i + 1] as usize]
        } else {
            PADDING_BLOCK_START
        };
        let code_point = block_start + byte1 as u32;

        // output wrap if requested
        if let Some((column, eol)) = config.line_wrap {
            if (i / 2) % column == 0 && i != 0 {
                buf.push_str(eol);
            }
        }

        // output code point
        //
        // It is safe to use this because we know that all code points within
        // 0x100 of any possible block_start are defined.
        buf.push(unsafe { from_u32_unchecked(code_point) });

        i += 2; // our loop goes up by two
    }
}
