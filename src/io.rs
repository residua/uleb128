use std::io::{Read, Write};

use crate::error::{Error, Result};
use crate::{ULEB128_U32_MAX_LENGTH, ULEB128_U64_MAX_LENGTH};

const VALUE_MASK: u8 = 0b0111_1111;
const VALUE_LENGTH: usize = 7;

macro_rules! read_method_body {
    ($self:expr, $ty:ty, $len:expr) => {{
        let mut value = 0;
        let mut bytes_read = 0;

        loop {
            let mut buf = [0; 1];
            $self.read_exact(&mut buf)?;

            let byte = buf[0];
            let byte_value = (byte & VALUE_MASK) as $ty;
            value |= byte_value << (VALUE_LENGTH * bytes_read);

            bytes_read += 1;
            if bytes_read > $len {
                return Err(Error::LengthOverflow($len));
            }

            if (byte & !VALUE_MASK) == 0 {
                break;
            }
        }

        Ok(value)
    }};
}

/// Extends [`Read`][reader] with methods for reading numbers encoded in
/// [unsigned LEB128*]
///
/// # Examples
///
/// Read unsigned LEB128 integers from a [reader]:
///
/// ```
/// use std::io::Cursor;
/// use uleb128::ReadULeb128Ext;
///
/// let mut rdr = Cursor::new(vec![
///     0b0111_1111, // 127
///     0b1000_0000, 0b0000_0001, // 128
///     0b1000_0001, 0b0000_0001 // 129
/// ]);
///
/// assert_eq!(127, rdr.read_uleb128_u32().unwrap());
/// assert_eq!(128, rdr.read_uleb128_u32().unwrap());
/// assert_eq!(129, rdr.read_uleb128_u32().unwrap());
/// ```
///
/// [unsigned LEB128]: https://en.wikipedia.org/wiki/LEB128#Unsigned_LEB128
/// [reader]: https://doc.rust-lang.org/std/io/trait.Read.html
pub trait ReadULeb128Ext: Read {
    /// Read an unsigned 32-bit integer that's encoded in [unsigned LEB128]
    /// from the underlying [reader].
    ///
    /// # Errors
    ///
    /// If this function encounters an error when performing an I/O operation,
    /// then this function immediately returns an [`Error::Io`] to propagate the
    /// [`io::Error`] returned by an internal call to [`Read::read_exact`].
    ///
    /// If this function encounters an encoded number with a length in bytes
    /// greater than what is permitted, an [`Error::LengthOverflow`] is
    /// immediately returned.
    ///
    /// # Examples
    ///
    /// Read an unsigned LEB128-encoded, 32-bit integer:
    ///
    /// ```
    /// use std::io::Cursor;
    /// use uleb128::ReadULeb128Ext;
    ///
    /// let mut rdr = Cursor::new(vec![
    ///     // 2_147_483_647
    ///     0b1111_1111, 0b1111_1111, 0b1111_1111, 0b1111_1111, 0b0000_0111
    /// ]);
    ///
    /// assert_eq!(2_147_483_647, rdr.read_uleb128_u32().unwrap());
    /// ```
    ///
    /// [unsigned LEB128]: https://en.wikipedia.org/wiki/LEB128#Unsigned_LEB128
    /// [reader]: https://doc.rust-lang.org/std/io/trait.Read.html
    /// [`io::Error`]: std::io::Error
    fn read_uleb128_u32(&mut self) -> Result<u32> {
        read_method_body!(self, u32, ULEB128_U32_MAX_LENGTH)
    }

    /// Read an unsigned 64-bit integer that's encoded in [unsigned LEB128]
    /// from the underlying [reader].
    ///
    /// # Errors
    ///
    /// If this function encounters an error when performing an I/O operation,
    /// then this function immediately returns an [`Error::Io`] to propagate the
    /// [`io::Error`] returned by an internal call to [`Read::read_exact`].
    ///
    /// If this function encounters an encoded number with a length in bytes
    /// greater than what is permitted, an [`Error::LengthOverflow`] is
    /// immediately returned.
    ///
    /// # Examples
    ///
    /// Read an unsigned LEB128-encoded, 64-bit integer:
    ///
    /// ```
    /// use std::io::Cursor;
    /// use uleb128::ReadULeb128Ext;
    ///
    /// let mut rdr = Cursor::new(vec![
    ///     // 9_223_372_036_854_775_807
    ///     0b1111_1111, 0b1111_1111, 0b1111_1111, 0b1111_1111, 0b1111_1111,
    ///     0b1111_1111, 0b1111_1111, 0b1111_1111, 0b0111_1111
    /// ]);
    ///
    /// assert_eq!(9_223_372_036_854_775_807, rdr.read_uleb128_u64().unwrap());
    /// ```
    ///
    /// [unsigned LEB128]: https://en.wikipedia.org/wiki/LEB128#Unsigned_LEB128
    /// [reader]: https://doc.rust-lang.org/std/io/trait.Read.html
    /// [`io::Error`]: std::io::Error
    fn read_uleb128_u64(&mut self) -> Result<u64> {
        read_method_body!(self, u64, ULEB128_U64_MAX_LENGTH)
    }
}

