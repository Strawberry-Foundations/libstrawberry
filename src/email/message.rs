#[derive(Default)]
pub enum ContentType {
    #[default]
    PLAIN,
    HTML,
}

#[derive(Default)]
pub enum Content {
    #[default]
    Default,
    Plain(&'static str),
    Html(&'static str),
}
