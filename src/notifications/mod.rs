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
#[derive(Debug)]
pub struct Notifier {
    pub title: String,
    pub message: String,
    pub application_name: String,
    pub urgency: String,
    pub icon: String,
    pub audio: Option<String>,
    pub timeout: i32,
    pub enable_logging: bool,
    pub internal_notifier: InternalNotifierObject,
    pub actions: Vec<(String, String)>,
}

/// Internal structure to handle OS-specific notification settings
#[derive(Debug)]
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
    /// * `default_notification_title` - The title of the notification
    /// * `default_notification_message` - The message content
    /// * `default_notification_application_name` - The application name
    /// * `default_notification_urgency` - Urgency level ("low", "normal", "critical")
    /// * `default_notification_icon` - Path to the notification icon
    /// * `default_notification_audio` - Optional audio file to play
    /// * `enable_logging` - Enable debug logging
    #[must_use]
    pub fn new(
        default_notification_title: impl ToString,
        default_notification_message: impl ToString,
        default_notification_application_name: impl ToString,
        default_notification_urgency: impl ToString,
        default_notification_icon: impl ToString,
        default_notification_audio: Option<String>,
        default_timeout: i32,
        enable_logging: bool,
    ) -> Self {
        Self {
            title: default_notification_title.to_string(),
            message: default_notification_message.to_string(),
            application_name: default_notification_application_name.to_string(),
            urgency: default_notification_urgency.to_string(),
            icon: default_notification_icon.to_string(),
            audio: default_notification_audio,
            timeout: default_timeout,
            enable_logging,
            internal_notifier: InternalNotifierObject {
                system: &OS::Undefined,
                override_windows_version_detection: false,
                override_windows_version: None,
            },
            actions: Vec::new(),
        }
    }

    /// Builds the notification by detecting the current operating system
    /// 
    /// This method determines the appropriate notification system based on
    /// the current OS and version.
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

        match self.internal_notifier.system {
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
    pub fn send_with_actions_and_wait(self, actions: Vec<(String, String)>) -> Result<Option<String>, NotificationError> {
        match self.internal_notifier.system {
            #[cfg(target_os = "linux")]
            OS::Linux => {
                let dbus_notifier = LinuxDBusNotifier::new(self);
                dbus_notifier.send_with_actions_and_wait(actions)
                    .map_err(|e| NotificationError::SendError(e))
            },
            _ => {
                // For other platforms, send normally and return None
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
