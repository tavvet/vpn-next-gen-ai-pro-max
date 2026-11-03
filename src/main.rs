//! VPN Next Gen AI Pro Max™
//! No network APIs. No dependencies. No false promises.
use std::env;

fn output(command: &str) -> String {
    match command {
        "free" | "connect" => format!(
            "VPN Next Gen AI Pro Max™\n\
             Plan: {}\n\
             Connecting to the next generation...\n\
             [====================] 100% marketing\n\
             Tunnel: NOT ESTABLISHED\n\
             Encryption: NOT ACTIVE\n\
             Network changes: NONE\n\
             This is a joke, not a VPN. Your traffic is unchanged.",
            if command == "free" { "Free" } else { "Pro Max (aspirational)" }
        ),
        "benchmark" => String::from(
            "VPN Next Gen AI Pro Max™ Benchmark\n\
             Packets sent: 0\n\
             RTT: undefined (no samples)\n\
             Throughput: N/A\n\
             Packet loss: undefined (0/0)\n\
             Zero-copy: technically yes; zero packets copied.\n\
             Result: our benchmark cannot measure a connection that does not exist."
        ),
        "audit" => String::from(
            "Security audit\n\
             Network sockets opened: 0\n\
             VPN tunnels established: 0\n\
             DNS configuration changes: 0\n\
             Firewall changes: 0\n\
             Claims of being a real VPN: 0\n\
             Warning: this demonstration provides NO network protection."
        ),
        "ipo" => String::from(
            "INVESTOR RELATIONS\n\
             Annual recurring revenue: $0\n\
             Paying customers: 0\n\
             Valuation: $25,000,000,000 (fictional)\n\
             Due diligence: postponed to post-IPO.\n\
             Disclaimer: there is no company, offering, or investment."
        ),
        "--help" | "-h" | "help" => String::from(
            "VPN Next Gen AI Pro Max™ (parody, NOT a VPN)\n\
             Usage: cargo run -- [free|connect|benchmark|audit|ipo|--help]\n\
             Every command prints text only. No network operations are performed."
        ),
        _ => String::from(
            "Unknown command. Try --help.\n\
             Nothing was changed. No network connection was attempted."
        ),
    }
}

fn main() {
    let argument = env::args().nth(1).unwrap_or_else(|| "--help".to_owned());
    println!("{}", output(&argument));
}

#[cfg(test)]
mod tests {
    use super::output;

    #[test]
    fn truthfully_discloses_no_tunnel() {
        assert!(output("connect").contains("NOT ESTABLISHED"));
        assert!(output("connect").contains("NOT ACTIVE"));
    }
    #[test]
    fn benchmark_avoids_division_by_zero_marketing() {
        let report = output("benchmark");
        assert!(report.contains("undefined (0/0)"));
        assert!(!report.contains("0% packet loss"));
    }
    #[test]
    fn all_commands_are_explicitly_parody() {
        assert!(output("--help").contains("NOT a VPN"));
        assert!(output("audit").contains("NO network protection"));
    }
}
