mod servicehelper;
use servicehelper::ServiceHelper;

use env_logger::Env;
use std::env;
use zbus::connection;

#[tokio::main]
async fn main() {
    // Check if the "--print" argument is provided
    let args: Vec<String> = env::args().collect();
    let print_output = args.contains(&String::from("--print"));

    // Initialize logger using env_logger with default filter set to "info"
    let log_level = if print_output { "info" } else { "warn" }; // Adjust log level based on "--print" argument
    env_logger::Builder::from_env(Env::default().default_filter_or(log_level)).init();

    // Create an instance of ServiceHelper
    let service_helper = ServiceHelper;

    // Run the service check timer asynchronously
    let dbus_server = run_dbus_server(service_helper).await;
    if let Err(e) = dbus_server {
        eprintln!("Error: {}", e);
    }
}

async fn run_dbus_server(service_helper: ServiceHelper) -> zbus::Result<()> {
    let greeter = Greeter { service_helper };

    let _conn = connection::Builder::session()?
        .name("com.atcults.anubhav")?
        .build()
        .await?;

        let _object_server = _conn.object_server().at("/com/atcults/anubhav/Greeter", greeter).await?;
    let mut h = _conn.request_name("com.atcults.anubhav").await?;

    Ok(())
}

struct Greeter {
    service_helper: ServiceHelper,
}

#[zbus::interface(name = "com.atcults.anubhav")]
impl Greeter {
    async fn get_menu_items(&self) -> zbus::fdo::Result<String> {
        let menu_items_data = self.service_helper.get_menu_item_data().await;
        Ok(menu_items_data.to_string())
    }
}