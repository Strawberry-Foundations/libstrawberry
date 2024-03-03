pub mod os;
pub mod notifiers;

#[cfg(target_os = "unix")]
use crate::notifications::notifiers::linux::{LinuxDBusNotifier, LinuxLibNotifyNotifier};
#[cfg(target_os = "windows")]
use crate::notifications::notifiers::windows::{WindowsLegacyNotifier, WindowsNotifier};

use crate::notifications::notifiers::BaseNotifier;
use crate::notifications::os::OS;

pub struct Notifier {
    pub title: String,
    pub message: String,
    pub application_name: String,
    pub urgency: String,
    pub icon: String,
    pub audio: Option<String>,
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
            "",
            None,
            false
        )
    }
}

impl Notifier {
    #[must_use]
    pub fn new(
        default_notification_title: impl ToString,
        default_notification_message: impl ToString,
        default_notification_application_name: impl ToString,
        default_notification_urgency: impl ToString,
        default_notification_icon: impl ToString,
        default_notification_audio: Option<String>,
        enable_logging: bool,
    ) -> Self {
        Self {
            title: default_notification_title.to_string(),
            message: default_notification_message.to_string(),
            application_name: default_notification_application_name.to_string(),
            urgency: default_notification_urgency.to_string(),
            icon: default_notification_icon.to_string(),
            audio: default_notification_audio,
            enable_logging,
            internal_notifier: InternalNotifierObject {
                system: &OS::Undefined,
                override_windows_version_detection: false,
                override_windows_version: None
            },
        }
    }

    pub fn build(mut self) -> Self {
        if self.internal_notifier.system == &OS::Undefined {
            self.internal_notifier.system = match std::env::consts::OS {
                "linux" => &OS::Linux,
                "windows" => {
                    let version = os_info::get().version().to_string();
                    let version: Vec<&str> = version.split(".").collect();
                    let version = *version.first().unwrap();
                    
                    if version == "10" || version == "11" {
                        &OS::Windows
                    }
                    else {
                        &OS::WindowsLegacy
                    }
                }
                "macos" => &OS::MacOS,
                _ => &OS::Unknown
            }
        }

        self
    }

    pub fn send(self,) -> bool {
        match self.internal_notifier.system {
            #[cfg(target_os = "unix")]
            OS::Linux => LinuxDBusNotifier::new(self).notification_send(),

            #[cfg(target_os = "unix")]
            OS::LinuxLibNotify => LinuxLibNotifyNotifier::new(self).notification_send(),

            #[cfg(target_os = "windows")]
            OS::Windows => WindowsNotifier::new(self).notification_send(),

            #[cfg(target_os = "windows")]
            OS::WindowsLegacy => WindowsLegacyNotifier::new(self).notification_send(),
            _ => {
                eprintln!("Unsupported operating system");
                std::process::exit(1);
            }
        }
    }

    #[must_use]
    pub const fn custom(notifier: Self) -> Self {
        notifier
    }
}