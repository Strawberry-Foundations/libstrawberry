use crate::notifications::Notifier;

pub mod linux;
pub mod windows;

pub trait BaseNotifier {
    fn new(notifier: Notifier) -> Self;
    fn notification_send(&self) -> bool;
}