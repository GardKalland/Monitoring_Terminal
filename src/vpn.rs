use std::process::Command;

// Made Claude add many providers, which i dont use. So i dont know if they work.
#[derive(Debug, Clone, PartialEq)]
pub enum VpnAction {
    None,
    //   Connecting,
    // Disconnecting,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VpnProvider {
    Unknown,
    ProtonVPN,
    NordVPN,
    Mullvad,
    ExpressVPN,
    Generic, // Detected via interface but unknown provider
}

impl VpnProvider {
    pub fn name(&self) -> &str {
        match self {
            VpnProvider::Unknown => "Unknown",
            VpnProvider::ProtonVPN => "ProtonVPN",
            VpnProvider::NordVPN => "NordVPN",
            VpnProvider::Mullvad => "Mullvad",
            VpnProvider::ExpressVPN => "ExpressVPN",
            VpnProvider::Generic => "VPN",
        }
    }
}

#[derive(Debug, Clone)]
pub struct VpnStatus {
    pub connected: bool,
    pub provider: VpnProvider,
    pub server: Option<String>,
    pub country: Option<String>,
    pub city: Option<String>,
    pub ip: Option<String>,
    pub protocol: Option<String>,
    pub interface: Option<String>,
    pub connection_time: Option<String>,
    pub raw_output: String,
    pub action: VpnAction,
    pub action_message: Option<String>,
}

/// Most of thise things just didnt work because i couldnt use an vpn service thingy mcjigg, or
/// maybe it works, i just didnt do much research. Ohh well.
/// AAAAAAANNNNNNDDDDD i have support for other vpn... but i havent tested it, cause i am using
/// proton. and couldnt be bothered to test other ones.
impl VpnStatus {
    pub fn new() -> Self {
        Self {
            connected: false,
            provider: VpnProvider::Unknown,
            server: None,
            country: None,
            city: None,
            ip: None,
            protocol: None,
            interface: None,
            connection_time: None,
            raw_output: String::new(),
            action: VpnAction::None,
            action_message: None,
        }
    }

    pub fn set_action(&mut self, action: VpnAction, message: Option<String>) {
        self.action = action;
        self.action_message = message;
    }

    pub fn clear_action(&mut self) {
        self.action = VpnAction::None;
        self.action_message = None;
    }

