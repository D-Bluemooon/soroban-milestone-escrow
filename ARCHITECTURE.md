# Architecture

## Overview

The contract stores a single `Escrow` struct per escrow id, containing an ordered list
of `Milestone`s. There is no global state beyond an id counter — every operation reads
one `Escrow`, mutates it, and writes it back.

## State machine

Each milestone moves through a strict, one-directional state machine:

```
Pending ──approve_milestone(client)──▶ Approved ──release_milestone(anyone)──▶ Released
   │
   └──raise_dispute(provider)──▶ Disputed ──resolve_dispute(arbiter)──▶ Released
```

Key invariants enforced by the contract:
- A milestone can only be approved from `Pending`.
- A milestone can only be released from `Approved`.
- A milestone can only be disputed from `Pending` (once approved, there's nothing left
  to dispute — the client already signed off).
- A milestone can only be resolved from `Disputed`.
- Every path terminates at `Released` exactly once; there is no way to release the same
  milestone's funds twice.

## Data model

```rust
pub struct Milestone {
    pub description: String,
    pub amount: i128,
    pub status: MilestoneStatus, // Pending | Approved | Released | Disputed
}

pub struct Escrow {
    pub client: Address,
    pub provider: Address,
    pub arbiter: Address,
    pub token: Address,
    pub milestones: Vec<Milestone>,
}
```

Stored under `DataKey::Escrow(id)` in persistent storage.

## Authorization model

| Action              | Requires auth from |
|----------------------|--------------------|
| `create_escrow`      | `client`            |
| `approve_milestone`  | `client`            |
| `release_milestone`  | *(none — outcome already determined by prior approval)* |
| `raise_dispute`      | `provider`          |
| `resolve_dispute`    | `arbiter`           |

`release_milestone` intentionally requires no additional authorization: by the time a
milestone is `Approved`, the client has already authorized that outcome, so releasing
the already-escrowed funds doesn't need a second signature. This also means anyone
(a keeper, the provider themselves) can trigger the release once it's approved, rather
than leaving the provider dependent on the client submitting a second transaction.

## Trust assumptions

The `arbiter` is a single trusted address chosen at escrow creation — this contract does
not implement multi-party arbitration, staking, or reputation. That's an explicit,
documented limitation (see `ISSUES.md`) rather than an oversight: the goal of this repo
is a minimal, readable base, and arbitration schemes are a large enough design space to
deserve their own scoped issue rather than being bolted on up front.

## Token transfers

Like `soroban-stream-fund`, this contract moves funds via `soroban_sdk::token::Client`,
targeting the SEP-41 token interface, so it works with any compliant Stellar asset,
including USDC.
