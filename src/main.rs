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
        "geo-check" => String::from(
            "REGIONAL COMPLIANCE DESK\n\
             Supply a fictional region: geo-check [uk|au|other].\n\
             No location detection, network access, or actual blocking occurs."
        ),
        "--help" | "-h" | "help" => String::from(
            "VPN Next Gen AI Pro Max™ (parody, NOT a VPN)\n\
             Usage: cargo run -- [free|connect|benchmark|audit|ipo|geo-check REGION|--help]\n\
             Every command prints text only. No network operations are performed."
        ),
        _ => String::from(
            "Unknown command. Try --help.\n\
             Nothing was changed. No network connection was attempted."
        ),
    }
}

fn geo_check(region: Option<&str>) -> String {
    let (label, outcome) = match region.unwrap_or("") {
        "uk" => ("UK", "BLOCKED (fictional): pending tea-based identity verification."),
        "au" => ("AUSTRALIA", "BLOCKED (fictional): kangaroo-grade review incomplete."),
        "other" => ("OTHER", "APPROVED (fictional): paperwork complete; still no VPN."),
        _ => ("NOT SELECTED", "Usage: geo-check [uk|au|other]"),
    };

    format!(
        "VPN Next Gen AI Pro Max™ — Regional Compliance Simulator\n\
         Region you typed: {label}\n\
         Simulated policy: {outcome}\n\
         Actual enforcement: NONE\n\
         Location/IP lookup: NONE\n\
         Network changes: NONE\n\
         THIS IS SATIRE, NOT A REAL VPN OR A REAL REGIONAL RESTRICTION."
    )
}

fn main() {
    let mut args = env::args().skip(1);
    let argument = args.next().unwrap_or_else(|| "--help".to_owned());
    if argument == "geo-check" {
        println!("{}", geo_check(args.next().as_deref()));
    } else {
        println!("{}", output(&argument));
    }
}

#[cfg(test)]
mod tests {
    use super::{geo_check, output};

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
    fn geography_is_only_a_user_selected_joke() {
        assert!(geo_check(Some("uk")).contains("tea-based"));
        assert!(geo_check(Some("au")).contains("kangaroo-grade"));
        assert!(geo_check(Some("other")).contains("still no VPN"));
        assert!(geo_check(None).contains("NOT SELECTED"));
        for region in ["uk", "au", "other"] {
            assert!(geo_check(Some(region)).contains("Location/IP lookup: NONE"));
            assert!(geo_check(Some(region)).contains("Network changes: NONE"));
        }
    }

    #[test]
    fn all_commands_are_explicitly_parody() {
        assert!(output("--help").contains("NOT a VPN"));
        assert!(output("audit").contains("NO network protection"));
    }
}
