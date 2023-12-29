pub struct FeatureSet {
    pub enable_file_handler: bool,
}

pub fn default() -> FeatureSet {
    FeatureSet {
        enable_file_handler: false,
    }
}