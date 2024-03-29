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

    // Hardcoded menu items with unicode symbols
    let menu_items = vec![
        ("🟢   User Behavior Analytics", "osquery is installed and halted.", 1),
        ("🔴   Endpoint Detection and Response", "Wazuh is not installed and halted.", 0),
        ("🔴   End-Point Protection", "ClamAV is installed and stopped.", 0),
    ];

    for (text, description, status) in menu_items.iter() {
        let menu_text = format!("{} - Status: {}", text, status);
        let text_clone = text.to_string();
        let description_clone = description.to_string();

        if tray.add_menu_item(&menu_text, move || {
            send_notification(&text_clone, &description_clone);
        }).is_err() {
            eprintln!("Failed to add dynamic menu item.");
        }
    }

    gtk::main();
}

fn send_notification(title: &str, message: &str) {
    let notification = Notification::new()
        .summary(title)
        .body(message)
        .icon("dialog-information")
        .finalize();

    if notification.show().is_err() {
        eprintln!("Error showing notification.");
    }
}
