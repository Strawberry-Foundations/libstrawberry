pub struct FeatureSet {
    pub enable_file_handler: bool,
}

impl FeatureSet {
    pub fn new() -> Self {
        Self {
            enable_file_handler: false,
        }
    }
}