    pub fn check_protonvpn_cli() -> Self {
        let output = Command::new("protonvpn-cli").arg("status").output();

        match output {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();

                let raw = if !stdout.is_empty() {
                    stdout.clone()
                } else {
                    stderr
                };

                Self::parse_protonvpn_output(&stdout, raw)
            }
            Err(_) => {
                let output = Command::new("protonvpn").arg("status").output();

                match output {
                    Ok(output) => {
                        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

                        let raw = if !stdout.is_empty() {
                            stdout.clone()
                        } else {
                            stderr
                        };

                        Self::parse_protonvpn_output(&stdout, raw)
                    }
                    Err(_) => {
                        // CLI not found, but check if ProtonVPN is running via process
                        Self::check_protonvpn_process()
                    }
                }
            }
        }
    }

    fn check_protonvpn_process() -> Self {
        // Check if ProtonVPN process is running
        let ps_output = Command::new("ps").args(&["aux"]).output();

        if let Ok(output) = ps_output {
            let stdout = String::from_utf8_lossy(&output.stdout);

            if stdout.contains("protonvpn") || stdout.contains("proton-vpn") {
                let mut status = Self::new();
                status.provider = VpnProvider::ProtonVPN;

                // Try to get connection info from interfaces
                for iface in ["proton0", "pvpn0", "tun0", "wg0"] {
                    let if_output = Command::new("ip").args(&["addr", "show", iface]).output();

                    if let Ok(if_output) = if_output {
                        let if_stdout = String::from_utf8_lossy(&if_output.stdout);

                        if !if_stdout.is_empty() && !if_stdout.contains("does not exist") {
                            status.connected = true;
                            status.interface = Some(iface.to_string());
                            status.raw_output =
                                format!("ProtonVPN process detected (interface: {})", iface);

                            // Extract IP
                            for line in if_stdout.lines() {
                                if line.contains("inet ") {
                                    let parts: Vec<&str> = line.trim().split_whitespace().collect();
                                    if parts.len() >= 2 {
                                        status.ip = Some(parts[1].to_string());
                                    }
                                }
                            }

                            return status;
                        }
                    }
                }

                status.raw_output =
                    "ProtonVPN process detected but no active connection".to_string();
                return status;
            }
        }

        // Return Unknown status if ProtonVPN not detected
        Self::new()
    }

    fn parse_protonvpn_output(output: &str, raw: String) -> Self {
        let mut status = Self::new();
        status.raw_output = raw;
        status.provider = VpnProvider::ProtonVPN;

        if output.contains("Status:") && output.contains("Connected") {
            status.connected = true;
        } else if output.contains("Disconnected") || output.contains("No active") {
            status.connected = false;
            return status;
        }

        for line in output.lines() {
            let line = line.trim();

            if line.starts_with("Server:") {
                status.server = Some(line.replace("Server:", "").trim().to_string());
            } else if line.starts_with("Country:") {
                status.country = Some(line.replace("Country:", "").trim().to_string());
            } else if line.starts_with("City:") {
                status.city = Some(line.replace("City:", "").trim().to_string());
            } else if line.starts_with("IP:") {
                status.ip = Some(line.replace("IP:", "").trim().to_string());
            } else if line.starts_with("Protocol:") {
                status.protocol = Some(line.replace("Protocol:", "").trim().to_string());
            } else if line.starts_with("Time:") || line.starts_with("Connection time:") {
                let time = line
                    .replace("Time:", "")
                    .replace("Connection time:", "")
                    .trim()
                    .to_string();
                status.connection_time = Some(time);
            }
        }

        status
    }

    fn check_nordvpn() -> Option<Self> {
        let output = Command::new("nordvpn").arg("status").output();

        match output {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();

                if stdout.contains("Status: Connected") || stdout.contains("Status: connected") {
                    let mut status = Self::new();
                    status.connected = true;
                    status.provider = VpnProvider::NordVPN;
                    status.raw_output = stdout.clone();

                    // Parse NordVPN output
                    for line in stdout.lines() {
                        let line = line.trim();

                        if line.starts_with("Server:") || line.starts_with("Hostname:") {
                            status.server = Some(line.split(':').nth(1)?.trim().to_string());
                        } else if line.starts_with("Country:") {
                            status.country = Some(line.split(':').nth(1)?.trim().to_string());
                        } else if line.starts_with("City:") {
                            status.city = Some(line.split(':').nth(1)?.trim().to_string());
                        } else if line.starts_with("Current server IP:") || line.starts_with("IP:")
                        {
                            status.ip = Some(line.split(':').nth(1)?.trim().to_string());
                        } else if line.starts_with("Current protocol:")
                            || line.starts_with("Protocol:")
                        {
                            status.protocol = Some(line.split(':').nth(1)?.trim().to_string());
                        } else if line.starts_with("Uptime:") {
                            status.connection_time =
                                Some(line.split(':').nth(1)?.trim().to_string());
                        }
                    }

                    return Some(status);
                }
            }
            Err(_) => return None,
        }

        None
    }

    fn check_mullvad() -> Option<Self> {
        let output = Command::new("mullvad").arg("status").output();

        match output {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();

                if stdout.contains("Connected") {
                    let mut status = Self::new();
                    status.connected = true;
                    status.provider = VpnProvider::Mullvad;
                    status.raw_output = stdout.clone();

                    // Parse Mullvad output (format: "Connected to <server> in <location>")
                    for line in stdout.lines() {
                        if line.contains("Connected to") {
                            // Extract server and location from the line
                            if let Some(parts) = line.split("to ").nth(1) {
                                if let Some(server_location) = parts.split(" in ").next() {
                                    status.server = Some(server_location.trim().to_string());
                                }
                                if let Some(location) = parts.split(" in ").nth(1) {
                                    status.country = Some(location.trim().to_string());
                                }
                            }
                        } else if line.contains("IPv4:") {
                            status.ip = Some(line.split("IPv4:").nth(1)?.trim().to_string());
                        }
                    }

                    return Some(status);
                }
            }
            Err(_) => return None,
        }

        None
    }

    fn check_vpn_interface() -> Self {
        let mut status = Self::new();

        let interfaces = [
            ("proton0", VpnProvider::ProtonVPN),
            ("pvpn0", VpnProvider::ProtonVPN),
            ("nordlynx", VpnProvider::NordVPN),
            ("nordtun", VpnProvider::NordVPN),
            ("wg-mullvad", VpnProvider::Mullvad),
            ("wg0", VpnProvider::Generic),  // Generic WireGuard
            ("tun0", VpnProvider::Generic), // Generic tunnel
            ("tap0", VpnProvider::Generic), // Generic TAP
            ("utun", VpnProvider::Generic), // macOS VPN tunnel
        ];

        for (iface, provider) in interfaces {
            let output = Command::new("ip").args(&["addr", "show", iface]).output();

            if let Ok(output) = output {
                let stdout = String::from_utf8_lossy(&output.stdout);

                if !stdout.is_empty() && !stdout.contains("does not exist") {
                    status.connected = true;
                    status.provider = provider;
                    status.interface = Some(iface.to_string());
                    status.raw_output =
                        format!("{} interface {} detected", status.provider.name(), iface);

                    for line in stdout.lines() {
                        if line.contains("inet ") {
                            let parts: Vec<&str> = line.trim().split_whitespace().collect();
                            if parts.len() >= 2 {
                                status.ip = Some(parts[1].to_string());
                            }
                        }
                    }
                    break;
                }
            }
        }

        if !status.connected {
            status.raw_output =
                "No VPN connection detected. Supported: ProtonVPN, NordVPN, Mullvad, and generic VPN interfaces."
                    .to_string();
        }

        status
    }


//     // Function to get public IP address, maybe i will reanable it later.
//     // but i had to curl it, so i couldnt be bothered. to have it enabled.
//     pub fn get_public_ip() -> Option<String> {
//         let output = Command::new("curl")
//             .args(&["-s", "https://api.ipify.org"])
//             .output();
//
//         if let Ok(output) = output {
//             let ip = String::from_utf8_lossy(&output.stdout).trim().to_string();
//             if !ip.is_empty() && ip.len() < 50 {
//                 return Some(ip);
//             }
//         }
//
//         None
//     }
}

pub fn get_vpn_status() -> VpnStatus {
    // 1. Check NordVPN
    if let Some(status) = VpnStatus::check_nordvpn() {
        return status;
    }

    // 2. Check Mullvad
    if let Some(status) = VpnStatus::check_mullvad() {
        return status;
    }

    let proton_status = VpnStatus::check_protonvpn_cli();
    if proton_status.provider == VpnProvider::ProtonVPN {
        return proton_status;
    }

    VpnStatus::check_vpn_interface()
}