impl<R: Read + ?Sized> ReadULeb128Ext for R {}

macro_rules! write_method_body {
    ($self:expr, $value:ident, $ty:ty) => {{
        let mut value = $value;
        loop {
            let mut byte = value & VALUE_MASK as $ty;
            value >>= VALUE_LENGTH;

            if value != 0 {
                byte |= !VALUE_MASK as $ty;
            }

            $self.write_all(&[byte as u8])?;

            if value == 0 {
                return Ok(());
            }
        }
    }};
}

/// Extends [`Write`][writer] with methods for writing unsigned integers to the
/// underlying writer encoded in [unsigned LEB128].
///
/// # Examples
///
/// Write unsigned integers to a [writer] encoded in LEB128:
///
/// ```
/// use uleb128::WriteULeb128Ext;
///
/// let mut wtr = vec![];
/// wtr.write_uleb128_u32(127).unwrap();
/// wtr.write_uleb128_u32(128).unwrap();
/// wtr.write_uleb128_u32(129).unwrap();
///
/// assert_eq!(wtr, vec![
///     0b0111_1111, // 127
///     0b1000_0000, 0b0000_0001, // 128
///     0b1000_0001, 0b0000_0001, // 129
/// ]);
/// ```
///
/// [unsigned LEB128]: https://en.wikipedia.org/wiki/LEB128#Unsigned_LEB128
/// [writer]: https://doc.rust-lang.org/std/io/trait.Write.html
pub trait WriteULeb128Ext: Write {
    /// Write an unsigned 32-bit integer to the underlying [writer] encoded in
    /// [unsigned LEB128].
    ///
    /// # Errors
    ///
    /// If this function encounters an error when performing an I/O operation,
    /// then this function immediately returns an [`Error::Io`] to propagate the
    /// [`io::Error`] returned by an internal call to [`Write::write_all`].
    ///
    /// # Examples
    ///
    /// Write an unsigned 32-bit integer to a [writer] encoded in LEB128:
    ///
    /// ```
    /// use uleb128::WriteULeb128Ext;
    ///
    /// let mut wtr = vec![];
    /// wtr.write_uleb128_u32(2_147_483_647).unwrap();
    ///
    /// assert_eq!(wtr, vec![
    ///     // 2_147_483_647
    ///     0b1111_1111, 0b1111_1111, 0b1111_1111, 0b1111_1111, 0b0000_0111
    /// ]);
    /// ```
    ///
    /// [unsigned LEB128]: https://en.wikipedia.org/wiki/LEB128#Unsigned_LEB128
    /// [writer]: https://doc.rust-lang.org/std/io/trait.Write.html
    /// [`io::Error`]: std::io::Error
    fn write_uleb128_u32(&mut self, value: u32) -> Result {
        write_method_body!(self, value, u32)
    }

    /// Write an unsigned 64-bit integer to the underlying [writer] encoded in
    /// [unsigned LEB128].
    ///
    /// # Errors
    ///
    /// If this function encounters an error when performing an I/O operation,
    /// then this function immediately returns an [`Error::Io`] to propagate the
    /// [`io::Error`] returned by an internal call to [`Write::write_all`].
    ///
    /// # Examples
    ///
    /// Write an unsigned 64-bit integer to a [writer] encoded in LEB128:
    ///
    /// ```
    /// use uleb128::WriteULeb128Ext;
    ///
    /// let mut wtr = vec![];
    /// wtr.write_uleb128_u64(9_223_372_036_854_775_807).unwrap();
    ///
    /// assert_eq!(wtr, vec![
    ///     // 9_223_372_036_854_775_807
    ///     0b1111_1111, 0b1111_1111, 0b1111_1111, 0b1111_1111, 0b1111_1111,
    ///     0b1111_1111, 0b1111_1111, 0b1111_1111, 0b0111_1111
    /// ]);
    /// ```
    ///
    /// [unsigned LEB128]: https://en.wikipedia.org/wiki/LEB128#Unsigned_LEB128
    /// [writer]: https://doc.rust-lang.org/std/io/trait.Write.html
    /// [`io::Error`]: std::io::Error
    fn write_uleb128_u64(&mut self, value: u64) -> Result {
        write_method_body!(self, value, u64)
    }
}

impl<W: Write + ?Sized> WriteULeb128Ext for W {}
