/// An embedable file.
///
/// The file is embedded into the program for release builds, and dynamically
/// read for debug builds. This trait allows common access for both types.
///
/// There are associated types for each file which you can get with
/// `EmbeddedFile::Data`/`EmbeddedFile::Meta` or
/// `DynamicFile::Data`/`DynamicFile::Meta`. The data of the file depends on
/// this associated types because embedded files use static types while dynamic
/// files have to use owned types.
///
/// You can access the data by calling the `as_ref` function through the `AsRef`
/// trait. For example:
/// ```
/// # use rust_embed_for_web_utils::EmbedableFile;
/// fn print_hash<T: EmbedableFile>(file: T) {
///     println!("The file hash is {}", file.hash().as_ref());
/// }
/// ```
pub trait EmbedableFile {
    type Data: 'static + AsRef<[u8]>;
    type Meta: 'static + AsRef<str>;

    /// The name of the embedded file.
    fn name(&self) -> Self::Meta;
    /// The contents of the embedded file.
    fn data(&self) -> Self::Data;
    /// The contents of the file, compressed with gzip.
    ///
    /// This is `Some` if precompression has been done. `None` if the file was
    /// not precompressed, either because the file doesn't benefit from
    /// compression or because gzip was disabled with `#[gzip = false]`.
    fn data_gzip(&self) -> Option<Self::Data>;
    /// The contents of the file, compressed with brotli.
    ///
    /// This is `Some` if precompression has been done. `None` if the file was
    /// not precompressed, either because the file doesn't benefit from
    /// compression or because gzip was disabled with `#[br = false]`.
    fn data_br(&self) -> Option<Self::Data>;
    /// The contents of the file, compressed with zstd.
    ///
    /// This is `Some` if precompression has been done. `None` if the file was
    /// not precompressed, either because the file doesn't benefit from
    /// compression or because zstd was disabled with `#[zstd = false]`.
    #[cfg(feature = "compression-zstd")]
    fn data_zstd(&self) -> Option<Self::Data>;

    /// The contents of the file, compressed with zstd.
    ///
    /// Always returns `None` when the compression-zstd feature is disabled.
    #[cfg(not(feature = "compression-zstd"))]
    fn data_zstd(&self) -> Option<Self::Data> {
        None
    }
    /// The UNIX timestamp of when the file was last modified.
    fn last_modified_timestamp(&self) -> Option<i64>;
    /// The rfc2822 encoded last modified date. This is the format you use for
    /// `Last-Modified` headers.
    fn last_modified(&self) -> Option<Self::Meta>;
    /// The hash value for the file. This is a base85 encoded sha256 hash.
    fn hash(&self) -> Self::Meta;
    /// The ETag value for the file. This is just the file hash, wrapped with
    /// quote symbols.
    fn etag(&self) -> Self::Meta;
    /// The mime type for the file, if one can be guessed from the file
    /// extension.
    fn mime_type(&self) -> Option<Self::Meta>;
}
