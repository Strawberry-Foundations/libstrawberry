#![cfg(target_os = "linux")]

use std::process::Command;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::thread;
use dbus::arg::RefArg;
use dbus::blocking::Connection;

use crate::notifications::Notifier;
use crate::notifications::notifiers::BaseNotifier;

pub struct LinuxLibNotifyNotifier {
    pub notifier: Notifier
}

impl BaseNotifier for LinuxLibNotifyNotifier {
    fn new(notifier: Notifier) -> Self {
        Self {
            notifier
        }
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
            urgency.as_str()
        ];

        if !self.notifier.icon.is_empty() {
            generated_command.push(icon.as_str());
        }

        match Command::new("notify-send").args(generated_command.clone()).output() {
            Ok(output) => {
                if !output.status.success() {
                    println!("Failed to send notification: {:?}", output);
                    println!("{}", generated_command.join(" "));
                    return false
                }
                true
            }
            Err(err) => {
                println!("Error sending notification: {}", err);
                false
            }
        }
    }
}


pub struct LinuxDBusNotifier {
    pub notifier: Notifier
}

impl BaseNotifier for LinuxDBusNotifier {
    fn new(notifier: Notifier) -> Self {
        Self {
            notifier
        }
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
}

impl LinuxDBusNotifier {
    pub fn with_custom_actions(notifier: Notifier, action_pairs: Vec<(String, String)>) -> Self {
        let mut actions = Vec::new();
        for (id, label) in action_pairs {
            actions.push(id);
            actions.push(label);
        }
        
        Self { notifier }
    }

    pub fn send_with_actions_and_wait(&self, custom_actions: Vec<(String, String)>) -> Result<Option<String>, String> {
        let conn = Connection::new_session()
            .map_err(|e| format!("Failed to connect to D-Bus: {}", e))?;

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

        let (notification_id,): (u32,) = proxy.method_call(
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
                self.notifier.timeout
            ),
        ).map_err(|e| format!("Failed to send notification: {}", e))?;

        println!("Notification sent with ID: {}", notification_id);

        let result = Arc::new(Mutex::new(None::<String>));
        let result_clone = Arc::clone(&result);
        let result_clone2 = Arc::clone(&result);

        let conn_clone = Connection::new_session()
            .map_err(|e| format!("Failed to create second D-Bus connection: {}", e))?;

        let _token1 = conn_clone.add_match(
            dbus::message::MatchRule::new_signal("org.freedesktop.Notifications", "ActionInvoked"),
            move |_: (), _, msg| {
                if let Ok((nid, action_key)) = msg.read2::<u32, String>() {
                    if nid == notification_id {
                        println!("Action '{}' invoked on notification {}", action_key, nid);
                        if let Ok(mut result_guard) = result_clone.lock() {
                            *result_guard = Some(action_key);
                        }
                    }
                }
                true
            },
        ).map_err(|e| format!("Failed to add ActionInvoked match rule: {}", e))?;

        let _token2 = conn_clone.add_match(
            dbus::message::MatchRule::new_signal("org.freedesktop.Notifications", "NotificationClosed"),
            move |_: (), _, msg| {
                if let Ok((nid, reason)) = msg.read2::<u32, u32>() {
                    if nid == notification_id {
                        println!("Notification {} closed (reason: {})", nid, reason);
                        if let Ok(mut result_guard) = result_clone2.lock() {
                            if result_guard.is_none() {
                                *result_guard = Some("__CLOSED__".to_string());
                            }
                        }
                    }
                }
                true
            },
        ).map_err(|e| format!("Failed to add NotificationClosed match rule: {}", e))?;

        let start_time = std::time::Instant::now();
        let timeout_duration = Duration::from_secs(30);

        while start_time.elapsed() < timeout_duration {
            if let Err(e) = conn_clone.process(Duration::from_millis(100)) {
                println!("Error processing D-Bus messages: {}", e);
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
}
