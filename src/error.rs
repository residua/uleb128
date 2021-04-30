use std::io;

/// A specialized [`Result`] type for unsigned LEB128 operations.
///
/// This type is broadly used across [`uleb128`](crate) for any operation which
/// may produce an error.
///
/// This type alias is generally used to avoid writing out
/// [`uleb128::Error`](Error) directly. Additionally, if no [`Ok`] type is
/// provided, it defaults to [`()`] due to its frequency. Otherwise, this is a
/// direct mapping to [`Result`].
///
/// While usual Rust style is to import types directly, aliases of [`Result`]
/// often are not, to make it easier to distinguish between them. [`Result`]
/// is generally assumed to be [`std::result::Result`], and so users of this
/// alias will generally use `uleb128::Result` instead of shadowing the
/// [prelude]'s import of [`std::result::Result`].
///
/// [`Result`]: std::result::Result
/// [`uleb128`]: crate
/// [`uleb128::Error`]: Error
/// [`Ok`]: std::result::Result::Ok
/// [`()`]: https://doc.rust-lang.org/std/primitive.unit.html
/// [prelude]: https://doc.rust-lang.org/std/prelude/index.html
pub type Result<T = (), E = Error> = std::result::Result<T, E>;

quick_error! {
    /// The error type for LEB128 operations of the [`ReadULeb128Ext`] and the
    /// [`WriteULeb128Ext`] extension traits.
    ///
    /// [`ReadULeb128Ext`]: crate::ReadULeb128Ext
    /// [`WriteULeb128Ext`]: crate::WriteULeb128Ext
    #[derive(Debug)]
    pub enum Error {
        /// An I/O operation failed.
        Io(err: io::Error) {
            from()
            source(err)
            display("io error: {}", err)
        }
        /// The read operation encountered data that was too long.
        LengthOverflow(max: usize) {
            display("can not read more than {} bytes", max)
        }
    }
}
