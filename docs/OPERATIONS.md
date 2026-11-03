# Production Operations Runbook

**Service:** VPN Next Gen AI Pro Max™ · **Severity:** SEV-0 (self-declared) · **Availability:** Not applicable.

> This is a joke repository, not a working VPN. There is no daemon, listener, tunnel or DNS modification.

## Day-0 launch

~~~sh
cargo run -- connect
~~~

Expected result: a truthful notice that no tunnel was established.

## Health checks

~~~sh
cargo run -- audit
cargo run -- benchmark
~~~

Expected result: no tunnel, no packet statistics and a full complement of marketing adjectives. These are **static reports**, not probes.

## Geo policy drill

~~~sh
cargo run -- geo-check uk
cargo run -- geo-check au
cargo run -- geo-check other
~~~

Expected result: fictional compliance status chosen by the user. No IP lookup, routing change or real geoblock.

## On-call escalation

| Alert | Playbook |
| --- | --- |
| No VPN tunnel | Working as designed |
| Reported packet loss | Impossible to calculate without samples |
| Startup failed | Check that Rust/Cargo are installed |
| Founder demanded higher valuation | Update the investor deck, not the data |
| Actual network access introduced | Stop: requires genuine threat modeling and review |
| Tea-based regulator unavailable | Retry after kettle boils |

## Disaster recovery

No servers, accounts or VPN credentials exist. Source restoration from Git is the full DR procedure.

## Change management

See [Architecture](ARCHITECTURE.md), [Security](SECURITY.md), and the [Architecture Decision Records](adr/README.md).

[← Documentation](README.md)
