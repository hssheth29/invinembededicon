extern crate notify_rust;
extern crate gtk;

use notify_rust::Notification;
use tray_item::{IconSource, TrayItem};

fn main() {
    gtk::init().expect("Failed to initialize GTK.");

    let mut tray = TrayItem::new("invinembededicon", IconSource::Resource("security-high-symbolic")).expect("Failed to create tray item.");

    tray.add_label("invinembededicon").expect("Failed to add tray label.");

    tray.add_menu_item("Hello", || {
        send_notification("Hello from Rust!");
    }).expect("Failed to add menu item.");

    gtk::main();
}

fn send_notification(message: &str) {
    // Create a new notification
    let notification = Notification::new()
        .summary("Rust Notification")
        .body(message)
        .icon("dialog-information")
        .finalize(); // Use finalize to construct the Notification object

    // Check if the notification is supported
    if let Err(error) = notification.show() {
        eprintln!("Error showing notification: {:?}", error);
    }
}
