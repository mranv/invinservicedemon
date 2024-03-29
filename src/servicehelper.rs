use std::process::Command;
use serde_json::{json, Value};
use log::info;
use tokio::sync::mpsc::{self, Receiver};

pub struct ServiceHelper {
    // Track previous states of services
    pub osquery_prev_status: String,
    pub wazuh_prev_status: String,
    pub clamav_prev_status: String,
}

impl ServiceHelper {
    pub async fn run_service_check_timer(&mut self) {
        let (tx, mut rx): (tokio::sync::mpsc::Sender<Value>, Receiver<Value>) = mpsc::channel(100);

        // Clone or copy the data needed from `self`
        let osquery_installed = self.is_osquery_installed();
        let wazuh_installed = self.is_wazuh_installed();
        let clamav_installed = self.is_clamav_installed();

        // Spawn a separate task to run the service checks and send updates through the channel
        tokio::spawn(async move {
            let mut osquery_prev_status = String::new();
            let mut wazuh_prev_status = String::new();
            let mut clamav_prev_status = String::new();

            loop {
                // Check for changes in service status
                let osquery_status = get_service_status("osqueryd").await;
                let wazuh_status = get_service_status("wazuh-agentd").await;
                let clamav_status = get_service_status("clamav-clamonacc").await;

                if osquery_status != osquery_prev_status ||
                   wazuh_status != wazuh_prev_status ||
                   clamav_status != clamav_prev_status {

                    // Update previous states
                    osquery_prev_status = osquery_status.clone();
                    wazuh_prev_status = wazuh_status.clone();
                    clamav_prev_status = clamav_status.clone();

                    // Send updated menu item data through the channel
                    let menu_item_data = get_menu_item_data(
                        osquery_installed,
                        wazuh_installed,
                        clamav_installed,
                        osquery_status,
                        wazuh_status,
                        clamav_status,
                    ).await;

                    if let Err(e) = tx.send(menu_item_data).await {
                        eprintln!("Error sending update through channel: {}", e);
                    }
                }

                // No explicit sleep here
            }
        });

        // Listen for updates from the channel and print them
        while let Some(update) = rx.recv().await {
            info!("{}", serde_json::to_string_pretty(&update).unwrap());
        }
    }

    fn is_osquery_installed(&self) -> bool {
        let osquery_paths = ["/usr/bin/osqueryi", "/usr/bin/osqueryctl"];
        osquery_paths.iter().all(|&path| std::path::Path::new(path).exists())
    }

    fn is_wazuh_installed(&self) -> bool {
        let required_files = ["agent-auth", "manage_agents", "wazuh-agentd", "wazuh-control", "wazuh-execd", "wazuh-logcollector", "wazuh-modulesd", "wazuh-syscheckd"];
        required_files.iter().all(|&file| std::path::Path::new("/var/ossec/bin/").join(file).exists())
    }

    fn is_clamav_installed(&self) -> bool {
        let output = Command::new("which")
            .arg("clamscan")
            .output()
            .expect("Failed to execute command");

        output.status.success()
    }
}

pub async fn get_menu_item_data(osquery_installed: bool, wazuh_installed: bool, clamav_installed: bool, osquery_status: String, wazuh_status: String, clamav_status: String) -> Value {
    let menu_item_data = json!({
        "menuItems": [
            {
                "text": "User Behavior Analytics",
                "description": format!("osquery is {}installed and {}.",
                                       if osquery_installed { "" } else { "not " },
                                       osquery_status),
                "status": if osquery_installed && osquery_status.contains("active") { 2 } else { 0 }
            },
            {
                "text": "Endpoint Detection and Response",
                "description": format!("Wazuh is {}installed and {}.",
                                       if wazuh_installed { "" } else { "not " },
                                       wazuh_status),
                "status": if wazuh_installed && wazuh_status.contains("active") { 2 } else { 0 }
            },
            {
                "text": "End-Point Protection",
                "description": format!("ClamAV is {}installed and {}.",
                                       if clamav_installed { "" } else { "not " },
                                       clamav_status),
                "status": if clamav_installed && clamav_status.contains("active") { 2 } else { 0 }
            }
        ]
    });

    menu_item_data
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
        "activating" | "deactivating" | "failed" | _ => "stopped".to_string(),
    }
}
