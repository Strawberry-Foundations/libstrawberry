use crate::scapi::context::Context;

pub type BoxFuture<T> = std::pin::Pin<Box<dyn std::future::Future<Output = T> + Send>>;
pub type CommandResponse = Result<Option<String>, String>;

#[derive(Clone)]
pub struct Command {
    /// Name of command (execution name, e.g. test -> /test)
    pub name: String,

    /// Aliases for commands
    pub aliases: Vec<&'static str>,

    /// Description of command
    pub description: String,

    /// Logic of command (function)
    pub handler: fn(Context) -> BoxFuture<CommandResponse>,
}

impl PartialEq for Command {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name &&
        self.aliases == other.aliases &&
        self.description == other.description
    }
}

impl Eq for Command {}

impl std::hash::Hash for Command {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.aliases.hash(state);
        self.description.hash(state);
    }
}
