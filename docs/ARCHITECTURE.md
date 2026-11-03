# Reference Architecture

**Architecture revision:** 0.0.1 · **Critical dependency:** User's sense of humor.

We use a proven optimization technique: **eliminate the entire data plane**.

## 1. Topology

~~~mermaid
flowchart LR
    User["User / Potential Shareholder"] --> CLI["Rust CLI"]
    CLI --> Args["Argument parser"]
    Args --> Engine["Enterprise response engine<br/>a match statement"]
    Engine --> Terminal["stdout"]
    CLI -. "zero network requests" .-> Internet["Internet"]
    Engine --> Compliance["Fictional geo policy"]
    Compliance --> Terminal
~~~

The dashed arrow depicts something that **does not occur**. No packets are sent by the executable.

## 2. Subsystems

| Component | Implementation | Service-level objective |
| --- | --- | --- |
| Client | Rust standard library | Print accurate disclaimers |
| Tunneling | Not implemented | Maintain zero connections |
| Cipher suite | Not implemented | Never claim encryption |
| DNS policy | No code | Never touch DNS |
| Firewall | No code | Never touch firewall |
| Geofencing | Static string match | Never read IP or location |
| Observability | Static terminal reports | Distinguish no samples from zero latency |
| Payments | None | Collect zero dollars |
| Investor Relations | Markdown | Exceed expectations before having customers |

## 3. Safety and correctness invariants

1. Every operation is local to the CLI process.
2. No credential, privilege or network connection is required.
3. There is no attempt to modify routes, DNS or firewall.
4. "Connect" must tell the truth: **NOT ESTABLISHED**.
5. "Geo-check" takes a **user-selected fictional jurisdiction** and performs no location lookup.
6. Undefined statistics are never reported as measured zero.
7. Naming, pricing and valuation claims are visibly satirical.

## 4. Boundaries and failure modes

The executable can fail due to compilation errors, stdout errors or unrealistic investor expectations. It cannot fail because of VPN protocol congestion: there is no protocol.

Adding real networking in the future would require a **new design and security review**, not an undocumented "minor enhancement."

[← Documentation](README.md) · [Protocol →](PROTOCOL.md) · [Security →](SECURITY.md)
