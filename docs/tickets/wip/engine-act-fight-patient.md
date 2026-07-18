---
needs: []
design: true
---
# engine-act-fight-patient — symmetric per-subject Fight trigger fact

Fix the `Fight` second-combatant gap inherited from the `Act` facet contract
(`docs/superpowers/specs/2026-07-16-engine-act-facet-contract-design.md`). The design fork the
contract left open is **resolved** — see the design spec:
**`docs/superpowers/specs/2026-07-18-fight-symmetric-trigger-design.md`**.

## Approved approach (was fork option (a), with (b) folded in)

Per-subject committed facts, uniform slot mapping (spec §"Approved design"):

- Read **both** fighters off the body at the resolve boundary (unify `fight_first_fighter`
  with the cards-side `fight_fighters`), emit **one committed `Act` per distinct fighter**
  (self-fight → one, [CR#701.14c]), grouped in one batch.
- **Delete** the `verb == "Fight"` matcher branch (`eval.rs:826-832`) → uniform
  `actor_matches(who, actor) && part_matches(on, object)`; aligns Rust to the Idris model
  (`idris_emit.rs:2693-2707`), which already maps `Act` uniformly.
- Add the missing **uniform** `Act` arm to `event_roles` (`trigger.rs:582-614`) so "it" binds
  per fighter — fixes the deeper bug that even a first-fighter fight trigger binds no
  `EventObject` today (Foe-Razer Regent's "counters on **it**" is currently unbindable).
- Drop the dead second slot from the filter twin (`Fight(pred)`, `"${0} fights"`) — folds fork
  option (b) in.
- Variant **3b**: widen engine-internal `GameEvent::Act.on` `Option → SmallVec<[ObjectId; 2]>`
  so the cant/replace lane ("creatures you control can't fight") is symmetric too.

Net: `verb == "Fight"` gone from the matcher (−1 branch, 0 added); every other verb
byte-identical. Correctness: symmetric fire, per-participant (Foe-Razer fires per your fighting
creature; self-fight once), 0-power fighters fire (subjects from body instructions, decoupled
from [CR#120.8]).

## Out of scope — separate tickets minted while grounding

- [[engine-damage-zero-no-event]] — effect lane emits spurious 0-amount `DamageDealt` ([CR#120.8]).
- [[engine-aggregate-fight-finalize]] — aggregate `Batch(n, Fight)` can never finalize (watch
  keys on `ZoneChange`; a fight moves nothing).
