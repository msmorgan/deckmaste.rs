---
needs: []
---
**A `Type` declaration's body is read by v2.** Routed on 2026-10-08 from
`plugins-v2-keyword-body-defects`, whose type half could not land: the three
type declarations that confer something (`creature`, `land` and `instant` under
`plugins_v2/builtin/macros/types/`) still write raw v1-shaped conferrals
inside `TypeDef(…)`, and nothing in v2 reads them. Design-bearing (Lean first);
standard constraints apply.

## Why it is blocked today

- `Type` is a loader tag in v2 (`DECLARATION_KINDS` doc in
  `crates/deckmaste_semantics_v2/src/ron.rs`): v2's card-type position is
  `CardType`, and no ticket has reconciled the two names.
- Lean `Definition` (`lean/Semantics/Rules.lean`) has no type constructor and
  the mirror has no `TypeDef`, so `cargo xtask expansions` skips all ten
  `types/` declarations ("no kind of [Type] is a v2 syntax type, and the body
  (`TypeDef`) is not a registry node"). A re-spelling of their conferrals
  would have no expansion proof.

## The change

1. Decide the v2 shape of a card type's Registry Definition (name, whether it
   is a permanent type, its conferrals), Lean first, and mirror it.
2. Make `Type` declarations read through it, and add its head to
   `DEFINITION_HEADS` in `crates/xtask/src/expansions.rs` so `types/` prints.
3. Then re-spell the conferrals over helpers that expand to the same term:
   creature's Combatant role [CR#508.1a], land's play permission
   [CR#305.1], and the instant cast permission [CR#304.1]. A re-spelling
   leaves every expanded term identical.

## Landing record

Measured on `vsmtmmtt` (stages `nollknmo`, `wyszmuvv`, `rpwzkxpn`, `vsmtmmtt`).

1. **Census delta**: none. No English grammar, lexicon or spelling changed, so no
   census was run; covered faces and Readings are unchanged. Lost faces: none.
   `cargo xtask expansions`: 1703 declarations, 1670 printed / 33 skipped before
   (`pxtrzlvk`); 1706, 1683 printed / 23 skipped after (`vsmtmmtt`). The ten
   `types/` declarations moved from skipped to printed, the three new helpers
   print, and `diff -r` shows no other expansion changed.
2. **Witnesses**: the ticket's three conferrals, each proved by
   `cargo xtask lean definitions` (card types 10/10 prove `Definition.check =
   []`) and pinned in `lean/Semantics/Proofs/Rules.lean`: creature `combatant`
   (`okCreatureType`), land `playableFrom(yourHand, asSorcery)` (`okLandType`),
   instant `castableFrom(yourHand, asInstant)` (`okInstantType`). The re-spelling
   over the helpers left all ten `types/*.expanded` byte-identical to the raw
   spelling of `rpwzkxpn`.
3. **Spot checks** (the ten newly printed definitions, no faces): artifact,
   battle, enchantment, planeswalker `CardType(_, true, [])`; kindred, sorcery,
   dungeon `CardType(_, false, [])`; creature `CardType(Creature, true,
   [Property(Combatant(This))])`; land `CardType(Land, true, [Property(
   DeonticRule(You, Permit, [play], Agent, None, Counterpart(This), None,
   Play(hand of You, None, asSorcery, false, ItsOwnCost)))])`; instant the same
   with `cast`, `false` and `OnlyIf(HasPriority(You))`.
4. **Deviations and STOPs**: no STOP. The ticket's letter names v1 nodes that
   v2 never had; re-read against v2: `Role(Combatant)` has no v2 node, so a new
   `StaticSpec.combatant (subject)` carries it (the glossary's Combatant entry
   forbids spelling it as a permission to attack or block). `May(Play(…))` and
   `May(cast(…, window: InstantSpeed))` became the existing permit deontic rule
   with a `play` rider, its timing per the 2026-10-08 temporal ruling (`asSorcery`,
   `asInstant`); each now names the hand [CR#305.1,304.1], which v1 left out.
   v1 creature's `Conditionally(Matches(This, Permanent), …)` is dropped: a
   Combatant is a Permanent by definition, and the checker asserts no zone.
   Added `CardType.dungeon` (Lean, mirror, `normalizeTypes`, `allTypes`, not
   permanent, not ascribable), because the `dungeon` declaration must name a
   type [CR#205.2a,309.2c]. The `Type` family stays a loader tag (a card writes
   the `CardType` word), so no `cardTypeTerm` projection and an unchanged `Type`
   meta (a bodyless open type noun still reads). `permanentType` is not
   cross-checked against `CardType.permanent` (no new refusal, charter
   2026-10-06). The one-land-per-turn limit [CR#305.2] is per player, not a
   rider of this permission, and is not written; no type confers the
   sorcery-speed cast permission of noninstant spells [CR#302.1,307.1], as
   before. `lean definitions` gains a card-type family (`GeneratedDefinitions.
   CardTypes`). The §11 macro recount was not updated: it was already stale
   (465 `semantic_macro` lines before this landing against 417 recorded).
   Glossary: Registry Definition amended to list the card type.
5. **Tests** (on `vsmtmmtt`): `cargo test -p deckmaste_semantics_v2` 100 passed;
   `cargo test -p xtask` 406 passed, 1 ignored (existing); `cargo test -p
   deckmaste_construction_core` 468 passed; `cargo test -p
   deckmaste_lexical_source` 62 passed; `cargo clippy -p <each> --tests -- -D
   warnings` clean for all four; `lake build --wfail` clean; `cargo xtask lean
   check` 173/173 cards; `cargo xtask lean definitions` 708/708 (218 counters,
   461 subtypes, 19 designations, 10 card types); `cargo xtask facts check` up
   to date; `cargo xtask cite check` 0 stale (309.2c blessed). Counts: restored
   0, re-spelled 1 (`builtin_v2_types_load_with_exact_semantics_and_noun_surfaces`,
   its ten expected bodies from `TypeDef` to `CardType`), ignored 0, added 7
   Lean pins (`okArtifactType`, `okCreatureType`, `okLandType`,
   `okInstantType`, `okDungeonType`, `combatantIsTheRole`,
   `playableFromIsThePlayPermission`), removed 0.
   After the refresh onto the integrated `semantics-v2-keyword-ability-census`
   (claim `pxtrzlvk`), `lake build --wfail`, the four crate test suites and
   clippy runs, and `cite check` were rerun with the same outcomes; `lean
   check` and `lean definitions` were not rerun.
6. **Timing**: no census run. `cargo xtask lean check` 227.2 s, one `lake`
   process.
