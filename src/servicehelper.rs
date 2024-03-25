use std::process::Command;
use serde_json::json;

pub struct ServiceHelper;

impl ServiceHelper {
    pub fn get_menu_item_data(&self) -> serde_json::Value {
        let osquery_installed = self.is_osquery_installed();
        let wazuh_installed = self.is_wazuh_installed();
        let clamav_installed = self.is_clamav_installed();

        let menu_item_data = json!({
            "menuItems": [
                {
                    "text": "User Behavior Analytics",
                    // "icon": "staroflife.shield",
                    "description": self.service_status_message("osquery", osquery_installed),
                    "status": if osquery_installed { 1 } else { 0 }
                },
                {
                    "text": "Endpoint Detection and Response",
                    // "icon": "lock.shield",
                    "description": self.service_status_message("Wazuh", wazuh_installed),
                    "status": if wazuh_installed { 1 } else { 0 }
                },
                {
                    "text": "End-Point Protection",
                    // "icon": "shield.checkered",
                    "description": format!("ClamAV is {}installed.", if clamav_installed { "" } else { "not " }),
                    "status": if clamav_installed { 1 } else { 0 }
                }
            ]
        });

        menu_item_data
    }

    fn service_status_message(&self, service: &str, is_installed: bool) -> String {
        format!("{} is {}installed.", service, if is_installed { "" } else { "not " })
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
