#![cfg(target_os = "linux")]

use dbus::arg::RefArg;
use dbus::arg::Variant;
use dbus::blocking::Connection;
use std::collections::HashMap;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::notifications::Notifier;
use crate::notifications::notifiers::BaseNotifier;

// Legacy notifier for Linux using `notify-send`
pub struct LinuxLibNotifyNotifier {
    pub notifier: Notifier,
}

impl BaseNotifier for LinuxLibNotifyNotifier {
    fn new(notifier: Notifier) -> Self {
        Self { notifier }
    }

    fn notification_send(&self) -> bool {
        let app_name = format!("--app-name={}", self.notifier.application_name);
        let urgency = format!("--urgency={}", self.notifier.urgency);
        let title = format!("{}", self.notifier.title.as_str());
        let message = format!("{}", self.notifier.message.as_str());

        let icon_path = shellwords::escape(&self.notifier.icon);
        let icon = format!("--icon={icon_path}");

        let mut generated_command = vec![
            app_name.as_str(),
            title.as_str(),
            message.as_str(),
            urgency.as_str(),
        ];

        if !self.notifier.icon.is_empty() {
            generated_command.push(icon.as_str());
        }

        match Command::new("notify-send")
            .args(generated_command.clone())
            .output()
        {
            Ok(output) => {
                if !output.status.success() {
                    println!("Failed to send notification: {:?}", output);
                    println!("{}", generated_command.join(" "));
                    return false;
                }
                true
            }
            Err(err) => {
                println!("Error sending notification: {}", err);
                false
            }
        }
    }

    fn send_with_actions_and_wait(
        &self,
        custom_actions: Vec<(String, String)>,
    ) -> Result<Option<String>, String> {
        let app_name = format!("--app-name={}", self.notifier.application_name);
        let urgency = format!("--urgency={}", self.notifier.urgency);
        let title = format!("{}", self.notifier.title.as_str());
        let message = format!("{}", self.notifier.message.as_str());

        let icon_path = shellwords::escape(&self.notifier.icon);
        let icon = format!("--icon={icon_path}");

        let mut generated_command = vec![app_name, title, message, urgency, "--wait".to_string()];

        if !self.notifier.icon.is_empty() {
            generated_command.push(icon);
        }

        for (name, text) in &custom_actions {
            if name.is_empty() {
                let action = format!("--action={}", text);
                generated_command.push(action);
            } else {
                let action = format!("--action={}={}", name, text);
                generated_command.push(action);
            }
        }

        let output = match Command::new("notify-send")
            .args(&generated_command)
            .output()
        {
            Ok(output) => output,
            Err(err) => {
                println!("Error sending notification with actions: {}", err);
                return Err(format!("Error: {}", err));
            }
        };

        if !output.status.success() {
            println!("Failed to send notification with actions: {:?}", output);
            println!("{}", generated_command.join(" "));
            return Err("notify-send failed".to_string());
        }

        // The selected action name is printed to stdout by notify-send
        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if stdout.is_empty() {
            Ok(None)
        } else {
            Ok(Some(stdout))
        }
    }

    fn show_progress(&self, _progress: u32, _message: &str) -> Result<u32, String> {
        eprintln!("[UNSUPPORTED]: show_progress() is not compatible with LinuxLibNotifyNotifier");
        Err("show_progress() is not supported".into())
    }

    fn update_progress(
        &self,
        _notification_id: u32,
        _progress: u32,
        _message: &str,
    ) -> Result<(), String> {
        eprintln!("[UNSUPPORTED]: update_progress() is not compatible with LinuxLibNotifyNotifier");
        Err("update_progress() is not supported".into())
    }

    fn stream_progress<F>(
        &self,
        _start: u32,
        _end: u32,
        _message: &str,
        _callback: F,
    ) -> Result<(), String>
    where
        F: FnMut(u32) -> bool,
    {
        eprintln!("[UNSUPPORTED]: stream_progress() is not compatible with LinuxLibNotifyNotifier");
        Err("stream_progress() is not supported".into())
    }
}

