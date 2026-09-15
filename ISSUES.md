# Open Issues

Mirrors the issues filed on the repo's Issues tab, with labels. Labels follow a **type**
label plus a **Drips Wave complexity** label (`complexity:trivial` = 100 pts,
`complexity:medium` = 150 pts, `complexity:high` = 200 pts).

---

### 1. Add a `reject_milestone` path so the client can send work back for revision

**Labels:** `enhancement`, `complexity:medium`

**Description**
Today a `Pending` milestone can only go to `Approved` (by the client) or `Disputed` (by
the provider). There's no way for a client to say "this isn't done yet, please revise"
without escalating straight to a dispute. Add a `reject_milestone(id, index, reason:
String)` entry point that keeps the milestone `Pending` but records the client's
feedback, so the provider can see it and resubmit.

**Acceptance criteria**
- New `rejection_note: Option<String>` (or similar) field on `Milestone`, or a
  separate storage entry keyed by `(id, index)`.
- `reject_milestone` requires client auth and only works on a `Pending` milestone.
- A test confirms a rejected milestone can still later be approved normally.

---

### 2. Add a time-bound auto-release fallback if the client goes unresponsive

**Labels:** `enhancement`, `complexity:high`

**Description**
If a client simply stops responding after work is delivered (never approves, never
disputes), the provider currently has no recourse short of the arbiter stepping in
informally. Add an optional `review_deadline: u64` (ledger timestamp) per milestone,
set at `create_escrow` time. If the deadline passes while the milestone is still
`Pending`, allow the provider to call a new `claim_after_deadline(id, index)` that
releases the funds without client approval.

This needs careful design around: what happens if the client *did* raise concerns but
never formally disputed; whether the deadline should be per-milestone or per-escrow; and
whether the arbiter should have a window to intervene before auto-release fires.

**Acceptance criteria**
- Deadline is optional (omitting it preserves today's behavior exactly).
- `claim_after_deadline` only succeeds after the deadline and only from `Pending`.
- Tests cover: claim before deadline (should fail), claim after deadline (should
  succeed), and claim after deadline on an already-`Disputed` milestone (should fail,
  since the arbiter path should take precedence).

---

### 3. `get_escrow` — add a docs example and confirm behavior on unknown ids

**Labels:** `documentation`, `bug`, `complexity:trivial`

**Description**
`get_escrow` currently panics with a generic "escrow not found" message if given an
unknown id, which is fine, but there's no test explicitly covering that path, and the
README doesn't show an example of reading escrow state. Add a test for the panic case,
and add a short "Reading escrow state" example to `README.md` showing a full
create → approve → release → `get_escrow` sequence via the Soroban CLI.

**Acceptance criteria**
- Test confirms `get_escrow` panics cleanly (not an SDK-level unwrap panic) on an
  unknown id.
- README has a copy-pasteable CLI walkthrough.
