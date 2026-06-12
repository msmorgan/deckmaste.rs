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

## UD-7 — concession granularity — OPEN

Current implicit behavior: loss is recorded at event boundaries only (no
mid-resolution interruption); there is no concession verb at all yet
([CR#104.3a] is unimplemented — see the alignment audit's game-end slice).
Decide when the concession verb lands: coarse (event-boundary) is the
working assumption.

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