// Notifier for Linux using modern D-Bus connections (but way more complex :skull:)
pub struct LinuxDBusNotifier {
    pub notifier: Notifier,
}

impl BaseNotifier for LinuxDBusNotifier {
    fn new(notifier: Notifier) -> Self {
        Self { notifier }
    }

    fn notification_send(&self) -> bool {
        let conn = match Connection::new_session() {
            Ok(c) => c,
            Err(err) => {
                println!("Error connecting to D-Bus: {}", err);
                return false;
            }
        };

        let proxy = conn.with_proxy(
            "org.freedesktop.Notifications",
            "/org/freedesktop/Notifications",
            Duration::from_millis(5000),
        );

        let actions: Vec<String> = Vec::new();

        let (id,): (u32,) = match proxy.method_call(
            "org.freedesktop.Notifications",
            "Notify",
            (
                &self.notifier.application_name,
                0u32,
                &self.notifier.icon,
                &self.notifier.title,
                &self.notifier.message,
                actions,
                std::collections::HashMap::<&str, dbus::arg::Variant<Box<dyn dbus::arg::RefArg>>>::new(),
                self.notifier.timeout
            ),
        ) {
            Ok(id) => id,
            Err(err) => {
                println!("Error sending notification via D-Bus: {}", err);
                return false;
            }
        };

        println!("Notification sent with ID: {}", id);

        true
    }

    fn send_with_actions_and_wait(
        &self,
        custom_actions: Vec<(String, String)>,
    ) -> Result<Option<String>, String> {
        let conn =
            Connection::new_session().map_err(|e| format!("Failed to connect to D-Bus: {}", e))?;

        let proxy = conn.with_proxy(
            "org.freedesktop.Notifications",
            "/org/freedesktop/Notifications",
            Duration::from_millis(5000),
        );

        let mut actions = Vec::new();
        for (id, label) in custom_actions {
            actions.push(id);
            actions.push(label);
        }

        let (notification_id,): (u32,) = proxy
            .method_call(
                "org.freedesktop.Notifications",
                "Notify",
                (
                    &self.notifier.application_name,
                    0u32,
                    &self.notifier.icon,
                    &self.notifier.title,
                    &self.notifier.message,
                    actions,
                    std::collections::HashMap::<&str, dbus::arg::Variant<Box<dyn RefArg>>>::new(),
                    self.notifier.timeout,
                ),
            )
            .map_err(|e| format!("Failed to send notification: {}", e))?;

        if self.notifier.enable_logging {
            eprintln!("Notification sent with ID: {}", notification_id);
        }

        let result = Arc::new(Mutex::new(None::<String>));
        let result_clone = Arc::clone(&result);
        let result_clone2 = Arc::clone(&result);

        let conn_clone = Connection::new_session()
            .map_err(|e| format!("Failed to create second D-Bus connection: {}", e))?;

        let _token1 = conn_clone
            .add_match(
                dbus::message::MatchRule::new_signal(
                    "org.freedesktop.Notifications",
                    "ActionInvoked",
                ),
                {
                    let enable_logging = self.notifier.enable_logging;
                    move |_: (), _, msg| {
                        if let Ok((nid, action_key)) = msg.read2::<u32, String>() {
                            if nid == notification_id {
                                if enable_logging {
                                    eprintln!(
                                        "Action '{}' invoked on notification {}",
                                        action_key, nid
                                    );
                                }
                                if let Ok(mut result_guard) = result_clone.lock() {
                                    *result_guard = Some(action_key);
                                }
                            }
                        }
                        true
                    }
                },
            )
            .map_err(|e| format!("Failed to add ActionInvoked match rule: {}", e))?;

        let _token2 = conn_clone
            .add_match(
                dbus::message::MatchRule::new_signal(
                    "org.freedesktop.Notifications",
                    "NotificationClosed",
                ),
                {
                    let enable_logging = self.notifier.enable_logging;
                    move |_: (), _, msg| {
                        if let Ok((nid, reason)) = msg.read2::<u32, u32>() {
                            if nid == notification_id {
                                if enable_logging {
                                    eprintln!("Notification {} closed (reason: {})", nid, reason);
                                }
                                if let Ok(mut result_guard) = result_clone2.lock() {
                                    if result_guard.is_none() {
                                        *result_guard = Some("__CLOSED__".to_string());
                                    }
                                }
                            }
                        }
                        true
                    }
                },
            )
            .map_err(|e| format!("Failed to add NotificationClosed match rule: {}", e))?;

        let start_time = std::time::Instant::now();
        let timeout_duration = Duration::from_secs(30);

        while start_time.elapsed() < timeout_duration {
            if let Err(e) = conn_clone.process(Duration::from_millis(100)) {
                if self.notifier.enable_logging {
                    eprintln!("Error processing D-Bus messages: {}", e);
                }
            }

            if let Ok(result_guard) = result.lock() {
                if let Some(ref action) = *result_guard {
                    if action == "__CLOSED__" {
                        return Ok(None);
                    } else {
                        return Ok(Some(action.clone()));
                    }
                }
            }

            thread::sleep(Duration::from_millis(50));
        }

        Ok(None)
    }

