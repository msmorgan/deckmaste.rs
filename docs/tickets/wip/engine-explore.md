---
needs: []
---
Explore [CR#701.44] — the last member of the look-at-top-of-library family, split
out of `engine-scry-surveil-explore` (Scry/Surveil/Fateseal landed there; Explore
was deferred because it is a different SHAPE — reveal-then-branch, not the ordered
partition that the `Distribute` primitive provides). ~99 cards.

To "explore" [CR#701.44a]: the permanent's controller REVEALS the top card of
their library (to all players); if a land card is revealed, put it into hand;
otherwise put a +1/+1 counter on the exploring permanent and the player MAY put the
revealed card into their graveyard.

Engine gaps (the landed partition primitive does NOT cover this):
- **Reveal seam** — `PlayerAction::Reveal` / `GameEvent::Revealed { objects, to }`
  are SHAPED but unbuilt (`todo!("P0.W6")` in `resolve/player_action.rs` and
  `step.rs`). Explore
  reveals the top card to all players, so this seam must be built first (emit
  `Revealed`; reveal-window lifetime [CR#701.20a]).
- **Branch on a revealed card's characteristic** — "if a land card is revealed": a
  conditional that tests the revealed card's type and forks the effect.
- **may-to-graveyard** — the optional "may put it into your graveyard" (a YesNo-style
  decision; the shell already exists).

Reuses existing primitives: `PutCounters` (+1/+1, built), `Move`/`PutInLibrary`,
and the keyword-action macro infra (`macros/action/`, exactly like the landed
Scry/Surveil/Fateseal macros). Once the Reveal seam + type-branch land, Explore is
a one-file `Explore.ron` macro. The look-grant model (a looker may see what they
revealed) is already in place from the look-and-distribute work.

Resolution (2026-07-19): the "Engine gaps" above were mostly already closed. The
**Reveal seam is built** — `PlayerAction::Reveal` emits `GameEvent::Revealed`
(`resolve/player_action.rs`), the apply being a deliberate no-op; the
`todo!("P0.W6")` markers were retired by a sibling change and only a stale
comment in `resolve/effect.rs` still named them (now corrected). The
**type-branch** (`If`/`Matches(It, Type("Land"))`) and **may-to-graveyard**
(`May`) also already existed. The ACTUAL gap: the keyword-action `Composite`
lowering (`composite_items`) is a CLOSED per-verb dispatch with no `Explore`
arm — so "one-file macro" was wrong. Delivered: `Explore.ron` (compose
`Reveal(It)` → `If(land ? Move(It,Hand) : PutCounters(This,+1/+1) +
May(Move(It,Graveyard)))`) PLUS a new `"Explore"` arm in `composite_items`
(mirrors the Scry/Surveil/Fateseal reorder arm; carries the body, emits the
`Act(Explore)` name-fact via `FinalizeAct{BodyRan}`; NO scry-0-style fizzle
since a permanent explores even if impossible, [CR#701.44b]). Behavioral tests
land in `resolve/effect.rs`. Full-card graduation of the ~99 explore cards
stays gated by each card's surrounding grammar (ETB/attack triggers, "target
creature explores"), out of scope here — the keyword action itself is complete.
