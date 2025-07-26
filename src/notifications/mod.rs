pub mod notifiers;
pub mod os;

use crate::colors::{C_RESET, RED};
use crate::notifications::notifiers::BaseNotifier;
use crate::notifications::os::OS;

#[cfg(target_os = "linux")]
use crate::notifications::notifiers::linux::{LinuxDBusNotifier, LinuxLibNotifyNotifier};
#[cfg(target_os = "windows")]
use crate::notifications::notifiers::windows::{WindowsLegacyNotifier, WindowsNotifier};

/// Main notification structure that handles cross-platform notifications
///
/// # Examples
///
/// ```
/// use stblib::notifications::Notifier;
/// let notification = Notifier::new(
///     "Title",
///     "Message",
///     "App Name",
///     "normal",
///     "",
///     None,
///     5000,
///     false,
/// );
/// notification.build().send()?;
/// ```
#[derive(Debug, Clone)]
pub struct Notifier {
    pub title: String,
    pub message: String,
    pub application_name: String,
    pub urgency: String,
    pub icon: String,
    pub audio: Option<String>,
    pub timeout: i32,
    pub enable_logging: bool,
    pub platform_notifier: PlatformSpecificNotifier,
    pub actions: Vec<(String, String)>,
}

/// Internal structure to handle OS-specific notification settings
#[derive(Debug, Clone)]
pub struct PlatformSpecificNotifier {
    pub os: &'static OS,
    pub bypass_windows_version_detection: bool,
    pub windows_version_override: Option<String>,
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
            5000,
            false,
        )
    }
}

impl Notifier {
    /// Creates a new notification with the specified parameters
    ///
    /// # Arguments
    ///
    /// * `title` - The title of the notification
    /// * `message` - The message content
    /// * `application_name` - The application name
    /// * `urgency` - Urgency level ("low", "normal", "critical")
    /// * `icon_path` - Path to the notification icon
    /// * `audio_path` - Optional audio file to play
    /// * `timeout` - Duration in milliseconds before the notification closes
    /// * `enable_logging` - Enable debug logging
    #[must_use]
    pub fn new(
        title: impl ToString,
        message: impl ToString,
        application_name: impl ToString,
        urgency: impl ToString,
        icon_path: impl ToString,
        audio_path: Option<String>,
        timeout: i32,
        enable_logging: bool,
    ) -> Self {
        Self {
            title: title.to_string(),
            message: message.to_string(),
            application_name: application_name.to_string(),
            urgency: urgency.to_string(),
            icon: icon_path.to_string(),
            audio: audio_path,
            timeout,
            enable_logging,
            platform_notifier: PlatformSpecificNotifier {
                os: &OS::Undefined,
                bypass_windows_version_detection: false,
                windows_version_override: None,
            },
            actions: Vec::new(),
        }
    }

    /// Builds the notification by detecting the current operating system
    ///
    /// This method determines the appropriate notification system based on
    /// the current OS and version.
    pub fn build(mut self) -> Self {
        if self.platform_notifier.os == &OS::Undefined {
            self.platform_notifier.os = match std::env::consts::OS {
                "linux" => &OS::Linux,
                "windows" => {
                    let version = os_info::get().version().to_string();
                    let version: Vec<&str> = version.split(".").collect();
                    let version = *version.first().unwrap();

                    if version == "10" || version == "11" {
                        &OS::Windows
                    } else {
                        &OS::WindowsLegacy
                    }
                }
                "macos" => &OS::MacOS,
                _ => &OS::Unknown,
            }
        }

        self
    }

    /// Sends the notification using the appropriate system notifier
    ///
    /// # Errors
    ///
    /// Returns `NotificationError` if:
    /// * The operating system is not supported
    /// * The notification fails to send
    pub fn send(self) -> Result<bool, NotificationError> {
        if self.enable_logging {
            println!("Sending notification");
        }

        match self.platform_notifier.os {
            #[cfg(target_os = "linux")]
            OS::Linux => Ok(LinuxDBusNotifier::new(self).notification_send()),

            #[cfg(target_os = "linux")]
            OS::LinuxLibNotify => Ok(LinuxLibNotifyNotifier::new(self).notification_send()),

            #[cfg(target_os = "windows")]
            OS::Windows => Ok(WindowsNotifier::new(self).notification_send()),

            #[cfg(target_os = "windows")]
            OS::WindowsLegacy => Ok(WindowsLegacyNotifier::new(self).notification_send()),

            OS::Undefined => {
                let err = NotificationError::UndefinedOS;
                if self.enable_logging {
                    eprintln!("{RED}{}{C_RESET}", err);
                }
                Err(err)
            }
            _ => {
                let err = NotificationError::UnsupportedOS;
                if self.enable_logging {
                    eprintln!("{RED}{}{C_RESET}", err);
                }
                Err(err)
            }
        }
    }

    /// Creates a custom notification from an existing configuration
    ///
    /// # Arguments
    ///
    /// * `notifier` - An existing Notifier instance to clone
    #[must_use]
    pub const fn custom(notifier: Self) -> Self {
        notifier
    }

    /// Adds action buttons to the notification
    ///
    /// # Arguments
    ///
    /// * `actions` - Vector of (id, label) pairs for the buttons
    ///
    /// # Example
    ///
    /// ```
    /// use stblib::notifications::Notifier;
    /// let notifier = Notifier::default()
    ///     .with_actions(vec![
    ///         ("yes".to_string(), "Yes".to_string()),
    ///         ("no".to_string(), "No".to_string())
    ///     ]);
    /// ```
    pub fn with_actions(mut self, actions: Vec<(String, String)>) -> Self {
        self.actions = actions;
        self
    }

    /// Sends the notification with custom actions and waits for user response
    ///
    /// # Returns
    ///
    /// * `Ok(Some(String))` - The ID of the action that was clicked
    /// * `Ok(None)` - Notification was closed without clicking any action
    /// * `Err(NotificationError)` - If the notification fails
    ///
    /// # Example
    ///
    /// ```
    /// use stblib::notifications::Notifier;
    /// let notifier = Notifier::default()
    ///     .with_actions(vec![
    ///         ("yes".to_string(), "Yes".to_string()),
    ///         ("no".to_string(), "No".to_string())
    ///     ]);
    /// let actions = vec![("yes".to_string(), "Yes".to_string()), ("no".to_string(), "No".to_string())];
    /// match notifier.send_with_actions_and_wait(actions) {
    ///     Ok(Some(action)) => println!("User clicked: {}", action),
    ///     Ok(None) => println!("Notification was closed"),
    ///     Err(e) => println!("Error: {:?}", e),
    /// }
    /// ```
    pub fn send_with_actions_and_wait(
        self,
        actions: Vec<(String, String)>,
    ) -> Result<Option<String>, NotificationError> {
        // Currently only Linux (DBus) supports actions
        match self.platform_notifier.os {
            #[cfg(target_os = "linux")]
            OS::Linux => {
                let dbus_notifier = LinuxDBusNotifier::new(self);
                dbus_notifier
                    .send_with_actions_and_wait(actions)
                    .map_err(|e| NotificationError::SendError(e))
            }
            _ => {
                self.send()?;
                Ok(None)
            }
        }
    }
}

/// Errors that can occur when sending notifications
#[derive(Debug, thiserror::Error)]
pub enum NotificationError {
    #[error("Unsupported operating system")]
    UnsupportedOS,
    #[error("Undefined operating system. Please run `Notifier::build()` first")]
    UndefinedOS,
    #[error("Failed to send notification: {0}")]
    SendError(String),
}
