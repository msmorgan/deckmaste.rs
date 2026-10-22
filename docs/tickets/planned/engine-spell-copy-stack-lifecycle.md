---
needs: []
---
**Engine: a copy of a spell makes its targets "become the target", and a spell
copy leaves the stack the way a spell does (to its owner's graveyard) before it
ceases to exist.**

Provenance: `docs/evidence/mtg-rules-review-2026-10-21/departures-engine.md` #19,
#36.

## 1. Copies never fire "becomes the target" (#19)

`crates/deckmaste_engine/src/step/stack.rs:20-79` `Copied::apply` clones
`targets: entry.targets.clone()` and returns only `GameEvent::Copied`; the only
`BecameTarget` producer is `decide/pending/cast.rs:143-154`, at announcement or
trigger placement. So ward, valiant and the like never fire for a copy. Reachable:
builtin `Storm.ron` (`CopySpell` AsIs), e.g. wizards Astral Steel's storm copies
never trigger Archive Dragon's ward; canon Phantasmal Bear.

A copy of a spell is itself a spell with the original's targets ([CR#707.10]),
put on the stack "with those targets" ([CR#707.10c]). Dormant Gomazoa ruling
(2010-06-15): "You become the target of a spell … when a spell that's targeting
you is copied and that target isn't changed." Angelic Cub ruling (2024-11-08):
copying a spell that targets it triggers its ability.

**Change:** `Copied::apply` emits `BecameTarget` for every target of the copy,
deduplicated per object as `cast.rs` does, with the copy as the spell. When the
"may choose new targets" copy path is wired (`resolve/player_action.rs:388-392`
returns `vec![]` for non-AsIs today), emit for every final target.
`engine-retarget-corners`' "an unchanged slot must not re-trigger" applies only to
redirecting an existing stack entry ([CR#115.7]).

## 2. Spell copies vanish instead of going to the graveyard (#36)

`resolve/mod.rs:174-179` (resolution) and `:200-210` (fizzle): "[CR#707.10a]: a
copy leaves the stack by CEASING to exist — no zone move", emitting
`AbilityResolved` / `AbilityCountered`; a countered copy likewise
(`step/stack.rs:145`). [CR#608.2n]: "As the final part of an instant or sorcery
spell's resolution, the spell is put into its owner's graveyard." [CR#701.6a]: "A
countered spell is put into its owner's graveyard." Then [CR#704.5e]: "If a copy
of a spell is in a zone other than the stack, it ceases to exist" (the same
state-based action [CR#707.10a] names). Only "spell put into a graveyard"
watchers or replacements see the difference.

**Change:** move a spell copy Stack→Graveyard with a `ZoneChange` like any spell
(resolution, counter, fizzle), and let the native SBA scan (`sba.rs:103` onward,
today stack-only) cease it there.

Cite corrections in the same landing:

- `AbilityCountered` doc (`event.rs:512-520`) cites [CR#608.2n] for fizzling; a
  fizzle is [CR#608.2b] (an intervening-if removal is [CR#608.2a]).
- The copy-cease sites cite [CR#704.5d] (the token rule) with [CR#707.10a]; use
  [CR#704.5e,707.10a]: `sba.rs:103, 115, 315, 921, 981, 997, 1060`,
  `event.rs:109`, `resolve/action.rs:176`, `crates/deckmaste_core/src/action.rs:146`,
  `crates/deckmaste_semantics/src/action.rs:169`,
  `crates/deckmaste_plugin/tests/no_dead_grammar.rs:263`. `Action::Cease` gets
  [CR#704.5d] only if token ceasing is routed through it.
- `crates/deckmaste_engine/src/stack.rs:106` ("vanishes instead of moving zones").

The tests at `crates/deckmaste_engine/tests/stack.rs:4041` ("a copy leaves the
stack by CEASING TO EXIST, never by moving") and `:4107` ("a COUNTERED copy also
ceases rather than moving to a graveyard"), and the docs at `:4213`, `:5158`, pin
the wrong model; rewrite them to assert the graveyard move and then the cease.
The counter-verb stamping on fizzles is `engine-resolution-illegal-targets`.

## Tests

- A storm copy targeting a creature with ward triggers ward.
- A resolved instant copy produces a Stack→Graveyard `ZoneChange` and is gone
  after the next SBA check.
