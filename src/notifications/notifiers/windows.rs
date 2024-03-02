use crate::notifications::Notifier;
use crate::notifications::notifiers::BaseNotifier;

pub struct WindowsNotifier {
    pub notifier: Notifier
}

impl BaseNotifier for WindowsNotifier {
    fn new(notifier: Notifier) -> Self {
        Self {
            notifier
        }
    }

    fn notification_send(&self) -> bool {
        todo!()
    }
}


pub struct WindowsLegacyNotifier {
    pub notifier: Notifier
}

impl BaseNotifier for WindowsLegacyNotifier {
    fn new(notifier: Notifier) -> Self {
        Self {
            notifier
        }
    }

    fn notification_send(&self) -> bool {
        todo!()
    }
}