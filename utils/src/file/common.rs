/// An embedable file.
///
/// The file is embedded into the program for release builds, and dynamically
/// read for debug builds.
pub trait EmbedableFile {
    type Data: 'static + AsRef<[u8]>;
    type Meta: 'static + AsRef<str>;

    /// The contents of the embedded file.
    fn data(&self) -> Self::Data;
    /// The contents of the file, compressed with gzip.
    ///
    /// If precompression has been done. None if the file was not precompressed.
    fn data_gzip(&self) -> Option<Self::Data>;
    /// The contents of the file, compressed with brotli.
    ///
    /// If precompression has been done. None if the file was not precompressed.
    fn data_br(&self) -> Option<Self::Data>;
    /// The timestamp of when the file was last modified.
    fn last_modified_timestamp(&self) -> Option<i64>;
    /// The rfc2822 encoded last modified date.
    fn last_modified(&self) -> Option<Self::Meta>;
    /// The hash value for the file.
    fn hash(&self) -> Self::Meta;
    /// The ETag value for the file, based on its hash.
    fn etag(&self) -> Self::Meta;
    /// The mime type for the file, if one is or can be guessed from the file.
    fn mime_type(&self) -> Option<Self::Meta>;
}
