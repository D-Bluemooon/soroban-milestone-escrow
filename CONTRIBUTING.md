# Contributing

Thanks for considering a contribution. This project is part of the Stellar open-source
ecosystem and participates in [Drips Wave](https://www.drips.network/wave/stellar).

## Ground rules

- Any change to milestone state transitions must preserve the invariants in
  `ARCHITECTURE.md` — please read that file before touching `src/lib.rs`.
- Every change needs a corresponding test in `src/test.rs`, including at least one
  test for the failure/panic path where relevant.
- Run `cargo fmt` and `cargo clippy --all-targets` before opening a PR.
- Follow [Conventional Commits](https://www.conventionalcommits.org/) for commit
  messages (`feat:`, `fix:`, `docs:`, `test:`, `chore:`).

## Picking up an issue

1. Check the [open issues](./ISSUES.md) or the repo's Issues tab.
2. Comment on the issue to claim it before starting work.
3. Issues are labeled by complexity, matching the Drips Wave points system:
   - `complexity:trivial` — small, well-scoped fixes (100 pts)
   - `complexity:medium` — a standard feature or non-trivial bug fix (150 pts)
   - `complexity:high` — a new capability or contract-level change (200 pts)
4. Open a draft PR early for feedback on direction before finishing, especially for
   anything touching the state machine.

## PR checklist

- [ ] `cargo build --target wasm32-unknown-unknown --release` succeeds
- [ ] `cargo test` passes, including new tests for the change
- [ ] `cargo clippy --all-targets -- -D warnings` is clean
- [ ] `ARCHITECTURE.md` updated if the state machine or auth model changed
- [ ] `README.md` updated if the public interface changed

## Reporting bugs or proposing features

Open an issue describing the current behavior, the expected behavior, and — for
anything touching fund movement or authorization — the specific edge case or attack
you're worried about. Escrow contracts are exactly the kind of code where "it works in
the happy path" isn't enough; please think about what a dishonest client, provider, or
arbiter could try to do.

## Code of conduct

Be respectful, assume good faith, and keep discussion focused on the work.
