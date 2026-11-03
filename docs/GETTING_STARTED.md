# Getting started — Free Enterprise Trial

> **Product warning:** This is a parody CLI, not a VPN. No traffic is encrypted, routed, concealed or tunneled.

## System requirements

You will need a shell and optionally Rust/Cargo. You do **not** need a VPN server, an account, credentials, administrator access or a support contract.

## Fast path

From the cloned repository, read the script and run it:

~~~sh
cat scripts/free-vpn.sh
bash scripts/free-vpn.sh
~~~

It prints a joke. The script does not access the network, change your configuration or download anything.

## Enterprise path (Rust)

~~~sh
git clone https://github.com/tavvet/vpn-next-gen-ai-pro-max.git
cd vpn-next-gen-ai-pro-max
cargo run -- free
cargo run -- connect
~~~

Git cloning uses network access; Cargo writes build artifacts. The compiled **demo** does not create connections.

You should see the words:

~~~text
Tunnel: NOT ESTABLISHED
Encryption: NOT ACTIVE
Network changes: NONE
~~~

If you do, congratulations: our transparency layer is functioning as designed.

## Cross-border compliance readiness

~~~sh
cargo run -- geo-check uk
cargo run -- geo-check au
cargo run -- geo-check other
~~~

These commands perform a **fake regulatory simulation** of the region you typed. They do not determine your actual location and never block users. [Read the compliance dossier](COMPLIANCE.md).

## Verify the deployment

~~~sh
cargo run -- benchmark
cargo run -- audit
cargo test
~~~

The "audit" command only describes the toy program's behavior. It does not audit or secure your machine.

## Troubleshooting

| Observation | Explanation |
| --- | --- |
| IP did not change | Correct: no tunnel exists |
| Nothing appears in the routing table | Correct: no routes are modified |
| RTT says undefined | Correct: no timing samples |
| UK/AU check says restricted | Only a simulated joke; you selected the label |
| Need a real VPN | Please use an actual VPN, not this repository |

[← Docs](README.md) · [CLI reference →](CLI_REFERENCE.md)
