#[cfg(feature = "include-exclude")]
use globset::{Glob, GlobMatcher};

#[derive(Debug)]
pub struct Config {
    #[cfg(feature = "include-exclude")]
    include: Vec<GlobMatcher>,
    #[cfg(feature = "include-exclude")]
    exclude: Vec<GlobMatcher>,
    gzip: bool,
    br: bool,
    zstd: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            #[cfg(feature = "include-exclude")]
            include: vec![],
            #[cfg(feature = "include-exclude")]
            exclude: vec![],
            gzip: true,
            br: true,
            zstd: true,
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
        self.include.push(
            Glob::new(&pattern)
                .expect("Failed to parse glob pattern for include")
                .compile_matcher(),
        );
    }

    #[cfg(feature = "include-exclude")]
    pub fn add_exclude(&mut self, pattern: String) {
        self.exclude.push(
            Glob::new(&pattern)
                .expect("Failed to parse glob pattern for exclude")
                .compile_matcher(),
        );
    }

    pub fn set_gzip(&mut self, status: bool) {
        self.gzip = status;
    }

    pub fn set_br(&mut self, status: bool) {
        self.br = status;
    }

    pub fn set_zstd(&mut self, status: bool) {
        self.zstd = status;
    }

    #[cfg(feature = "include-exclude")]
    pub fn get_includes(&self) -> &Vec<GlobMatcher> {
        &self.include
    }

    #[cfg(feature = "include-exclude")]
    pub fn get_excludes(&self) -> &Vec<GlobMatcher> {
        &self.exclude
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
            self.include
            .iter()
            .any(|include| include.is_match(path))
            // If not, then we check if the file has been excluded. Any file
            // that is not explicitly excluded will be 
            || !self
                .exclude
                .iter()
                .any(|exclude| exclude.is_match(path))
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

    pub fn should_zstd(&self) -> bool {
        self.zstd
    }
}