    fn show_progress(&self, progress: u32, message: &str) -> Result<u32, String> {
        let conn =
            Connection::new_session().map_err(|e| format!("Failed to connect to D-Bus: {}", e))?;

        let proxy = conn.with_proxy(
            "org.freedesktop.Notifications",
            "/org/freedesktop/Notifications",
            Duration::from_millis(5000),
        );

        // Prepare hints for progress bar
        let mut hints: HashMap<&str, Variant<Box<dyn RefArg>>> = HashMap::new();
        hints.insert("value", Variant(Box::new(progress)));

        let (id,): (u32,) = proxy
            .method_call(
                "org.freedesktop.Notifications",
                "Notify",
                (
                    &self.notifier.application_name,
                    0u32,
                    &self.notifier.icon,
                    &self.notifier.title,
                    message,
                    Vec::<String>::new(),
                    hints,
                    self.notifier.timeout,
                ),
            )
            .map_err(|e| format!("Failed to send notification: {}", e))?;

        Ok(id)
    }

    fn update_progress(
        &self,
        notification_id: u32,
        progress: u32,
        message: &str,
    ) -> Result<(), String> {
        let conn =
            Connection::new_session().map_err(|e| format!("Failed to connect to D-Bus: {}", e))?;

        let proxy = conn.with_proxy(
            "org.freedesktop.Notifications",
            "/org/freedesktop/Notifications",
            Duration::from_millis(5000),
        );

        let mut hints: HashMap<&str, Variant<Box<dyn RefArg>>> = HashMap::new();
        hints.insert("value", Variant(Box::new(progress)));

        proxy
            .method_call::<(), _, _, _>(
                "org.freedesktop.Notifications",
                "Notify",
                (
                    &self.notifier.application_name,
                    notification_id, // replaces_id
                    &self.notifier.icon,
                    &self.notifier.title,
                    message,
                    Vec::<String>::new(),
                    hints,
                    self.notifier.timeout,
                ),
            )
            .map_err(|e| format!("Failed to update notification: {}", e))?;

        Ok(())
    }

    fn stream_progress<F>(
        &self,
        start: u32,
        end: u32,
        message: &str,
        mut callback: F,
    ) -> Result<(), String>
    where
        F: FnMut(u32) -> bool,
    {
        let mut current = start;
        let notification_id = self.show_progress(current, message)?;

        while current <= end {
            if !callback(current) {
                break;
            }
            self.update_progress(notification_id, current, message)?;
            current += 1;
            std::thread::sleep(Duration::from_millis(100));
        }
        Ok(())
    }
}
