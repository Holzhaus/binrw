use crate::{
    io::{Read, Seek},
    BinResult, Endian,
};

mod options;
pub use options::*;

mod impls;
pub use impls::VecArgs;

/// The `BinRead` trait reads data from streams and converts it into objects.
///
/// [`io`]: crate::io
///
/// This trait is usually derived, but can also be manually implemented by
/// writing an appropriate [`Args`] type and [`read_options()`] function.
///
/// [`Args`]: Self::Args
/// [`read_options()`]: Self::read_options
///
/// # Derivable
///
/// This trait can be used with `#[derive]` or `#[derive_binread]`. Each field
/// of a derived type must either implement `BinRead` or be annotated with an
/// attribute containing a [`map`], [`try_map`], or [`parse_with`] directive.
///
/// [`map`]: crate::attribute#map
/// [`parse_with`]: crate::attribute#parse_with
/// [`try_map`]: crate::attribute#map
///
/// Using `#[derive_binread]` instead of `#[derive]` is required when using
/// [temporary fields].
///
/// [temporary fields]: crate::attribute#temp
pub trait BinRead: Sized + 'static {
    /// The type used for the `args` parameter of [`read_args()`] and
    /// [`read_options()`].
    ///
    /// When the given type implements [`Default`], convenience functions like
    /// [`read()`] are enabled. `BinRead` implementations that don’t receive any
    /// arguments should use the `()` type.
    ///
    /// When `BinRead` is derived, the [`import`] and [`import_tuple`]
    /// directives define this type.
    ///
    /// [`import`]: crate::attribute#arguments
    /// [`import_tuple`]: crate::attribute#arguments
    /// [`read()`]: Self::read
    /// [`read_args()`]: Self::read_args
    /// [`read_options()`]: Self::read_options
    type Args: Clone;

    /// Read `Self` from the reader using default arguments.
    fn read<R: Read + Seek>(reader: &mut R) -> BinResult<Self>
    where
        Self::Args: Default,
    {
        Self::read_options(reader, &ReadOptions::default(), Self::Args::default())
    }

    /// Read `Self` from the reader using the given arguments.
    fn read_args<R: Read + Seek>(reader: &mut R, args: Self::Args) -> BinResult<Self> {
        Self::read_options(reader, &ReadOptions::default(), args)
    }

    /// Read `Self` from the reader using the given [`ReadOptions`] and
    /// arguments.
    fn read_options<R: Read + Seek>(
        reader: &mut R,
        options: &ReadOptions,
        args: Self::Args,
    ) -> BinResult<Self>;

    /// Runs any post-processing steps required to finalize construction of the
    /// object.
    fn after_parse<R: Read + Seek>(
        &mut self,
        _: &mut R,
        _: &ReadOptions,
        _: Self::Args,
    ) -> BinResult<()> {
        Ok(())
    }
}

/// Extension methods for reading [`BinRead`] objects directly from a reader.
///
/// # Examples
///
/// ```rust
/// use binrw::BinReaderExt;
/// use binrw::endian::LE;
/// use binrw::io::Cursor;
///
/// let mut reader = Cursor::new(b"\x07\0\0\0\xCC\0\0\x05");
/// let x: u32 = reader.read_le().unwrap();
/// let y: u16 = reader.read_type(LE).unwrap();
/// let z = reader.read_be::<u16>().unwrap();
///
/// assert_eq!((x, y, z), (7u32, 0xCCu16, 5u16));
/// ```
pub trait BinReaderExt: Read + Seek + Sized {
    /// Read `T` from the reader with the given byte order.
    fn read_type<T: BinRead>(&mut self, endian: Endian) -> BinResult<T>
    where
        T::Args: Default,
    {
        self.read_type_args(endian, T::Args::default())
    }

    /// Read `T` from the reader assuming big-endian byte order.
    fn read_be<T: BinRead>(&mut self) -> BinResult<T>
    where
        T::Args: Default,
    {
        self.read_type(Endian::Big)
    }

    /// Read `T` from the reader assuming little-endian byte order.
    fn read_le<T: BinRead>(&mut self) -> BinResult<T>
    where
        T::Args: Default,
    {
        self.read_type(Endian::Little)
    }

    /// Read `T` from the reader assuming native-endian byte order.
    fn read_ne<T: BinRead>(&mut self) -> BinResult<T>
    where
        T::Args: Default,
    {
        self.read_type(Endian::Native)
    }

    /// Read `T` from the reader with the given byte order and arguments.
    fn read_type_args<T: BinRead>(&mut self, endian: Endian, args: T::Args) -> BinResult<T> {
        let options = ReadOptions::default().with_endian(endian);

        let mut res = T::read_options(self, &options, args.clone())?;
        res.after_parse(self, &options, args)?;

        Ok(res)
    }

    /// Read `T` from the reader, assuming big-endian byte order, using the
    /// given arguments.
    fn read_be_args<T: BinRead>(&mut self, args: T::Args) -> BinResult<T> {
        self.read_type_args(Endian::Big, args)
    }

    /// Read `T` from the reader, assuming little-endian byte order, using the
    /// given arguments.
    fn read_le_args<T: BinRead>(&mut self, args: T::Args) -> BinResult<T> {
        self.read_type_args(Endian::Little, args)
    }

    /// Read `T` from the reader, assuming native-endian byte order, using the
    /// given arguments.
    fn read_ne_args<T: BinRead>(&mut self, args: T::Args) -> BinResult<T> {
        self.read_type_args(Endian::Native, args)
    }
}

impl<R: Read + Seek + Sized> BinReaderExt for R {}
