use std::process::Command;
use serde_json::Value;
use log::info;
use tokio::sync::mpsc::{self, Receiver};
use tokio::time::{interval, Duration};
use serde_yaml; // Required for YAML serialization

pub struct ServiceHelper {
    pub osquery_prev_status: String,
    pub wazuh_prev_status: String,
    pub clamav_prev_status: String,
}

impl ServiceHelper {
    pub async fn run_service_check_timer(&mut self) {
        let (tx, mut rx): (tokio::sync::mpsc::Sender<Value>, Receiver<Value>) = mpsc::channel(100);

        tokio::spawn(async move {
            let mut osquery_prev_status = String::new();
            let mut wazuh_prev_status = String::new();
            let mut clamav_prev_status = String::new();

            let mut interval = interval(Duration::from_secs(10));

            loop {
                let osquery_status = get_service_status("osqueryd").await;
                let wazuh_status = get_service_status("wazuh-agentd").await;
                let clamav_status = get_service_status("clamav-clamonacc").await;

                if osquery_status != osquery_prev_status ||
                   wazuh_status != wazuh_prev_status ||
                   clamav_status != clamav_prev_status {

                    osquery_prev_status = osquery_status.clone();
                    wazuh_prev_status = wazuh_status.clone();
                    clamav_prev_status = clamav_status.clone();

                    let menu_item_data = get_menu_item_data(
                        true, // Assuming services are installed for simplicity; replace with actual checks
                        true,
                        true,
                        osquery_status,
                        wazuh_status,
                        clamav_status,
                    ).await;

                    if let Err(e) = tx.send(menu_item_data).await {
                        eprintln!("Error sending update through channel: {}", e);
                    }
                }

                interval.tick().await;
            }
        });

        while let Some(update) = rx.recv().await {
            let yaml_string = serde_yaml::to_string(&update).expect("Failed to convert to YAML");
            info!("{}", yaml_string);
        }
    }

    // Placeholder for actual service installation checks
    #[allow(dead_code)]
    fn is_osquery_installed(&self) -> bool {
        true
    }
    #[allow(dead_code)]
    fn is_wazuh_installed(&self) -> bool {
        true
    }
    #[allow(dead_code)]
    fn is_clamav_installed(&self) -> bool {
        true
    }
}

async fn get_menu_item_data(_osquery_installed: bool, _wazuh_installed: bool, _clamav_installed: bool, osquery_status: String, wazuh_status: String, clamav_status: String) -> Value {
    serde_json::json!({
        "menuItems": [
            {
                "text": "User Behavior Analytics",
                "description": format!("osquery is installed and {}.", osquery_status),
                "status": if osquery_status.contains("active") { 2 } else { 0 }
            },
            {
                "text": "Endpoint Detection and Response",
                "description": format!("Wazuh is installed and {}.", wazuh_status),
                "status": if wazuh_status.contains("active") { 2 } else { 0 }
            },
            {
                "text": "End-Point Protection",
                "description": format!("ClamAV is installed and {}.", clamav_status),
                "status": if clamav_status.contains("active") { 2 } else { 0 }
            }
        ]
    })
}

async fn get_service_status(service: &str) -> String {
    let output = Command::new("systemctl")
        .arg("is-active")
        .arg(service)
        .output()
        .expect("Failed to execute command");

    let status = String::from_utf8_lossy(&output.stdout).trim().to_string();

    match status.as_str() {
        "active" => "running".to_string(),
        "inactive" => "halted".to_string(),
        _ => "stopped".to_string(),
    }
}
