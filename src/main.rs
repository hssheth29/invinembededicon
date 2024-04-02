extern crate notify_rust;
extern crate gtk;

use notify_rust::Notification;
use tray_item::{IconSource, TrayItem};
use std::{thread, time};

fn main() {
    if gtk::init().is_err() {
        eprintln!("Failed to initialize GTK.");
        return;
    }

    let mut tray = match TrayItem::new("invinembededicon", IconSource::Resource("security-high-symbolic")) {
        Ok(tray) => tray,
        Err(e) => {
            eprintln!("Failed to create tray item: {:?}", e);
            return;
        }
    };

    // Define hardcoded menu items with service names, technical names, and status codes
    let menu_items = vec![
        ("User Behavior Analytics", "osquery", 1),
        ("Endpoint Detection and Response", "Wazuh", 0),
        ("End-Point Protection", "ClamAV", 2),
    ];

    for (user_friendly_name, technical_name, status) in &menu_items {
        let emoji = match status {
            0 => "🟢  ", // Green for operational
            1 => "🔴  ", // Red for error
            2 => "🟡  ", // Orange for warning or attention
            _ => "⚪  ", // Fallback to white
        };

        let menu_text = format!("{} {}", emoji, user_friendly_name);
        let user_friendly_name_clone = user_friendly_name.to_string();
        let technical_name_clone = technical_name.to_string();
        let status_clone = *status;

        if tray.add_menu_item(&menu_text, move || {
            send_notification(&user_friendly_name_clone, &technical_name_clone, status_clone);
        }).is_err() {
            eprintln!("Failed to add dynamic menu item.");
        }
    }

    // Start the notification thread
    let notification_interval = time::Duration::from_secs(10); // Check every 10 seconds
    let cloned_menu_items = menu_items.clone();
    thread::spawn(move || {
        let mut last_notification_times = vec![time::Instant::now(); cloned_menu_items.len()];

        loop {
            for (index, (user_friendly_name, technical_name, status)) in cloned_menu_items.iter().enumerate() {
                let interval = match status {
                    1 => time::Duration::from_secs(60),
                    2 => time::Duration::from_secs(100),
                    _ => continue, // No automatic notification for status 0
                };

                // Check if enough time has elapsed since the last notification
                if last_notification_times[index].elapsed() >= interval {
                    send_notification(user_friendly_name, technical_name, *status);
                    last_notification_times[index] = time::Instant::now();
                }
            }
            thread::sleep(notification_interval);
        }
    });

    gtk::main();
}

fn send_notification(user_friendly_name: &str, technical_name: &str, status: i32) {
    let message = match status {
        0 => format!("{} is healthy.", technical_name),
        1 => format!("{} is not installed.", technical_name),
        2 => format!("{} is installed but not running.", technical_name),
        _ => format!("{} status is unknown.", technical_name),
    };

    let icon_name = match status {
        0 => "security-low-symbolic",
        1 => "dialog-error-symbolic",
        2 => "dialog-warning-symbolic",
        _ => "dialog-information",
    };

    let notification = Notification::new()
        .summary(user_friendly_name)
        .body(&message)
        .icon(icon_name)
        .finalize();

    if notification.show().is_err() {
        eprintln!("Error showing notification.");
    }
}
