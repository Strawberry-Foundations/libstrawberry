use crate::scapi::context::Context;

pub type BoxFuture<T> = std::pin::Pin<Box<dyn std::future::Future<Output = T> + Send>>;
pub type CommandResponse = Result<Option<String>, String>;

#[derive(Hash, PartialEq, Eq, Clone)]
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
