use std::any::type_name;

/// Get the type of a variable
#[must_use]
pub fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}
