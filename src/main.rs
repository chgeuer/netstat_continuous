use chrono::Utc;
use netstat2::{get_sockets_info, AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo};
use std::{collections::HashMap, thread, time::Duration};

fn main() {
    // Set up flags for TCP IPv4 and IPv6 sockets
    let af_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
    let proto_flags = ProtocolFlags::TCP;

    loop {
        // Get current UTC timestamp
        let now = Utc::now();
        
        // Collect socket information
        let sockets = match get_sockets_info(af_flags, proto_flags) {
            Ok(sockets) => sockets,
            Err(e) => {
                eprintln!("Error getting socket information: {}", e);
                thread::sleep(Duration::from_secs(1));
                continue;
            }
        };

        // Count connections by state
        let mut state_counts: HashMap<String, u32> = HashMap::new();
        
        for socket in sockets {
            if let ProtocolSocketInfo::Tcp(tcp_socket) = socket.protocol_socket_info {
                let state = format!("{:?}", tcp_socket.state);
                *state_counts.entry(state).or_insert(0) += 1;
            }
        }

        // Format JSON string manually
        let json = format!(
            concat!(
                "{{",
                "\"timestamp\":\"{}\", ",
                "\"established\":{:7}, ",
                "\"time_wait\":{:7}, ",
                "\"close_wait\":{:7}, ",
                "\"syn_sent\":{:7}, ",
                "\"syn_received\":{:7}, ",
                "\"fin_wait1\":{:7}, ",
                "\"fin_wait2\":{:7}, ",
                "\"closing\":{:7}, ",
                "\"last_ack\":{:7}, ",
                "\"listen\":{:7}",
                "}}"
            ),
            now.format("%Y-%m-%dT%H:%M:%SZ"),
            state_counts.get("Established").unwrap_or(&0),
            state_counts.get("TimeWait").unwrap_or(&0),
            state_counts.get("CloseWait").unwrap_or(&0),
            state_counts.get("SynSent").unwrap_or(&0),
            state_counts.get("SynReceived").unwrap_or(&0),
            state_counts.get("FinWait1").unwrap_or(&0),
            state_counts.get("FinWait2").unwrap_or(&0),
            state_counts.get("Closing").unwrap_or(&0),
            state_counts.get("LastAck").unwrap_or(&0),
            state_counts.get("Listen").unwrap_or(&0)
        );

        println!("{}", json);

        // Sleep for one second before next update
        thread::sleep(Duration::from_secs(1));
    }
}