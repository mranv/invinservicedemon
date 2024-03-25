mod servicehelper;
use servicehelper::ServiceHelper;

#[tokio::main]
async fn main() {
    let service_helper = ServiceHelper;
    service_helper.run_service_check_timer().await;
}
