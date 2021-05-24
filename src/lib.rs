//! Extension traits that allow for the reading and writing of [unsigned LEB128]
//! integer values.
//!
//! # Read
//!
//! To read [unsigned LEB128] integers, do so through the [`ReadULeb128Ext`]
//! extension trait. Importing this trait into a file allows for reading
//! unsigned integers encoded in [unsigned LEB128] from any type that implements
//! [`Read`].
//!
//! ```
//! use std::io::Cursor;
//! use uleb128::ReadULeb128Ext;
//!
//! let mut rdr = Cursor::new(vec![0b1000_0000, 0b0000_0001]);
//! assert_eq!(128, rdr.read_uleb128_u32().unwrap());
//! ```
//!
//! # Write
//!
//! To write unsigned integers as [unsigned LEB128], do so through the
//! [`WriteULeb128Ext`] extension trait. Importing this trait into a file allows
//! for writing unsigned integers as [unsigned LEB128] from any type that
//! implements [`Write`].
//!
//! ```
//! use uleb128::WriteULeb128Ext;
//!
//! let mut wtr = vec![];
//! wtr.write_uleb128_u32(128).unwrap();
//!
//! assert_eq!(wtr, vec![0b1000_0000, 0b0000_0001]);
//! ```
//!
//! [unsigned LEB128]: https://en.wikipedia.org/wiki/LEB128#Unsigned_LEB128
//! [`ReadULeb128Ext`]: crate::ReadULeb128Ext
//! [`Read`]: std::io::Read
//! [`Write`]: std::io::Write

#![warn(missing_docs)]

#[macro_use]
extern crate quick_error;

mod error;
mod io;

pub use error::{Error, Result};
pub use io::{ReadULeb128Ext, WriteULeb128Ext};

pub(crate) const ULEB128_U32_MAX_LENGTH: usize = 5;
pub(crate) const ULEB128_U64_MAX_LENGTH: usize = 10;

const fn max_value(len: usize) -> usize {
    128usize.pow(len as u32) - 1
}

macro_rules! len_body {
    ($n:ident, $ty:ty, $len:expr) => {{
        for len in 1..$len {
            if $n <= max_value(len) as $ty {
                return len;
            }
        }
        $len
    }};
}

/// Get the length of the unsigned 32-bit integer's [unsigned LEB128]
/// representation in bytes.
///
/// # Examples
///
/// ```
/// use uleb128::uleb128_u32_len;
///
/// assert_eq!(1, uleb128_u32_len(127));
/// assert_eq!(2, uleb128_u32_len(128));
/// ```
///
/// [unsigned LEB128]: https://en.wikipedia.org/wiki/LEB128#Unsigned_LEB128
pub fn uleb128_u32_len(n: u32) -> usize {
    len_body!(n, u32, ULEB128_U32_MAX_LENGTH)
}

/// Get the length of the unsigned 64-bit integer's [unsigned LEB128]
/// representation in bytes.
///
/// # Examples
///
/// ```
/// use uleb128::uleb128_u64_len;
///
/// assert_eq!(5, uleb128_u64_len(34_359_738_367));
/// assert_eq!(6, uleb128_u64_len(34_359_738_368));
/// ```
///
/// [unsigned LEB128]: https://en.wikipedia.org/wiki/LEB128#Unsigned_LEB128
pub fn uleb128_u64_len(n: u64) -> usize {
    len_body!(n, u64, ULEB128_U64_MAX_LENGTH)
}
