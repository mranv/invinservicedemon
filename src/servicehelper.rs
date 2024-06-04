// src/servicehelper.rs
use std::process::Command;
use ansi_term::Color;

pub struct ServiceHelper {
    pub osquery_prev_status: String,
    pub wazuh_prev_status: String,
    pub clamav_prev_status: String,
}

impl ServiceHelper {
    pub async fn run_service_check_timer(&mut self) {
        let osquery_installed = self.is_osquery_installed();
        let wazuh_installed = self.is_wazuh_installed();
        let clamav_installed = self.is_clamav_installed();

        loop {
            let new_osquery_status = get_service_status("osqueryd").await;
            let new_wazuh_status = get_service_status("wazuh-agentd").await;
            let new_clamav_status = get_service_status("clamav-clamonacc").await;

            if new_osquery_status != self.osquery_prev_status || new_wazuh_status != self.wazuh_prev_status || new_clamav_status != self.clamav_prev_status {
                self.osquery_prev_status = new_osquery_status.clone();
                self.wazuh_prev_status = new_wazuh_status.clone();
                self.clamav_prev_status = new_clamav_status.clone();

                let osquery_color = if osquery_installed { Color::Green } else { Color::Red };
                let wazuh_color = if wazuh_installed { Color::Green } else { Color::Red };
                let clamav_color = if clamav_installed { Color::Green } else { Color::Red };

                let mut osquery_string = String::new();
                if !osquery_installed {
                    osquery_string.push_str("not ");
                }
                osquery_string.push_str(&osquery_color.paint("installed").to_string());

                let mut wazuh_string = String::new();
                if !wazuh_installed {
                    wazuh_string.push_str("not ");
                }
                wazuh_string.push_str(&wazuh_color.paint("installed").to_string());

                let mut clamav_string = String::new();
                if !clamav_installed {
                    clamav_string.push_str("not ");
                }
                clamav_string.push_str(&clamav_color.paint("installed").to_string());

                let menu_item_data = get_menu_item_data(
                    osquery_installed,
                    wazuh_installed,
                    clamav_installed,
                    new_osquery_status.clone(),
                    new_wazuh_status.clone(),
                    new_clamav_status.clone(),
                    osquery_string,
                    wazuh_string,
                    clamav_string,
                ).await;
                println!("{}", menu_item_data);
            }

            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        }
    }

    pub async fn get_menu_item_data(&self) -> String {
        let osquery_installed = self.is_osquery_installed();
        let wazuh_installed = self.is_wazuh_installed();
        let clamav_installed = self.is_clamav_installed();

        let osquery_color = if osquery_installed { Color::Green } else { Color::Red };
        let wazuh_color = if wazuh_installed { Color::Green } else { Color::Red };
        let clamav_color = if clamav_installed { Color::Green } else { Color::Red };

        let mut osquery_string = String::new();
        if !osquery_installed {
            osquery_string.push_str("not ");
        }
        osquery_string.push_str(&osquery_color.paint("installed").to_string());

        let mut wazuh_string = String::new();
        if !wazuh_installed {
            wazuh_string.push_str("not ");
        }
        wazuh_string.push_str(&wazuh_color.paint("installed").to_string());

        let mut clamav_string = String::new();
        if !clamav_installed {
            clamav_string.push_str("not ");
        }
        clamav_string.push_str(&clamav_color.paint("installed").to_string());

        get_menu_item_data(
            osquery_installed,
            wazuh_installed,
            clamav_installed,
            self.osquery_prev_status.clone(),
            self.wazuh_prev_status.clone(),
            self.clamav_prev_status.clone(),
            osquery_string,
            wazuh_string,
            clamav_string,
        ).await
    }

    fn is_osquery_installed(&self) -> bool {
        let osquery_paths = ["/usr/bin/osqueryi", "/usr/bin/osqueryctl"];
        osquery_paths.iter().all(|&path| std::path::Path::new(path).exists())
    }

    fn is_wazuh_installed(&self) -> bool {
        let required_files = [
            "agent-auth", "manage_agents", "wazuh-agentd", "wazuh-control",
            "wazuh-execd", "wazuh-logcollector", "wazuh-modulesd", "wazuh-syscheckd"
        ];
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

async fn get_menu_item_data(
    osquery_installed: bool,
    wazuh_installed: bool,
    clamav_installed: bool,
    osquery_status: String,
    wazuh_status: String,
    clamav_status: String,
    osquery_string: String,
    wazuh_string: String,
    clamav_string: String,
) -> String {
    format!(
        r#"menuItems:
  - text: "User Behavior Analytics"
    description: "{}"
    status: {}
  - text: "Endpoint Detection and Response"
    description: "{}"
    status: {}
  - text: "End-Point Protection"
    description: "{}"
    status: {}
"#,
        format!(
            "osquery is {} and {}.",
            osquery_string,
            osquery_status
        ),
        if !osquery_installed { "0" } else { if osquery_status == "halted" { "0" } else { "1" } },
        format!(
            "Wazuh is {} and {}.",
            wazuh_string,
            wazuh_status
        ),
        if !wazuh_installed { "0" } else { if wazuh_status == "halted" { "0" } else { "1" } },
        format!(
            "ClamAV is {} and {}.",
            clamav_string,
            clamav_status
        ),
        if !clamav_installed { "0" } else { if clamav_status == "halted" { "0" } else { "1" } },
    )
}

async fn get_service_status(service: &str) -> String {
    let output = Command::new("systemctl")
        .arg("is-active")
        .arg(service)
        .output()
        .expect("Failed to execute command");

    let status = String::from_utf8_lossy(&output.stdout).trim().to_string();

    match status.as_str() {
        "active" => Color::Green.paint("running").to_string(),
        "inactive" => Color::Yellow.paint("halted").to_string(),
        "activating" | "deactivating" | "failed" | _ => "stopped".to_string(),
    }
}
