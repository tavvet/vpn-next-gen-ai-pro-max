# Global Packet Delivery Network™ (GPDN/0)

**International courier-grade packet switching.**  
*From your network stack to a sorting facility near you.*

> [!CAUTION]
> **Entirely fictional satire.** VPN Next Gen AI Pro Max™ does not create VPN tunnels, send network packets, book shipments, print real postage, contact delivery operators, or offer any network protection. No partnership with any courier or logistics service exists. This document describes an imaginary service, not a feature you can use.

## Our revolutionary idea

Traditional VPNs transmit packets over the internet.

**We ship the packets using every international delivery network.**

Why trust fragile routers when an entire planet already knows how to move cardboard boxes?

Our research division has identified an untapped layer of the OSI model: **Layer 8 — Logistics**.

*By "every international delivery network" we mean absolutely none in reality. This is an imaginary, universal courier ecosystem with no named or actual commercial partners.*

> **Global availability. Local dispatch. International sorting. Unprecedented paperwork.**
>
> **Your data isn't traveling across the network. It is waiting for a delivery attempt.**

## How GPDN/0 works

Our imaginary reference architecture is a *store-and-forward* network, with additional emphasis on **store**.

~~~mermaid
flowchart LR
    App["User application"] --> Encoder["Packet-to-Parcel Encoder"]
    Encoder --> Box["Cardboard Encapsulation Layer"]
    Box --> Depot["Origin Sorting Hub"]
    Depot --> Customs{"Regulatory Vibes Check"}
    Customs -->|"Approved, allegedly"| Hub["Intercontinental Hub"]
    Customs -->|"More paperwork"| Queue["Compliance Backlog"]
    Queue -. "eventually, probably" .-> Hub
    Hub --> Depot2["Destination Sorting Hub"]
    Depot2 --> Recipient["Recipient / ACK signature"]
    Recipient -. "paper receipt by return mail" .-> App
~~~

**Diagram disclaimer:** Every component above is fictional. No such path is implemented, and no parcels, data or requests are sent.

### 1. Packet encapsulation — Cardboard-in-Cardboard (CiC)

Each hypothetical IP packet becomes a **Parcel Data Unit (PDU)**.

- **Header:** a shipping label.
- **Payload:** the packet printed on paper, unreadable to anyone who doesn't understand networking.
- **Frame:** a corrugated cardboard box.
- **Integrity check:** visually confirm that the box is still a box.
- **Confidentiality:** **none**. Cardboard is not encryption.

We call this **Physical Layer Abstraction by Literal Interpretation™**.

### 2. Routing — Border Gateway Postage (BGP*)

The parcel travels from sorting center to sorting center. Forwarding decisions are allegedly based on postal regions, hub opening hours and the mysterious availability of a loading dock.

*BGP here is a parody acronym. It is not the internet's Border Gateway Protocol and no real carrier routing is involved.*

### 3. Congestion control — Warehouse Window Scaling

Traditional TCP adjusts congestion windows.

GPDN/0 adjusts **warehouse windows**. During peak demand, our imaginary packets enter a queue behind holiday gifts and misplaced office chairs.

- **Packet queue:** warehouse shelf.
- **Head-of-line blocking:** a pallet in the aisle.
- **Traffic shaping:** repacking oversized parcels.
- **Priority traffic:** a sticker saying "URGENT".
- **Backpressure:** a sign reading "SORRY, WE ARE CLOSED".

### 4. Acknowledgements — Signature-Based ACK

A successful delivery would theoretically produce an ACK signed on paper.

If no signature arrives, the sender enters a **Retransmission Timeout** measured in business days and attempts to mail another imaginary box.

A packet arriving out of order is handled by our **Unboxing Reassembly Team**, which is also imaginary.

### 5. MTU — Maximum Truck Unit

The Maximum Transmission Unit has been replaced by the **Maximum Truck Unit (MTU)**.

- Small messages: envelope frames.
- Medium messages: cardboard frames.
- Large messages: pallet-level jumbo frames.
- Over-MTU packets: fragmentation into several boxes.
- Path MTU discovery: measure the warehouse entrance.

**Caution:** This is wordplay, not real networking guidance.

### 6. Reliability — Parcel Delivery Reliability Engineering (PDRE)

Our company guarantees **zero observed losses** among the **zero parcels actually shipped**. This is **not** a 0% loss rate. The statistical rate is undefined.

## The performance advantage

| Metric | Conventional networking | GPDN/0 fictional physical layer | Actual product |
| --- | --- | --- | --- |
| One-way latency | Usually measured in milliseconds | Ideally 2–14 **business days**, entirely invented | No measurements |
| RTT | Typically milliseconds | Two-way delivery, if an ACK ever arrives | **Undefined** |
| Throughput | Bits per second | Envelopes per working day | **N/A** |
| Jitter | Packet-arrival variation | Holiday-season variability | **Undefined** |
| Packet loss | Measurable on real traffic | "Address not found" notice | No packets sent |
| Congestion control | Algorithms and queues | Warehouse queues | Not implemented |
| Encryption | Cryptographic protocols | Cardboard and optimism | **None** |
| Global compliance | Depends on actual law | Fictional customs queue | No geo enforcement |

> Our marketing department is pleased to announce **99.999% delivery confidence**. Our engineering department notes that no data exists to estimate a probability. Investor Relations has requested that engineering stop taking questions.

## The OSI model, reimagined

| Layer | Standard concept | GPDN/0 interpretation |
| --- | --- | --- |
| 7 — Application | App data | Request shipping |
| 6 — Presentation | Serialization | Print the packet |
| 5 — Session | Session management | Remember the parcel tracking number |
| 4 — Transport | TCP/UDP | Truck / delivery van |
| 3 — Network | IP routing | Postal districts |
| 2 — Data link | Frames | Boxes |
| 1 — Physical | Cable / radio | Roads |
| **8 — Logistics** | **Not a real OSI layer** | **Customs declarations and lunchtime** |

**Engineering footnote:** The actual OSI reference model has **seven** layers. Layer 8 is a joke.

## International Coverage

Instead of listing courier partners, we proudly disclose:

**Carrier partners:** 0.  
**Carrier names disclosed:** 0.  
**Countries with established parcel-data routes:** 0.  
**Global presence:** present in the documentation of a public GitHub repository.

The project does not impersonate, name, integrate, or claim endorsement by any real international delivery company.

## Deployment

No real shipment is possible. For the **local, safe, text-only demo**:

~~~sh
cargo run -- parcel
~~~

Expected: a fictional shipping manifest and a clear statement that nothing was sent. The CLI uses no networking, prints no real shipping label and reads no personal addresses.

## Frequently anticipated support tickets

**My packet hasn't arrived.** We haven't sent it.

**Does this improve ping?** No. The theoretical "express" SLA is measured in business days.

**Can I track the shipment?** No physical shipment or tracking number exists. We recommend tracking our Git commits instead.

**Is the package encrypted?** No. A cardboard box is a container, not a cipher.

**Do you have an API?** Yes, in the sense that we can write "API" in this paragraph.

**Why this over fiber optics?** We already have enough fiber in our marketing presentation.

---

**The future of packet switching is parcel sorting.**  
*Restrictions First. Product Later. Signature Required.*

[← Documentation](README.md) · [Architecture](ARCHITECTURE.md) · [Protocol](PROTOCOL.md) · [Benchmarks](BENCHMARKS.md) · [Press](PRESS.md)
