# ADR-0002: Regional Compliance Without Geolocation

**Status:** Accepted (satire) · **Scope:** UK and Australia joke scenarios

## Context

Enterprise software often features elaborate regional policy documentation. Our project has no networking, so enforcing geo-restrictions would be especially performative.

## Decision

Create an explicit local command \`geo-check\` with three user-selected regions: \`uk\`, \`au\`, \`other\`.

The command prints **fictional** status messages about tea verification and kangaroo review. The software never examines IP addresses, locale, interfaces or network configuration. It never blocks, allows or bypasses real traffic.

## Consequences

- Two more sections for investor slides.
- Exactly zero actual restrictions.
- The user can change the *joke output* by passing a different label, with no security consequence.
- We must distinguish satire from real legal or availability information.

## Alternatives considered

**Geolocate the user:** Rejected as unnecessary, intrusive and entirely pointless.

**Implement regional blocks:** Rejected because the product has no service to block.

[← ADR index](README.md) · [Compliance policy](../COMPLIANCE.md)
