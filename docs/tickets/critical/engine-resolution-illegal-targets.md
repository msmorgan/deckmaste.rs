---
needs: []
---
**Engine: a resolving spell or ability does not affect, or read information
from, a target that has become illegal; and a spell or ability that does not
resolve because all its targets are illegal is not "countered".** Part 1 is
reachable with canon Pounce, Arc Lightning and Fate Transfer.

Provenance: `docs/evidence/mtg-rules-review-2026-10-21/departures-engine.md` #13,
#12. `done/engine-target-distinctness.md` §3 specified "illegal targets are
excluded from `Target(n)`/plural reads"; only departed-target exclusion landed.

## 1. Illegal targets are still read (#13)

- `crates/deckmaste_engine/src/resolve/targets.rs:28` `targets_still_legal(&self,
  entry) -> bool` computes legality per target but returns one bool
  (`:84-104`); its doc (`:20-22`) says "(the illegal targets are then excluded
  from the reads)".
- `resolve/query.rs:94-99` `live_target_slot` filters only departed objects
  (`self.objects.get(target).is_some()`).
- The unpruned `&entry.targets` reaches `resolution_region_frame` and
  `cast::announced_effect_items` at `resolve/mod.rs:166-196` (spells),
  `:303-312` (triggered), `:332-349` (activated).

So a target that gains hexproof or protection, changes controller, or stops
matching its filter while staying on the battlefield is still affected (Pounce
still fights; Arc Lightning still damages a target that gained protection).
[CR#608.2b]: "Illegal targets, if any, won't be affected by parts of a resolving
spell's effect for which they're illegal", and a part that needs information
about an illegal target "fails to determine any such information".

**Change:** return per-slot legality and pass a pruned target set (illegal
members read as nothing) into the frame and the effect items.

**Check alongside:** the permanent-spell branch (`resolve/mod.rs:126-159`, an
Aura's `attach_to` from `entry.targets`) never calls `targets_still_legal`.
[CR#608.3b]: a permanent spell with an illegal target, other than a bestowed Aura
or mutating creature spell, doesn't resolve. Verify with canon Pacifism and fix
here if it reproduces.

## 2. A fizzle is stamped as a counter (#12)

Four sites stamp targets-illegal removal `Cause::counter(Agency::StateBasedAction,
None)`: `resolve/mod.rs:201-210` (copy, `AbilityCountered`), `:211-224` (spell,
Stack→Graveyard `ZoneChange`), `:315-322` (triggered), `:353-360` (activated).
`event.rs:103-107` documents `Cause::counter` as the cause a "becomes countered"
view narrows by, and a real `Action::Counter` (`resolve/action.rs:117-135`)
produces the same shape. [CR#608.2b]: the spell or ability "doesn't resolve. It's
removed from the stack and, if it's a spell, put into its owner's graveyard";
nothing counters it ([CR#701.6a] is the counter verb). [CR#704.3] confines
state-based actions to the priority check, so the SBA agency is wrong too.
Multani's Presence ruling (2018-04-27): "A spell that doesn't resolve because its
targets are illegal isn't countered, so this ability won't trigger." Latent: no
countered-event view exists yet and Multani's Presence is `.todo`.

**Change:** give targets-illegal removal its own cause (a "does not resolve" verb
under a resolution agency) at all four sites; keep "Counter" for
`Action::Counter`.

## Tests

- Arc Lightning on two creatures, one gains protection from red before
  resolution: only the other is dealt damage.
- Pounce, the opponent's creature gains hexproof: no fight, no damage.
- A spell whose only target became illegal goes to the graveyard and does not
  fire a "whenever a spell you control is countered" trigger.
