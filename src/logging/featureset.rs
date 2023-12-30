pub struct FeatureSet {
    pub enable_file_handler: bool,
}

pub enum Features {
    FileHandler,
}

impl FeatureSet {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            enable_file_handler: false,
        }
    }

    pub fn enable(&mut self, feature: &Features) {
        match feature {
            Features::FileHandler => self.enable_file_handler = true,
        }
    }

    pub fn disable(&mut self, feature: &Features) {
        match feature {
            Features::FileHandler => self.enable_file_handler = false,
        }
    }

    #[must_use]
    pub const fn target_return(&self) -> &Self {
        self
    }
}