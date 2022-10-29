#![forbid(unsafe_code)]

#[allow(unused_imports)]
#[macro_use]
extern crate rust_embed_for_web_impl;
pub use rust_embed_for_web_impl::*;

pub use rust_embed_for_web_utils::{DynamicFile, EmbedableFile, EmbeddedFile};

#[doc(hidden)]
pub extern crate rust_embed_for_web_utils as utils;

/// A folder of embedded files.
///
/// The type of the file `RustEmbed::File` depends on whether we're in debug
/// mode or release mode:
///
/// - In debug mode it will be a `DynamicFile`
/// - In release mode it will be a `EmbeddedFile`
///
/// The derivation will automatically generate the correct file type. You don't
/// need to directly interface with the different file types that might get
/// returned: you should instead use the `EmbedableFile`  trait which is
/// implemented for both.
pub trait RustEmbed {
    type File: EmbedableFile;

    /// Get a file out of the folder.
    fn get(file_path: &str) -> Option<Self::File>;
}
