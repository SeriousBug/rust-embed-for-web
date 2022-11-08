use globset::{Glob, GlobMatcher};

#[derive(Debug)]
pub struct Config {
    include: Vec<GlobMatcher>,
    exclude: Vec<GlobMatcher>,
    gzip: bool,
    br: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            include: vec![],
            exclude: vec![],
            gzip: true,
            br: true,
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    // Builder functions
    pub fn add_include(&mut self, pattern: String) {
        self.include.push(
            Glob::new(&pattern)
                .expect("Failed to parse glob pattern for include")
                .compile_matcher(),
        );
    }

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

    pub fn get_includes(&self) -> &Vec<GlobMatcher> {
        &self.include
    }

    pub fn get_excludes(&self) -> &Vec<GlobMatcher> {
        &self.exclude
    }

    /// Check if a file at some path should be included based on this config.
    ///
    /// When deciding, includes always have priority over excludes. That means
    /// you typically will list paths you want excluded, then add includes to
    /// make an exception for some subset of files.
    pub fn should_include(&self, path: &str) -> bool {
        // Includes have priority.
        self.include
            .iter()
            .find(|include| include.is_match(path))
            .is_some()
            // If not, then we check if the file has been excluded. Any file
            // that is not explicitly excluded will be 
            || self
                .exclude
                .iter()
                .find(|exclude| exclude.is_match(path))
                .is_none()
    }

    pub fn should_gzip(&self) -> bool {
        self.gzip
    }

    pub fn should_br(&self) -> bool {
        self.br
    }
}
