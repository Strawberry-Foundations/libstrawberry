use std::process::Command;

use dbus::{Connection, Message, BusType, MessageItem};

use crate::notifications::Notifier;
use crate::notifications::notifiers::BaseNotifier;

pub struct LinuxLibNotifyNotifier {
    pub notifier: Notifier
}

pub struct LinuxDBusNotifier {
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
                    false
                } else {
                    true
                }
            }
            Err(err) => {
                println!("Error sending notification: {}", err);
                false
            }
        }
    }
}


impl BaseNotifier for LinuxDBusNotifier {
    fn new(notifier: Notifier) -> Self {
        Self {
            notifier
        }
    }

    fn notification_send(&self) -> bool {
        let c = Connection::get_private(BusType::Session).unwrap();

        let mut m = Message::new_method_call(
            "org.freedesktop.Notifications",
            "/org/freedesktop/Notifications",
            "org.freedesktop.Notifications",
            "Notify"
        ).unwrap();

        m.append_items(&[
            MessageItem::Str(self.notifier.application_name.to_string()),
            MessageItem::UInt32(0),
            MessageItem::Str(self.notifier.icon.to_string()),
            MessageItem::Str(self.notifier.title.to_string()),
            MessageItem::Str(self.notifier.message.to_string()),
            MessageItem::new_array(vec![ MessageItem::Str("".to_string())]).unwrap(),
            MessageItem::new_array(vec![MessageItem::DictEntry(
                Box::new(MessageItem::Str("".to_string())),
                Box::new(MessageItem::Variant(
                    Box::new(MessageItem::Str("".to_string()))
                ))
            )]).unwrap(),
            MessageItem::Int32(5000),
        ]);

        c.send(m).unwrap();

        true
    }
}