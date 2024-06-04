use env_logger::{Builder, Env};
use std::env;
use tokio;
use std::sync::Arc;
use tokio::sync::Mutex;

mod server;
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

    let service_helper = Arc::new(Mutex::new(ServiceHelper {
        osquery_prev_status: String::new(),
        wazuh_prev_status: String::new(),
        clamav_prev_status: String::new(),
    }));

    let service_helper_clone = service_helper.clone();
    let server_task = tokio::spawn(server::run_server(service_helper_clone));
    let service_task = tokio::spawn(async move {
        service_helper.lock().await.run_service_check_timer().await;
    });

    let _ = tokio::join!(server_task, service_task);
}
