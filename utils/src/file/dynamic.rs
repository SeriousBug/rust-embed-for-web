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

    fn last_modified(&self) -> Option<Self::Meta> {
        self.last_modified_timestamp()
            .map(|v| chrono::Utc.timestamp(v, 0).to_rfc2822())
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
                    .and_then(|v| v.as_secs().try_into().ok().map(|v: i64| v * -1))
            })
    })
}

impl DynamicFile {
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
