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
  are SHAPED but unbuilt (`todo!("P0.W6")` in `resolve.rs` + `step.rs`). Explore
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
