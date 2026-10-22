---
needs: []
---
# The untap step's untap is the active player's deed

`DeonticAction::Untap` (`crates/deckmaste_core/src/deontic.rs:288-299`) has one patient slot,
and its doc comment says untapping "is a turn-based action, not a deed some object performs, so
there is no agent slot". That is wrong. [CR#502.3]: "the active player determines which
permanents they control will untap. Then they untap them all simultaneously." The untap has an
agent (the active player) and a decision point (which permanents untap).

Change: give `DeonticAction::Untap` an agent slot beside `what`, and model the untap step's
turn-based action as the active player's deed, with [CR#502.3]'s choice as its decision point.
Player-keyed caps (Winter Orb, Static Orb, Smoke) and "you may choose not to untap" (Rubinia
Soulsinger) need the agent. Reword the "no agent slot" doc comment, and the matching one in
`crates/deckmaste_semantics/src/deontic.rs:307` while that crate lives.

Decided: the Lean v3 workbench already writes the untap this way, and its shapes are the model
for this change: `doesntUntap` and `mayDeclineUntap` (`Definitions/Actions.lean`, `cant any
(untap …)`), the player-keyed cap `cant (actor …) (untap …)` (`Proofs/Zone.lean:406-411`), the
one-shot `skipUntap` (next untap step), and `partSkip … .untapStep` ("skip your untap step",
`Templates.lean`). Card authoring for these lands through the v3 mirror, not the Idris surface;
this ticket replaces `cards-untap-skip-authoring`, which targeted the deletion-bound
`idris_emit` path.

Provenance: `docs/evidence/mtg-rules-review-2026-10-21/departures-engine.md` #26.
