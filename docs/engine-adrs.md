# Engine ADRs — decisions on CR-underdetermined semantics

Each entry answers one item from the mtg-rules skill's
`references/underdetermined.md` registry (the engine-choice list), keyed by
its stable `UD-n` id. Convention: the skill records *what the CR leaves
open*, engine-agnostically; this file records *which option deckmaste chose*
and where the code embodies it. A decision made implicitly by code is still
a decision — write it down here when discovered. When a CR update settles an
entry upstream, the skill moves it to its Settled section and the ADR here
becomes historical.

## UD-8 — shuffle randomness — DECIDED

Uniform and independent, from a seeded deterministic generator:
`GameState.rng` is a `ChaCha8Rng` built with `seed_from_u64(config.seed)`
(`crates/deckmaste_engine/src/state.rs`); shuffles draw from it uniformly.
Determinism-under-seed is load-bearing for simulation reproducibility
(`sim.rs` takes `seed: u64`). No commitment yet to a verifiability story
(committed-seed reveal for adversarial play) — that is a runner/server
concern, like hidden-information views.

## UD-9 — LKI snapshot extent — DECIDED

A snapshot is an **enumerated-field value**, not a full object copy:
`LkiSnapshot { object, source, controller, tapped, damage, left }`
(`crates/deckmaste_engine/src/lki.rs`), where `source` is the `CardId` spine
that derives printed characteristics. Extent policy: fields are added as
consumers demand them (triggers, damage attribution), never removed; the
snapshot is a VALUE riding the event — no retained object references
(GC-freedom is the point; see the zone-changes-immediate design note).

## UD-12 — intra-batch event ordering — DECIDED

The event log is a total order (commit order), including within a batch
(SBA sweeps, simultaneous damage). The engine asserts no rule may OBSERVE
relative order within a batch — batch members are simultaneous for trigger
and replacement purposes; the log order is representational only. Any future
rule found reading intra-batch order is a bug in the rule's encoding, not a
license to rely on the order.

## UD-7 — concession granularity — DECIDED (P0.W7, 2026-06-12)

**Coarse: decision-boundary.** USER RULING (W7 closeout, superseding the
W6 "accepted-but-never-offered" reading): [CR#104.3a]'s "at any time"
means a correct steppable engine ENUMERATES concession at every boundary
that emits choices — `Action::Concede` rides every priority legal list,
and `submit_decision` accepts it as the answer to EVERY pending decision
(the decider walks away mid-discard, mid-targeting, mid-payment).
Filtering it out of a UI or a bot's choices is the RUNNER's problem (and
leaving it in is kind of funny, which is a bonus). Granularity stays
coarse in one sense: the loss applies as a normal `PlayerLost` event at
the decision boundary — there is no sub-event interruption, and a player
who is NOT being asked anything conceding out-of-band remains a
runner-API seam. Revisit only if a real ruling demands finer grain.

## UD-11 — game-state equality — OPEN

No equality predicate exists; livelock is guarded by a turn-count ceiling in
`sim.rs`. The mandatory-loop draw rule ([CR#104.4b]) needs a real
state-equality definition (which components count, what is excluded — e.g.
the event log). Decide alongside loop detection.

W6 (2026-06-12) deliberately shipped no loop monitor: there is no trip
point to guard (a seam note rides `check_game_end` in `step.rs`). Working
lean for the eventual decision: the skill's *event-sequence equality*
alternative ([CR#104.4b]'s own "sequence of events" framing) detects loops
without defining whole-state identity — re-evaluate when the first
mandatory-loop fixture exists.

**P0.W7 closeout: formally RE-DEFERRED** — blocked on a fixture that
forces it; the working lean above stands.

## Assumed-category sweep (P0.W7, 2026-06-12) — UD-1 / UD-4 / UD-6 / UD-10

None embodied by P0; each waits on its forcing machinery:

- **UD-1** (snow-ness timing for banked mana): the W2 mana-unit grammar
  (`ManaProduction`/`ManaRider`) SHAPES the unit, but pool units are still
  plain counts (engine-seam) — the source-snapshot field that would decide
  this arrives with pool units.
- **UD-4** (cast-from-stack identity): untouched; adjacent W4 `CopySpell`
  is copy-on-stack, not casting. Decide with cast-a-copy ([CR#707.12]).
- **UD-6** (⊥ undefined values): documented convention only; formalize at
  the first ⊥ collision (conformance 3b row).
- **UD-10** (knowledge across rewinds): untouched; arrives with the
  [CR#733.1] payment-rewind machinery (post-P0 backlog).

## Settled-section implementation ledger (P0.W7)

The skill's settled entries, implement-or-note:

- **S1** (oracle paragraph segmentation): implemented long-standing — the
  card pipeline's extract stage segments by paragraph.
- **U2** (agent attribution for rules-performed events): implemented by
  the W3 cause triple — engine `Cause.agent` is `None` exactly for
  turn-based/state-based actions.
- **U3** (vigilance = procedure carve-out, settled-by-policy): implemented
  at alignment — vigilance is intrinsic; the attack tap checks the keyword
  in the declare-attackers procedure ([CR#508.1f]), no cause-tagged
  replacement.
- **U5** (can't-lose/can't-win base semantics): implemented W6 —
  `OutcomeGate` doc + SBA-sweep guard carry the
  precedence-not-consumption, per-SBA-survival reading.
