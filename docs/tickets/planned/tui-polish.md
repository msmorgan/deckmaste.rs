---
needs: [tui-decision-actions]
---
Polish once the client is playable: a game-log / history pane, a help overlay (key
bindings), combat highlighting (attackers / blockers / damage), a cue when the
perspective hands off between players, and a short usage example in the README (the
publish-prep "see it work" link). Stretch — not blocking a playable demo. Part of
tui-client.

Follow-ups carried over from the tui-decision-actions review (2026-06-13):
- During blocker *pairing* (a blocker chosen, picking the attacker it blocks), the
  attacking creatures aren't highlighted as candidates and the cursor isn't steered
  to them — `Interaction::candidates()` is empty while `pending.is_some()`, so the
  UI should derive/highlight live attackers (`state.combat`) during that step. (Part
  of "combat highlighting" above; an off-target pick is still caught by the engine.)
- Test coverage gaps: the keyboard blocker-pairing flow and the `ChooseTargets` path
  have no end-to-end coverage (the goblins/elves demo decks never multi-block and
  carry no targeted spells). Add a fixture/deck with a targeted spell and a forced
  multi-block, or a scripted integration test, to exercise `pair_with` and the
  Targets path through a real game.
