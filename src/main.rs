use tray_item::{TrayItem, IconSource};

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

    gtk::main();
}
