---
needs: []
---
**Engine: every rules-facing controller read uses the derived (layer-2)
Controller, and a layer-2 control change makes the permanent summoning sick for
its new Controller.** Today a continuous control-changing effect
([CR#613.1b]; `Modification::SetController`, `layer.rs:1965-1969`) changes only
the layered view's `controller`, while most of the engine reads the base
`GameObject.controller`, and only the one-shot `ControlChanged` apply
(`step/player.rs:153-167`) sets `summoning_sick`.

Provenance: `docs/evidence/mtg-rules-review-2026-10-21/departures-engine.md` #2.
Witness: canon Avarice Totem (two `Continuously(… SetController …)` halves), and
any Control Magic-shaped static.

## Base reads at head (crates/deckmaste_engine/src)

- `step/mod.rs:1346-1350` (`begin_turn`): sickness clears for
  `self.objects.obj(id).controller == active`. [CR#302.6]: "under its
  controller's control continuously since their most recent turn began".
- `step/mod.rs:1424-1426` (untap TBA): `if obj.controller != active { continue; }`.
  [CR#502.3]: "the active player determines which permanents they control will
  untap".
- `trigger.rs:1038-1043` (watcher construction): `(o.source, o.controller, …)`.
  [CR#603.3a]: a triggered ability "is controlled by the player who controlled its
  source at the time it triggered".
- `activate.rs:587-596` (`Sacrifice` payability):
  `self.player(object.controller).object == sacrificer`. [CR#701.21a]: a player
  can't sacrifice "a permanent they don't control".
- `step/combat.rs:96` (lifelink payee): `g.objects.obj(source).controller`.
  [CR#702.15b]: "that source's controller".
- `target.rs:408-411` (`RelationPredicate::ControlledBy`):
  `state.objects.obj(id).controller`, so "creatures you control" misses a stolen
  creature.
- `lki.rs:57` (`LkiSnapshot::capture`): `controller: o.controller` (the snapshot
  side is carried by `engine-lki-robustness`).

There are about 200 non-test `.controller` reads in the crate; the sites above
are the ones the review traced, not an exhaustive list.

The test `stolen_creature_attacks_for_new_controller`
(`crates/deckmaste_engine/tests/layers.rs:804`) asserts a Grizzly Bears stolen by a
`SetController` continuous effect "can attack" for its new Controller that turn.
That contradicts [CR#302.6]; the test is rewritten, not kept.

## Change

1. Read the derived Controller (`state.layers().controller(id)`, or one view per
   step/scan) at every rules-facing read: the sites above, then an audit of the
   remaining `.controller` reads in `crates/deckmaste_engine/src`. A read that must
   stay base (the layer system's own input, the one-shot `ControlChanged` apply,
   off-battlefield objects whose controller is the owner per [CR#108.4a]) says why
   in a comment.
2. Model [CR#302.6] as continuity of the derived Controller: whenever the derived
   Controller of a battlefield permanent changes (detected by diffing layer-2
   results between rebuilds, or a stamped controller-since marker), the permanent
   is summoning sick for its new Controller until that player's next turn begins.
   The one-shot `ControlChanged` path and the continuous path go through the same
   rule.
3. Rewrite `stolen_creature_attacks_for_new_controller` to assert the stolen
   non-haste creature can NOT attack that turn and CAN on its new Controller's
   next turn; add one with haste that can attack at once.

## Tests

With Avarice Totem's exchange (or a locked `SetController` effect), the stolen
creature: untaps in its new Controller's untap step, not its owner's; has its
triggered abilities controlled by the new Controller; can be sacrificed by the new
Controller and not by the owner; pays lifelink to the new Controller; is matched by
the new Controller's "creatures you control"; and can't attack or use a
{T}-cost ability until its new Controller's next turn.

`engine-summoning-sickness-from-history` (planned) derives sickness from
`ControlChanged` history; that query inherits the same layer-2 gap and must use
the rule from step 2.
