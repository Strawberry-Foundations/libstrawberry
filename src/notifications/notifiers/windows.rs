use std::process::Command;
use winrt_notification::{Duration, IconCrop, Sound, Toast};

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
        let audio = match &self.notifier.audio {
            None => String::from(r#"<audio silent="true" />"#),
            Some(audio) => format!(r#"<audio src="ms-winsoundevent:Notification.{}" />"#, audio)
        };

        let script = format!(
r#"[Windows.UI.Notifications.ToastNotificationManager, Windows.UI.Notifications, ContentType = WindowsRuntime] | Out-Null
[Windows.UI.Notifications.ToastNotification, Windows.UI.Notifications, ContentType = WindowsRuntime] | Out-Null
[Windows.Data.Xml.Dom.XmlDocument, Windows.Data.Xml.Dom.XmlDocument, ContentType = WindowsRuntime] | Out-Null

$APP_ID = "{}"

$template = @"
<toast duration="short">{}<visual><binding template="ToastImageAndText02"><image id="1" src="{}" /><text id="1">{}</text><text id="2">{}</text></binding></visual></toast>
"@

$xml = New-Object Windows.Data.Xml.Dom.XmlDocument
$xml.LoadXml($template)
$toast = New-Object Windows.UI.Notifications.ToastNotification $xml
[Windows.UI.Notifications.ToastNotificationManager]::CreateToastNotifier($APP_ID).Show($toast)"#,
            self.notifier.application_name,
            audio,
            self.notifier.icon,
            self.notifier.title,
            self.notifier.message,
        );

        let output = Command::new("powershell")
            .args(&["-Command", &script])
            .output()
            .expect("Failed to execute PowerShell script");

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!("Error executing PowerShell script: {}", stderr);
            return false
        }

        true
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
        Toast::new(Toast::POWERSHELL_APP_ID)
            .title(&self.notifier.title)
            .icon(&self.notifier.icon.as_ref(), IconCrop::Square, "Logo")
            .text1(&self.notifier.message)
            .sound(Some(Sound::SMS))
            .duration(Duration::Short)
            .show()
            .expect("unable to toast");

        true
    }
}