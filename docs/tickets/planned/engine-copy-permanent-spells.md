---
needs: [engine-copy-spells]
---
Copies of PERMANENT spells. A resolving permanent-spell copy ceases being a
copy of a spell and becomes a token permanent [CR#707.10f] (needs the
token-permanence handoff at resolution — today a copy always vanishes on
resolution). Double-faced permanent-spell copies carry both faces
[CR#707.10g] — blocked in practice on transform support.

## Interim behaviour and the pinned test (added 2026-10-21)

At head `crates/deckmaste_engine/src/resolve/mod.rs:119-126` (`if entry.copy &&
self.is_permanent_spell(spell)`) emits only `AbilityResolved` and nothing enters:
"it vanishes like any other copy ([CR#707.10a])". [CR#707.10a] covers only a copy
"in a zone other than the stack", so it does not license deleting a resolving
copy. The test `resolved_permanent_copy_vanishes_without_entering_battlefield`
(`crates/deckmaste_engine/tests/stack.rs:4304`) pins the vanishing; it is
rewritten to assert the token enters. Reachable through the testing fixture
`plugins/testing/cards/Creature tap-activated CopySpell Target Spell.ron`.

The handoff: as the copy resolves it becomes a token permanent ([CR#608.3f]) that
is not "created" for replacements or triggers that refer to creating a token
([CR#111.13]). Land the handoff rather than refusing to copy permanent spells
(refusing is itself a departure from [CR#707.10]). Provenance:
`docs/evidence/mtg-rules-review-2026-10-21/departures-engine.md` #11.
