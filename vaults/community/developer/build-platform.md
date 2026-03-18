---
title: "Star Atlas Build — Developer Platform"
date: 2026-03-18
tags: [developer, f-kit, sdk, api, unreal-engine, solana]
author: community
status: draft
source: "https://build.staratlas.com/"
---

# Star Atlas Build — Developer Platform

Star Atlas provides an open developer platform at [build.staratlas.com](https://build.staratlas.com/) for builders to create on top of the Star Atlas ecosystem. The platform offers SDKs, APIs, and tooling for both game developers and data/application builders.

## F-KIT (Foundation Kit)

The F-KIT is Star Atlas's flagship developer tool — a free, open-source Unreal Engine 5 plugin that enables game developers to integrate Solana blockchain functionality into their projects.

**Repository:** [github.com/staratlasmeta/FoundationKit](https://github.com/staratlasmeta/FoundationKit)

**Capabilities:**
- In-engine generation and import of Solana private keys and wallets
- Reading on-chain data from within Unreal Engine 5
- Performing on-chain transactions (sNFTs, xNFTs, tokens) through the game interface
- Wallet management — viewing and interacting with SPL tokens and NFTs
- Integration with any Solana program (not limited to Star Atlas)

**Key Feature:** F-KIT is designed to be general-purpose. While it was built for Star Atlas, it works with any Solana-based project, making it a tool for the broader Solana gaming ecosystem.

**Getting Started:**
- [How to Setup F-Kit in Your Project](https://build.staratlas.com/dev-resources/unreal-engine-tooling/how-to-setup-f-kit-in-your-project)
- [F-Kit Blueprints Guide](https://build.staratlas.com/dev-resources/unreal-engine-tooling/f-kit-documentation)

## APIs and Data

Star Atlas exposes several APIs for developers building applications, dashboards, bots, and tools:

### Galaxy API

A traditional REST API serving metadata about the Star Atlas ecosystem — ships, resources, marketplace data, and more.

- [Galaxy API Documentation](https://build.staratlas.com/dev-resources/apis-and-data/galaxy-api)

### Data Source

Direct access to on-chain data from Star Atlas Solana programs, enabling builders to read game state, fleet positions, marketplace orders, and economic data.

- [Data Source Documentation](https://build.staratlas.com/dev-resources/apis-and-data/data-source)

### Factory

API for interacting with the Star Atlas asset creation and management system.

- [Factory Documentation](https://build.staratlas.com/apis-and-data/factory)

## What You Can Build

The Star Atlas developer platform supports a range of projects:

- **Games and experiences** — Build Solana-integrated UE5 games that connect to the Star Atlas metaverse
- **Tools and dashboards** — Fleet trackers, marketplace analyzers, resource calculators
- **Bots and automation** — Trading bots, fleet management automation, alert systems
- **Data analytics** — On-chain analysis of the Star Atlas economy
- **Community tools** — DAO dashboards, governance trackers, notification systems

## Open Development

Star Atlas has opened its Solana programs for free development, allowing builders and creators to build directly on the SAGE Labs programs and beyond. See [[solana-programs]] for technical details on the on-chain architecture.

## Related

- [[solana-programs]] — On-chain programs and Solana integration
- [[sage-overview]] — The gameplay layer built on these programs
- [[token-economics]] — The economic system developers can interact with
- [[galactic-marketplace]] — Marketplace APIs and data

> [!info] Disclaimer
> Developer tools and APIs evolve. Check [build.staratlas.com](https://build.staratlas.com/) for the latest documentation. DYOR.
