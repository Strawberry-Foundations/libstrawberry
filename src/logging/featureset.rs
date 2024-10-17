/// Struct for creating a custom featureset for `stblib::logger`
pub struct FeatureSet {
    pub enable_file_handler: bool,
}

pub enum Features {
    FileHandler,
}

impl Default for FeatureSet {
    fn default() -> Self {
        Self::new()
    }
}

impl FeatureSet {
    /// Create a new featureset object
    #[must_use]
    pub const fn new() -> Self {
        Self {
            enable_file_handler: false,
        }
    }

    /// Enable a feature
    pub fn enable(&mut self, feature: &Features) {
        match feature {
            Features::FileHandler => self.enable_file_handler = true,
        }
    }

    /// Disable a feature
    pub fn disable(&mut self, feature: &Features) {
        match feature {
            Features::FileHandler => self.enable_file_handler = false,
        }
    }

    /// return self
    #[must_use]
    pub const fn unwrap(&self) -> &Self {
        self
    }
}