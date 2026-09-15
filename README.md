# soroban-milestone-escrow

A minimal **milestone-based escrow contract** for [Soroban](https://soroban.stellar.org),
Stellar's smart contract platform.

## Why this exists

Freelance and contract-based work — the kind a lot of Stellar's growing app ecosystem
(gig platforms, marketplaces, tourism/rental booking apps) already needs — benefits from
splitting payment into milestones rather than all-at-once. This repo is a small,
readable reference contract for that pattern: a client deposits funds up front, splits
them across milestones, approves each as work lands, and a neutral arbiter can step in
if client and provider disagree.

It's deliberately scoped down compared to full-featured escrow platforms already active
in the Stellar ecosystem (e.g. Trustless Work, SafeTrust) — the goal here is a compact,
easy-to-audit reference contract that smaller projects can fork or learn from, not a
competing platform.

## What it does

- `create_escrow(client, provider, arbiter, token, milestones) -> id`
  Deposits the sum of all milestone amounts from `client` and opens an escrow.
- `approve_milestone(id, index)` — client-only. Marks a milestone as approved.
- `release_milestone(id, index)` — pays an approved milestone's funds to the provider.
- `raise_dispute(id, index)` — provider-only. Flags a pending milestone as disputed.
- `resolve_dispute(id, index, release_to_provider)` — arbiter-only. Sends the disputed
  milestone's funds to whichever side the arbiter decides.
- `get_escrow(id)` — read-only view of the full escrow state.

## Getting started

```bash
# build
cargo build --target wasm32-unknown-unknown --release

# run tests
cargo test
```

## Docs

- [`ARCHITECTURE.md`](./ARCHITECTURE.md) — state machine, storage layout, auth model
- [`CONTRIBUTING.md`](./CONTRIBUTING.md) — how to propose changes and pick up an issue
- [`ISSUES.md`](./ISSUES.md) — current open issues, mirrored here for convenience

## License

MIT — see [`LICENSE`](./LICENSE).
