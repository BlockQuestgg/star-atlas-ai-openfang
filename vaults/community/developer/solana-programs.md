---
title: "Solana Programs & On-Chain Architecture"
date: 2026-03-18
tags: [developer, solana, programs, on-chain, blockchain]
author: community
status: draft
source: "https://build.staratlas.com/, https://medium.com/star-atlas/star-atlas-opens-free-development-solana-builders-creators-sage-labs-programs"
---

# Solana Programs & On-Chain Architecture

Star Atlas is built natively on the Solana blockchain. All core game mechanics — fleet management, resource extraction, crafting, trading — execute as on-chain transactions through Solana programs (smart contracts).

## Why On-Chain

Building gameplay on-chain provides:

- **Transparency** — all game state is publicly verifiable
- **Composability** — third-party developers can build on top of game programs
- **True ownership** — assets (ships, resources) are SPL tokens and NFTs owned by players
- **Trustlessness** — game rules are enforced by code, not a central server
- **Interoperability** — game assets can interact with the broader Solana DeFi ecosystem

## Core Programs

Star Atlas gameplay is governed by several Solana programs:

### Fleet Formation (SAGE)

The core SAGE program manages fleet staking, movement, resource consumption, and sector interactions. When players stake ships, mine resources, or move between Starbases in [[sage-overview|SAGE Labs]], they are invoking instructions on this program.

### Marketplace Programs

On-chain order books for the [[galactic-marketplace|Galactic Marketplace]], handling both the global marketplace and local Starbase markets. Trade execution, fee collection, and order matching happen on-chain.

### Token Programs

Standard SPL token programs for [[token-economics|ATLAS and POLIS]], along with the POLIS Locker protocol that manages governance locking and voting power for the [[dao-overview|DAO]].

### NFT Programs

Ship NFTs use Solana's compressed NFT standards, enabling cost-effective minting and transfer of large asset collections.

## Open Development

Star Atlas has opened its programs for free development by the community. Builders can:

- Read any on-chain state from Star Atlas programs
- Build applications that interact with game state
- Create tools that compose with the existing on-chain economy
- Develop automation and analytics on top of transparent game data

See [[build-platform]] for API documentation and developer tools.

## Solana Ecosystem Integration

Star Atlas assets and programs exist within the broader Solana ecosystem:

- **ATLAS/POLIS** are tradeable on Solana DEXs (Raydium, Orca, Jupiter)
- **Ship NFTs** are listed on Solana NFT marketplaces (Tensor, Magic Eden)
- **On-chain data** is indexed by Solana explorers (Solscan, Birdeye)
- **DeFi composability** allows ATLAS integration with lending, staking, and yield protocols

## Key Resources for Developers

- [build.staratlas.com](https://build.staratlas.com/) — Official developer documentation
- [Galaxy API](https://build.staratlas.com/dev-resources/apis-and-data/galaxy-api) — REST API for game metadata
- [Data Source](https://build.staratlas.com/dev-resources/apis-and-data/data-source) — On-chain data access
- [F-KIT](https://github.com/staratlasmeta/FoundationKit) — UE5 Solana integration
- [Star Atlas GitHub](https://github.com/staratlasmeta) — Open-source repositories

## Related

- [[build-platform]] — Developer platform overview and F-KIT
- [[sage-overview]] — Gameplay built on these programs
- [[token-economics]] — Token programs and economic architecture

> [!info] Disclaimer
> Solana programs may be upgraded. Verify program IDs and interfaces at [build.staratlas.com](https://build.staratlas.com/). DYOR.
