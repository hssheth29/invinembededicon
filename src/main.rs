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

    // Define hardcoded menu items with service names and status codes
    let menu_items = vec![
        ("osquery", 0),
        ("Wazuh", 2),
        ("ClamAV", 1),
    ];

    for (service_name, status) in menu_items.iter() {
        // Determine the emoji based on the status code
        let emoji = match status {
            0 => "🟢", // Green for operational
            1 => "🔴", // Red for error
            2 => "🟠", // Orange for warning or attention
            _ => "⚪", // Fallback to white if an unexpected status code is encountered
        };

        // Create menu text without the description, only the service name and status code
        let menu_text = format!("{} {} - Status: {}", emoji, service_name, status);
        let service_name_clone = service_name.to_string();
        let status_clone = *status;

        if tray.add_menu_item(&menu_text, move || {
            send_dynamic_notification(&service_name_clone, status_clone);
        }).is_err() {
            eprintln!("Failed to add dynamic menu item.");
        }
    }

    gtk::main();
}

fn send_dynamic_notification(service_name: &str, status: i32) {
    // Determine the message based on the status code
    let message = match status {
        0 => format!("{} is healthy.", service_name),
        1 => format!("{} is not installed.", service_name),
        2 => format!("{} is installed but not running.", service_name),
        _ => format!("{} status is unknown.", service_name), // Fallback message
    };

    // Determine the icon based on the status code
    let icon_name = match status {
        0 => "security-low-symbolic",
        1 => "dialog-error-symbolic",
        2 => "dialog-warning-symbolic",
        _ => "dialog-information", // Fallback icon
    };

    let notification = Notification::new()
        .summary(service_name)
        .body(&message)
        .icon(icon_name)
        .finalize();

    if notification.show().is_err() {
        eprintln!("Error showing notification.");
    }
}
