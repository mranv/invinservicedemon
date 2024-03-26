use std::process::Command;
use serde_json::{json, Value};
use log::info;

pub struct ServiceHelper;

impl ServiceHelper {
    pub async fn run_service_check_timer(&self) -> Value {
        loop {
            // Run service checks
            let menu_item_data = self.get_menu_item_data().await;
            info!("{}", serde_json::to_string_pretty(&menu_item_data).unwrap());

            // Wait for 10 seconds before running the checks again
            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        }
    }

    async fn get_menu_item_data(&self) -> Value {
        let osquery_installed = self.is_osquery_installed();
        let osquery_status = self.get_service_status("osqueryd");

        let wazuh_installed = self.is_wazuh_installed();
        let wazuh_status = self.get_service_status("wazuh-agentd");

        let clamav_installed = self.is_clamav_installed();
        let clamav_status = self.get_service_status("clamav-clamonacc");

        json!({
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
        })
    }

    fn is_osquery_installed(&self) -> bool {
        let osquery_paths = ["/usr/bin/osqueryi", "/usr/bin/osqueryctl"];
        osquery_paths.iter().all(|&path| std::path::Path::new(path).exists())
    }

    fn get_service_status(&self, service: &str) -> String {
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