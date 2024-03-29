use env_logger::Env;
use std::env;
mod servicehelper;
use servicehelper::ServiceHelper;

#[tokio::main]
async fn main() {
    // Check if the "--print" argument is provided
    let args: Vec<String> = env::args().collect();
    let print_output = args.contains(&String::from("--print"));

    // Initialize logger using env_logger with default filter set to "info"
    let log_level = if print_output { "info" } else { "warn" }; // Adjust log level based on "--print" argument
    env_logger::Builder::from_env(Env::default().default_filter_or(log_level)).init();

    // Create an instance of ServiceHelper
    let mut service_helper = ServiceHelper {
        osquery_prev_status: String::new(),
        wazuh_prev_status: String::new(),
        clamav_prev_status: String::new(),
    };

    // Run the service check timer asynchronously
    service_helper.run_service_check_timer().await;
}
