#[cfg(feature = "include-exclude")]
use globset::{Glob, GlobSet, GlobSetBuilder};
#[cfg(feature = "include-exclude")]
use std::sync::OnceLock;

/// A set of glob patterns that are compiled into a single [`GlobSet`] once and
/// then reused for every path check.
///
/// The original pattern strings are kept around so that the build-time
/// configuration can be embedded into the generated code (see the dynamic mode
/// codegen). The compiled `GlobSet` is built lazily on the first match and
/// cached in a `OnceLock`, so we only pay the glob compilation cost once rather
/// than on every path check.
#[cfg(feature = "include-exclude")]
#[derive(Debug, Default)]
pub struct PathMatcher {
    patterns: Vec<String>,
    set: OnceLock<GlobSet>,
}

#[cfg(feature = "include-exclude")]
impl PathMatcher {
    fn add(&mut self, pattern: String) {
        self.patterns.push(pattern);
        // A previously built set would be stale now, so drop it. It will be
        // rebuilt on the next match. In practice all patterns are added before
        // any match happens, so the set is only ever built once.
        self.set = OnceLock::new();
    }

    fn glob_set(&self) -> &GlobSet {
        self.set.get_or_init(|| {
            let mut builder = GlobSetBuilder::new();
            for pattern in &self.patterns {
                builder.add(Glob::new(pattern).expect("Failed to parse glob pattern"));
            }
            builder.build().expect("Failed to build glob set")
        })
    }

    fn is_match(&self, path: &str) -> bool {
        self.glob_set().is_match(path)
    }

    fn patterns(&self) -> &[String] {
        &self.patterns
    }
}

#[derive(Debug)]
pub struct Config {
    #[cfg(feature = "include-exclude")]
    include: PathMatcher,
    #[cfg(feature = "include-exclude")]
    exclude: PathMatcher,
    gzip: bool,
    br: bool,
    zstd: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            #[cfg(feature = "include-exclude")]
            include: PathMatcher::default(),
            #[cfg(feature = "include-exclude")]
            exclude: PathMatcher::default(),
            gzip: true,
            br: true,
            #[cfg(feature = "compression-zstd")]
            zstd: true,
            #[cfg(not(feature = "compression-zstd"))]
            zstd: false,
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    // Builder functions
    #[cfg(feature = "include-exclude")]
    pub fn add_include(&mut self, pattern: String) {
        self.include.add(pattern);
    }

    #[cfg(feature = "include-exclude")]
    pub fn add_exclude(&mut self, pattern: String) {
        self.exclude.add(pattern);
    }

    pub fn set_gzip(&mut self, status: bool) {
        self.gzip = status;
    }

    pub fn set_br(&mut self, status: bool) {
        self.br = status;
    }

    /// Enable or disable zstd compression for embedded files.
    pub fn set_zstd(&mut self, status: bool) {
        self.zstd = status;
    }

    #[cfg(feature = "include-exclude")]
    pub fn get_includes(&self) -> &[String] {
        self.include.patterns()
    }

    #[cfg(feature = "include-exclude")]
    pub fn get_excludes(&self) -> &[String] {
        self.exclude.patterns()
    }

    /// Check if a file at some path should be included based on this config.
    ///
    /// When deciding, includes always have priority over excludes. That means
    /// you typically will list paths you want excluded, then add includes to
    /// make an exception for some subset of files.
    #[allow(unused_variables)]
    pub fn should_include(&self, path: &str) -> bool {
        #[cfg(feature = "include-exclude")]
        {
            // Includes have priority.
            self.include.is_match(path)
            // If not, then we check if the file has been excluded. Any file
            // that is not explicitly excluded will be included.
            || !self.exclude.is_match(path)
        }
        #[cfg(not(feature = "include-exclude"))]
        {
            true
        }
    }

    pub fn should_gzip(&self) -> bool {
        self.gzip
    }

    pub fn should_br(&self) -> bool {
        self.br
    }

    /// Check if zstd compression should be used for embedded files.
    ///
    /// Returns `false` when the compression-zstd feature is not enabled,
    /// even if the config value is set to `true`.
    pub fn should_zstd(&self) -> bool {
        #[cfg(feature = "compression-zstd")]
        {
            self.zstd
        }
        #[cfg(not(feature = "compression-zstd"))]
        {
            false
        }
    }
}
