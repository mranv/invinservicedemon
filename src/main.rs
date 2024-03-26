use env_logger::Env;
// use gtk::prelude::*;
// use serde_json::Value;
use tray_item::{IconSource, TrayItem};
use servicehelper::ServiceHelper;

mod servicehelper;

#[tokio::main]
async fn main() {
    // Initialize logger using env_logger with default filter set to "info"
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // Create an instance of ServiceHelper
    let service_helper = ServiceHelper;

    // Run the service check timer asynchronously
    let menu_items_data = service_helper.run_service_check_timer().await;

    // Initialize GTK
    gtk::init().expect("Failed to initialize GTK.");

    // Create the system tray icon
    let mut tray = TrayItem::new("Tray Example", IconSource::Resource("accessories-calculator"))
        .expect("Failed to create tray icon.");

    // Add a label to the tray
    tray.add_label("Tray Label").expect("Failed to add label to tray.");

    // Add menu items based on the data obtained from ServiceHelper
    if let Some(menu_items_array) = menu_items_data["menuItems"].as_array() {
        for item in menu_items_array {
            let text = item["text"].as_str().unwrap_or_default();
            let description = item["description"].as_str().unwrap_or_default();
            let description_clone = description.to_string(); // Clone description
            tray.add_menu_item(text, move || {
                println!("{}", description_clone); // Use the cloned description
            }).expect("Failed to add menu item.");
        }
    }

    // Add a "Quit" menu item
    tray.add_menu_item("Quit", move || {
        gtk::main_quit();
    }).expect("Failed to add 'Quit' menu item.");

    // Run the GTK event loop
    gtk::main();
}
