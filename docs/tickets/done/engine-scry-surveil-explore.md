---
needs: [engine-resolve-selections]
---
## INTEGRATED 2026-06-17 — Scry / Surveil / Fateseal (Explore split out)

Landed: the `Effect::With`→`Those` group binder, `Selection::{Those,TopOfLibrary}`,
`PlayerAction::Distribute` + `Bin` with the `PendingDecision::Distribute` ordered-
partition decision (direct VecDeque surgery for top/bottom — identity-preserving —
graveyard via the event path), identity-keyed `look_grants`, the Composite-style
`GameEvent::Distributed { name }`, `Reference::Opponent`, and the three
`macros/action/{Scry,Surveil,Fateseal}.ron` macros. **Explore was SPLIT OUT to
`engine-explore` (planned)** — it is a different shape (reveal-then-branch) and needs
the unbuilt Reveal seam. Correct CR numbers (the body below is STALE): Scry
[CR#701.22], Surveil [CR#701.25], Fateseal [CR#701.29], Explore [CR#701.44].

---

Look-at-top-of-library DECISION primitive — the single biggest graduation lever
left (~1,866 cards: Scry 1,307 + Surveil 455 + Explore 99 + Fateseal 5; ~100+
are one unparsed line away RIGHT NOW). The parse infrastructure is ready (the
effect-clause parser + keyword-action macros route here); the gate is a missing
engine decision.

Add a "look at the top N cards, then partition them" decision: the player sees
the top N and chooses an ordering / which go to a second zone. Specializations:
- **Scry N** [CR#701.18]: look at top N, put any number on the bottom (in any
  order), rest on top (in any order).
- **Surveil N** [CR#701.42]: look at top N, put any number into the graveyard,
  rest on top in any order. (= scry-to-graveyard.)
- **Explore** [CR#701.40]: reveal the top card; if it's a land, put it into hand;
  otherwise put a +1/+1 counter on the exploring creature and you choose
  top-or-graveyard for the revealed card.
- **Fateseal N** [CR#701.25]: scry an opponent's library.

Implement as a reorder/partition Decision (the runner already has ChooseObjects;
this is an ordered-partition over a peeked top-N window). Once landed, author the
Scry/Surveil/Explore/Fateseal keyword-action macros (`macros/action/`) over it —
each becomes a one-file macro, exactly like Investigate. Identified by the
macro-keyword-actions worker (the action macros were skipped pending this).
