use crate::notifications::Notifier;

pub mod linux;
pub mod windows;

pub trait BaseNotifier {
    fn new(notifier: Notifier) -> Self;
    fn notification_send(&self) -> bool;
    fn send_with_actions_and_wait(
        &self,
        custom_actions: Vec<(String, String)>,
    ) -> Result<Option<String>, String>;
    fn show_progress(&self, progress: u32, message: &str) -> Result<u32, String>;
    fn update_progress(
        &self,
        notification_id: u32,
        progress: u32,
        message: &str,
    ) -> Result<(), String>;
    fn stream_progress<F>(
        &self,
        start: u32,
        end: u32,
        message: &str,
        callback: F,
    ) -> Result<(), String>
    where
        F: FnMut(u32) -> bool;
}
