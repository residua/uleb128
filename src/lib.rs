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

#[macro_use]
extern crate quick_error;

mod error;
mod io;

pub use error::{Error, Result};
pub use io::{ReadULeb128Ext, WriteULeb128Ext};
