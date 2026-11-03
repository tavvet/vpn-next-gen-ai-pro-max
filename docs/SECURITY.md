# Security Model and Threat Assessment

**Overall assessment: Not a VPN. Not a privacy tool.**

> [!CAUTION]
> This repository is satire. Its demo does **not** encrypt or tunnel traffic, hide IP addresses, bypass restrictions, protect public Wi-Fi, or authenticate peers.

## Intended program behavior

The Rust demo prints static strings based on command-line arguments. The Bash demo only prints messages. No network access, IP inspection, credential collection, firewall edits, DNS changes, privilege escalation, installer or background service is implemented.

Downloading the repository with Git, or installing compilers, are separate user actions with their own implications.

## Threat model

| Threat | Mitigation provided |
| --- | --- |
| Network eavesdropping | **None** |
| Malicious Wi-Fi | **None** |
| Traffic interception | **None** |
| Identity or IP exposure | **None** |
| Third-party tracking | **None** |
| Actual regional restrictions | **None** |
| Misunderstanding this is a VPN | Prominent and repeated disclosures |

We do not represent lack of functionality as a security guarantee.

## Compliance and jurisdiction simulation

The \`geo-check\` command is **a theatrical string lookup**, not a jurisdiction detection engine. It does not geolocate, inspect IPs, reject users, or change access. The UK and Australia jokes are not claims about their laws.

## Code review and contributions

Any addition of network requests, credential handling, privileged operations or actual tunneling changes the project scope and must never be hidden behind parody documentation.

Potential bugs can be reported via ordinary GitHub issues. Never post private information or credentials.

**This page is not a security certification.**

[← Documentation](README.md) · [Compliance](COMPLIANCE.md) · [Quickstart](GETTING_STARTED.md)
