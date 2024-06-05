use env_logger::{Builder, Env};
use std::env;
use tokio;

mod servicehelper;
use servicehelper::ServiceHelper;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let print_output = args.contains(&String::from("--print"));

    let log_level = if print_output { "info" } else { "warn" };

    Builder::from_env(Env::default().default_filter_or(log_level))
        .target(env_logger::Target::default())
        .init();

    let mut service_helper = ServiceHelper;
    service_helper.run_service_check_timer().await;
}
