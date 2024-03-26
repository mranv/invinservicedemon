use env_logger::Env; // Import the env_logger crate

mod servicehelper; // Import the servicehelper module
use servicehelper::ServiceHelper; // Import the ServiceHelper struct from the module

#[tokio::main]
async fn main() {
    // Initialize logger using env_logger with default filter set to "info"
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // Create an instance of ServiceHelper
    let service_helper = ServiceHelper;

    // Run the service check timer asynchronously
    service_helper.run_service_check_timer().await;
}
