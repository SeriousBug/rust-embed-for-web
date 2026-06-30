//! This crate contains utility code for `rust-embed-for-web`.
//!
//! You generally don't want to use this crate directly, `rust-embed-for-web`
//! re-exports any necessary parts from this crate.
#![forbid(unsafe_code)]

mod file;
pub use file::*;

mod config;
pub use config::Config;

pub struct FileEntry {
    pub rel_path: String,
    pub full_canonical_path: String,
}

pub fn get_files<'t>(
    folder_path: &'t str,
    config: &'t Config,
    prefix: &'t str,
) -> impl Iterator<Item = FileEntry> + 't {
    walkdir::WalkDir::new(folder_path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter_map(move |e| {
            let rel_path = path_to_str(e.path().strip_prefix(folder_path).unwrap());
            let rel_path = format!("{prefix}{rel_path}");
            let full_canonical_path = match std::fs::canonicalize(e.path()) {
                Ok(path) => path_to_str(path),
                // With allow_missing set, tolerate entries that disappear (e.g.
                // broken symlinks) instead of aborting the build.
                Err(err)
                    if config.allow_missing() && err.kind() == std::io::ErrorKind::NotFound =>
                {
                    return None
                }
                Err(err) => panic!("Could not get canonical path: {}", err),
            };

            let rel_path = if std::path::MAIN_SEPARATOR == '\\' {
                rel_path.replace('\\', "/")
            } else {
                rel_path
            };

            if !config.should_include(&rel_path) {
                return None;
            }

            Some(FileEntry {
                rel_path,
                full_canonical_path,
            })
        })
}

fn path_to_str<P: AsRef<std::path::Path>>(p: P) -> String {
    p.as_ref()
        .to_str()
        .expect("Path does not have a string representation")
        .to_owned()
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;

    #[test]
    fn allow_missing_skips_broken_symlinks() {
        let dir = std::env::temp_dir().join(format!("rust-embed-for-web-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let link = dir.join("broken.txt");
        let _ = std::fs::remove_file(&link);
        std::os::unix::fs::symlink(dir.join("does-not-exist"), &link).unwrap();

        let mut config = Config::default();
        config.set_allow_missing(true);
        let files: Vec<_> = get_files(dir.to_str().unwrap(), &config, "").collect();

        std::fs::remove_dir_all(&dir).unwrap();
        assert!(files.is_empty());
    }
}
