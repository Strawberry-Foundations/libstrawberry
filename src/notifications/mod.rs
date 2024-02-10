mod os;
mod notifiers;

use crate::notifications::notifiers::linux::LinuxNotifier;
use crate::notifications::notifiers::{NotifierBackend, NotifierBuilder};
use crate::notifications::os::OS;
use crate::notifications::os::OS::Linux;

pub struct Notifier {
    pub default_notification_title: String,
    pub default_notification_message: String,
    pub default_notification_application_name: String,
    pub default_notification_urgency: String,
    pub default_notification_icon: Option<String>,
    pub default_notification_audio: Option<String>,
    pub enable_logging: bool,
    pub internal_notifier: InternalNotifierObject,
}

pub struct InternalNotifierObject {
    pub system: &'static OS,
    pub override_windows_version_detection: bool,
    pub override_windows_version: Option<String>,

}

impl Default for Notifier {
    fn default() -> Self {
        Self::new(
            "Default Title".to_string(),
            "Default Message".to_string(),
            "Rust Application".to_string(),
            "normal".to_string(),
            None,
            None,
            false
        )
    }
}

impl Notifier {
    #[must_use]
    pub const fn new(
        default_notification_title: String,
        default_notification_message: String,
        default_notification_application_name: String,
        default_notification_urgency: String,
        default_notification_icon: Option<String>,
        default_notification_audio: Option<String>,
        enable_logging: bool,
    ) -> Self {
        Self {
            default_notification_title,
            default_notification_message,
            default_notification_application_name,
            default_notification_urgency,
            default_notification_icon,
            default_notification_audio,
            enable_logging,
            internal_notifier: InternalNotifierObject {
                system: &OS::Undefined,
                override_windows_version_detection: false,
                override_windows_version: None
            },
        }
    }

    pub fn build(&mut self) {
        self.internal_notifier.system = if self.internal_notifier.system == &OS::Undefined {
            match std::env::consts::OS {
                "linux" => &OS::Linux,
                "windows" => &OS::Windows,
                "macos" => &OS::MacOS,
                _ => &OS::Unknown
            }
        }
        else {
            &self.internal_notifier.system
        };

        Self::selected_notification_system(&self.internal_notifier.system);
    }

    fn selected_notification_system<T: NotifierBackend>(os_override: &OS) -> NotifierBuilder<Box<dyn NotifierBackend>> {
         NotifierBuilder::from(LinuxNotifier::new())

    }

    #[must_use]
    pub const fn custom(notifier: Self) -> Self {
        notifier
    }
}