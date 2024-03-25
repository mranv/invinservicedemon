use env_logger::{Builder, Env};
// use log::LevelFilter;

mod servicehelper;
use servicehelper::ServiceHelper;

#[tokio::main]
async fn main() {
    Builder::from_env(Env::default().default_filter_or("info")).init(); // Initialize logger using Builder

    let service_helper = ServiceHelper;
    service_helper.run_service_check_timer().await;
}
