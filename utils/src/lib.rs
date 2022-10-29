#![forbid(unsafe_code)]

mod file;
pub use file::*;

pub struct FileEntry {
    pub rel_path: String,
    pub full_canonical_path: String,
}

#[cfg(not(feature = "include-exclude"))]
pub fn is_path_included(_path: &str, _includes: &[&str], _excludes: &[&str]) -> bool {
    true
}

#[cfg(feature = "include-exclude")]
pub fn is_path_included(rel_path: &str, includes: &[&str], excludes: &[&str]) -> bool {
    use globset::Glob;

    // ignore path matched by exclusion pattern
    for exclude in excludes {
        let pattern = Glob::new(exclude)
            .unwrap_or_else(|_| panic!("invalid exclude pattern '{}'", exclude))
            .compile_matcher();

        if pattern.is_match(rel_path) {
            return false;
        }
    }

    // accept path if no includes provided
    if includes.is_empty() {
        return true;
    }

    // accept path if matched by inclusion pattern
    for include in includes {
        let pattern = Glob::new(include)
            .unwrap_or_else(|_| panic!("invalid include pattern '{}'", include))
            .compile_matcher();

        if pattern.is_match(rel_path) {
            return true;
        }
    }

    false
}

pub fn get_files<'t>(folder_path: &'t str) -> impl Iterator<Item = FileEntry> + 't {
    walkdir::WalkDir::new(folder_path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter_map(move |e| {
            let rel_path = path_to_str(e.path().strip_prefix(&folder_path).unwrap());
            let full_canonical_path =
                path_to_str(std::fs::canonicalize(e.path()).expect("Could not get canonical path"));

            let rel_path = if std::path::MAIN_SEPARATOR == '\\' {
                rel_path.replace('\\', "/")
            } else {
                rel_path
            };

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
