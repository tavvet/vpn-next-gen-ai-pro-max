# Command-Line Interface Reference

**Binary:** \`vpn-next-gen-ai-pro-max\` · **Runtime dependencies:** none · **Network dependencies:** absolutely none.

All commands only print local text. None establish a VPN, encrypt traffic, inspect IP addresses or change network settings.

| Command | Description |
| --- | --- |
| \`free\` | Activate the $0 non-tunnel plan |
| \`connect\` | Announce that no tunnel exists |
| \`benchmark\` | Present an honest non-benchmark with undefined p99 |
| \`audit\` | Print a disclaimer, **not** an actual security audit |
| \`ipo\` | Reveal our fictional $25 billion valuation |
| \`geo-check uk\` | Pretend to block access pending tea-based review |
| \`geo-check au\` | Pretend to block access pending kangaroo clearance |
| \`geo-check other\` | Approve the paperwork, not a connection |
| \`--help\` | Show the menu of enterprise-grade non-features |

## Examples

~~~sh
cargo run -- --help
cargo run -- connect
cargo run -- parcel
cargo run -- geo-check uk
cargo run -- geo-check au
cargo run -- benchmark
cargo run -- audit
cargo test
~~~

For the fictional logistics protocol and an explanation of Maximum Truck Units, see [GPDN/0](GLOBAL_PACKET_DELIVERY.md).

## Exit codes

All recognized and unknown commands exit normally because no actual deployment operation is performed. Operating-system errors (e.g., failing to write stdout) are outside this fictional SLA.

## Privacy

The demo does **not** check your location, IP address, hostname, locale, credentials, network interfaces, or DNS settings. The fake regional policy only reads the explicit argument you type.

[← Documentation](README.md) · [Compliance](COMPLIANCE.md)
