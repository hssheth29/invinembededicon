extern crate notify_rust;
extern crate gtk;

use notify_rust::Notification;
use std::thread;
use std::time::Duration;

use tray_item::{IconSource, TrayItem};

fn main() {
    gtk::init().unwrap();

    let mut tray = TrayItem::new("invinsense", IconSource::Resource("security-high-symbolic")).unwrap();

    tray.add_label("Tray Label").unwrap();

    tray.add_menu_item("Hello", || {
        println!("Hello!");
    }).unwrap();

    tray.add_menu_item("Quit", || {
        gtk::main_quit();
    }).unwrap();

    // Spawn a thread to send notifications
    thread::spawn(|| {
        send_notifications();
    });

    gtk::main();
}

fn send_notifications() {
    loop {
        // Send notification
        send_notification("Hello from Rust!");

        // Sleep for one second
        thread::sleep(Duration::from_secs(1));
    }
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
