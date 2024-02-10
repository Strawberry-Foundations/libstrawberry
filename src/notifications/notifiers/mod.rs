pub mod linux;

pub trait NotifierBackend {
    fn new(&self,) -> Self ;
    fn notification_send(&self);
}

pub struct NotifierBuilder<T> {
    pub backend: T
}

impl<T: NotifierBackend> NotifierBuilder<T> {
    pub fn from(backend: T) -> NotifierBuilder<T> {
        NotifierBuilder {
            backend
        }
    }
}