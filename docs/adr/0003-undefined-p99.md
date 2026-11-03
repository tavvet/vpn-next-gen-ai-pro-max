# ADR-0003: Undefined Is Not Zero

**Status:** Accepted (mathematically)

## Context

A product that sends no packets has no samples of network latency. Nevertheless, modern corporate marketing incentives encourage presenting extremely small numbers.

## Decision

Represent RTT, packet-loss fraction and p99 as **undefined** or **not applicable**, not as measured zero.

## Consequences

- No misleading performance claims.
- Every benchmark output becomes an uncomfortable lesson in sample sizes.
- Fewer opportunities for fraudulent charts; more opportunities for programming jokes.

## Non-goals

We do not attempt to measure network performance. We do not claim to outperform real VPNs.

[← ADR index](README.md) · [Benchmark policy](../BENCHMARKS.md)
