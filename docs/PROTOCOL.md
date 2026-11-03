# RFC-NGT-0000: Next Generation Transport (NGT/0)

**Category:** Informational Parody · **Status:** Zero Draft · **Standards authority:** None.

> This document describes a fictional protocol. No VPN exists, no encryption is provided, and this is not a security standard.

## Abstract

NGT/0 reduces protocol overhead by removing the requirement for *a protocol*.

## 1. Conventions

- **MUST** means the documentation sounds authoritative.
- **SHOULD** means the next funding round depends on it.
- **MAY** means we have not begun.
- **Packet** means a concept strictly outside scope.

## 2. Wire format

There is no wire format. NGT/0 produces no transport packets.

An arbitrary packet observed on a device is not evidence of NGT/0 because this program does not communicate over any network.

## 3. State machine

~~~mermaid
stateDiagram-v2
    [*] --> Unconnected
    Unconnected --> Unconnected: connect (prints disclaimer)
    Unconnected --> [*]: program exits
~~~

This is a conceptual representation of static CLI behavior, not a real networking state machine.

## 4. Handshake

The handshake completes in **zero exchanges** because none is attempted. This is **not** a claim of zero latency. No latency measurement exists.

## 5. Congestion control

No congestion control algorithm is implemented. A protocol that sends zero packets cannot measure capacity, loss or queuing delay.

## 6. Authentication, confidentiality, integrity

Not provided. There are no keys, certificates, tunnels, or encrypted messages.

**Never use NGT/0 to protect traffic.**

## 7. Geographic policy extension

The CLI recognizes fictional labels \`uk\`, \`au\` and \`other\` for a printed policy simulation. It does not inspect the host, network, IP, locale or geolocation. See [Compliance](COMPLIANCE.md).

## 8. Optional physical courier transport (GPDN/0)

A separate *fictional* extension proposes encapsulating network packets in cardboard boxes and forwarding them through imaginary international sorting centers. No courier service, real parcel delivery, internet transmission or tunnel exists.

See the [Global Packet Delivery Network™ specification](GLOBAL_PACKET_DELIVERY.md) for the proposed business-day RTT and Maximum Truck Unit.

## 9. Version negotiation

0 is the only available version. Backward compatibility is flawless because there are no preceding versions.

[← Documentation](README.md) · [Architecture](ARCHITECTURE.md) · [Benchmarking](BENCHMARKS.md)
