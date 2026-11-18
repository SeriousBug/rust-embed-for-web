use std::{
    convert::TryInto,
    fmt::Debug,
    io::{BufReader, Read},
    path::Path,
    time::SystemTime,
};

use chrono::TimeZone;
use new_mime_guess::MimeGuess;
use sha2::{Digest, Sha256};

use super::common::EmbedableFile;

/// A file read from the file system dynamically.
///
/// `rust-embed-for-web` changes which type of file you get based on whether
/// it's a debug or release build. In debug builds, you'll get `DynamicFile`s.
///
/// You should interface with this object using the `EmbedableFile` trait, which
/// is implemented for both the embedded and dynamic files.
#[derive(Clone)]
pub struct DynamicFile {
    name: String,
    data: Vec<u8>,
    hash: String,
    last_modified_timestamp: Option<i64>,
    mime_type: Option<String>,
}

impl EmbedableFile for DynamicFile {
    type Data = Vec<u8>;
    type Meta = String;

    fn name(&self) -> Self::Meta {
        self.name.clone()
    }

    fn data(&self) -> Self::Data {
        self.data.clone()
    }

    fn data_gzip(&self) -> Option<Self::Data> {
        None
    }

    fn data_br(&self) -> Option<Self::Data> {
        None
    }

    #[cfg(feature = "compression-zstd")]
    fn data_zstd(&self) -> Option<Self::Data> {
        None
    }

    fn last_modified(&self) -> Option<Self::Meta> {
        self.last_modified_timestamp()
            .map(|v| chrono::Utc.timestamp_opt(v, 0).unwrap().to_rfc2822())
    }

    fn last_modified_timestamp(&self) -> Option<i64> {
        self.last_modified_timestamp
    }

    fn hash(&self) -> Self::Meta {
        self.hash.clone()
    }

    fn etag(&self) -> Self::Meta {
        format!("\"{}\"", self.hash)
    }

    fn mime_type(&self) -> Option<Self::Meta> {
        self.mime_type.clone()
    }
}

fn modified_unix_timestamp(metadata: &std::fs::Metadata) -> Option<i64> {
    metadata.modified().ok().and_then(|modified| {
        modified
            .duration_since(SystemTime::UNIX_EPOCH)
            .ok()
            .and_then(|v| v.as_secs().try_into().ok())
            .or_else(|| {
                SystemTime::UNIX_EPOCH
                    .duration_since(modified)
                    .ok()
                    .and_then(|v| v.as_secs().try_into().ok().map(|v: i64| -v))
            })
    })
}

impl DynamicFile {
    pub fn without_mtime(mut self) -> Self {
        self.last_modified_timestamp = None;
        self
    }

    pub fn read_from_fs<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let file = std::fs::OpenOptions::new().read(true).open(&path)?;

        let last_modified_timestamp = modified_unix_timestamp(&file.metadata()?);

        let mut data = Vec::new();
        BufReader::new(file).read_to_end(&mut data)?;

        let mut hasher = Sha256::new();
        hasher.update(&data);
        let hash = hasher.finalize();
        let hash = base85rs::encode(&hash[..]);

        let mime_type = MimeGuess::from_path(&path).first().map(|v| v.to_string());
        let name = Path::file_name(path.as_ref())
            .expect("Unable to parse the file name")
            .to_string_lossy()
            .to_string();

        Ok(DynamicFile {
            name,
            data,
            hash,
            last_modified_timestamp,
            mime_type,
        })
    }
}

impl Debug for DynamicFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DynamicFile")
            .field("name", &self.name)
            .field("hash", &self.hash)
            .field("last_modified", &self.last_modified())
            .field("mime_type", &self.mime_type)
            .finish()
    }
}

impl PartialEq for DynamicFile {
    fn eq(&self, other: &Self) -> bool {
        self.hash.eq(&other.hash)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read_cargo_toml() -> DynamicFile {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
        DynamicFile::read_from_fs(path).unwrap()
    }

    #[test]
    fn mtime_is_present_when_read_from_fs() {
        let file = read_cargo_toml();
        assert!(file.last_modified_timestamp().is_some());
        assert!(file.last_modified().is_some());
    }

    #[test]
    fn without_mtime_drops_the_timestamp() {
        let file = read_cargo_toml().without_mtime();
        assert!(file.last_modified_timestamp().is_none());
        assert!(file.last_modified().is_none());
    }
}
