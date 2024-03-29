extern crate notify_rust;
extern crate gtk;
use notify_rust::Notification;
use tray_item::{IconSource, TrayItem};

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

    // Define hardcoded menu items
    let menu_items = vec![
        ("User Behavior Analytics", "osquery is installed and halted.", 1),
        ("Endpoint Detection and Response", "Wazuh is not installed and halted.", 0),
        ("End-Point Protection", "ClamAV is installed and stopped.", 2),
    ];

    for (text, description, status) in menu_items.iter() {
        // Determine the emoji based on the status code
        let emoji = match status {
            0 => "🟢", // Green for operational
            1 => "🔴", // Red for error
            2 => "🟠", // Orange for warning or attention
            _ => "⚪", // Fallback to white if an unexpected status code is encountered
        };

        let menu_text = format!("{} {} - Status: {}", emoji, text, status);
        let text_clone = text.to_string();
        let description_clone = description.to_string();
        let status_clone = *status;

        if tray.add_menu_item(&menu_text, move || {
            send_notification(&text_clone, &description_clone, status_clone);
        }).is_err() {
            eprintln!("Failed to add dynamic menu item.");
        }
    }

    gtk::main();
}

fn send_notification(title: &str, message: &str, status: i32) {
    // Determine the icon based on the status code
    let icon_name = match status {
        0 => "security-low-symbolic",
        1 => "dialog-error-symbolic",
        2 => "dialog-warning-symbolic",
        _ => "dialog-information", // Fallback icon
    };

    let notification = Notification::new()
        .summary(title)
        .body(message)
        .icon(icon_name)
        .finalize();

    if notification.show().is_err() {
        eprintln!("Error showing notification.");
    }
}
