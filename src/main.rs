use env_logger::Env;
use log::info; // Import the log info macro

mod servicehelper;
use servicehelper::ServiceHelper;

#[tokio::main]
async fn main() {
    // Initialize logger using env_logger
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    // Create an instance of ServiceHelper
    let service_helper = ServiceHelper;

    // Run the service check timer
    service_helper.run_service_check_timer().await;
}
