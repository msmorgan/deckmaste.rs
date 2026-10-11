---
needs: []
design: true
---
**Write the four rules-defined subtype conferrals in v2 syntax.** Four subtype
declarations carry an EMPTY `rules` list because the syntax to write their type
rules does not exist yet. `plugins-v2-subtypes-macro-only` derived every subtype
declaration's `Definition::Subtype(subtype:, rules:)` body from its meta and
converted the corpus, so the `rules` list is the only thing left: this ticket
teaches the syntax the three gaps below need, then fills the four lists. Each
declaration keeps its v1 core type-rule record verbatim as a comment under its
STOP, which is what those lists must reproduce in v2 terms. Standard
constraints apply.

## The four records, and where they are

| declaration | file |
| --- | --- |
| `equipment` | `plugins_v2/builtin/macros/subtypes/artifact/equipment.ron` |
| `fortification` | `plugins_v2/builtin/macros/subtypes/artifact/fortification.ron` |
| `aura` | `plugins_v2/builtin/macros/subtypes/enchantment/aura.ron` |
| `saga` | `plugins_v2/builtin/macros/subtypes/enchantment/saga.ron` |

Each is pinned by
`crates/deckmaste_construction_core/tests/builtin_v2_noncreature_subtypes.rs::rules_defined_conferrals_stay_on_their_subtype_declarations`,
which asserts the preserved record text and the derived body; the guard
`crates/xtask/src/facts/lean.rs::subtype_definitions_read` carries an empty
exception list, so nothing may go back to a non-`Definition` body.

## The three syntax gaps

Recorded verbatim by `semantics-v2-definition-bodies` (2026-09-08):

| declaration | what it needs |
| --- | --- |
| `equipment`, `fortification` | the [CR#301.5,301.6] host rule is a deontic over the `Attach` deed [CR#701.3a], whose `actFacts` row declares no patient role, so "…to a creature" has no role to bind |
| `aura` | [CR#704.5m] fires on "attached to an illegal object or player, OR is not attached"; `Predicate.attachment` spells only the second half |
| `saga` | [CR#714.4] reads the final chapter number off the Saga's own chapter abilities [CR#714.2d]; `Amount` has no aggregate over a card's chapter marks |

## Landing record

1. **Census delta** (measured on `zvpvuvul`): `cargo xtask lean definitions` 711 → 711
   definitions proved (218 counters, 461 subtypes, 19 designations, 10 card types,
   3 relations); the four rules-defined subtypes now prove with their rules written
   (Equipment 2, Fortification 2, Aura 1, Saga 3 conferrals) where they proved empty
   before. Bodied keyword abilities +2 (`enchant`, `readAhead`). Canon 232 → 234
   cards, 234/234 prove `Card.check = []`. Lost: none.
2. **Witnesses** (`zvpvuvul`, each proving `Card.check = []`): Aura — Boar Umbra;
   Equipment — Warrior's Sword; Saga — History of Benalia (new); read-ahead Saga —
   Love Song of Night and Day (new).
3. **Spot checks** (`wsskrpxl`): Equipment `Property(mayBeAttachedTo(thisEquipment,
   allOf(creature)))`; Equipment `StateBased(not attached, attachment(Unattached,
   thisEquipment))`; Fortification `mayBeAttachedTo(thisFortification, allOf(land))`;
   Fortification state-based unattach; Aura `StateBased(not attached,
   move(thisAura, battlefield, graveyard))`; Saga entry `onlyUnless(asEvent(enters,
   establish(entersWithCounters(…, 1, loreCounter))), hasKeyword(readAhead))`; Saga
   `TurnBased(precombatMainPhase, putCounters(1, loreCounter, thisSaga))`; Saga
   `StateBased(and([final chapter ≥ 1, lore ≥ finalChapterNumber]), sacrifice)`;
   read ahead's forbid-trigger over chapter abilities whose chapter ≠ lore count, the
   turn it entered; Enchant's `addsTarget`, `mayBeAttachedTo(thisAura,
   allOf(Param(0)))` and `asEvent(enters(thisAura, AnywhereBut([stack])),
   attach(thisAura, a(Param(0))))`.
4. **Deviations and STOPs.** No STOP stands. Enchant was first STOPped on
   `semantics-v2-subject-param-kind`, which integrated during refresh; the body was
   then written. Deviations from the rulings' letter: (a) Enchant's entry clause is
   one `attach(thisAura, a(Param(0)))` rather than `choose` then `that(…)`, because
   no pronoun reaches back over an argument that may be an object or a player, and it
   is limited to entering other than from the stack [CR#303.4f]; (b) the Aura spell's
   target is `StaticSpec.addsTarget`, whose checker now admits an object-or-player
   description [CR#115.1,702.5d], and Enchant's facts row takes regime `atCasting`;
   (c) Saga's state-based condition keeps v1's "with one or more chapter abilities"
   conjunct; (d) `Amount.aggregate` admits one object when the axis is multi-valued
   (`ProjAxis.multiValued`); (e) `xtask facts` reads subtype bodies through the
   builtin plugin's macros, as counters already were; (f) Equipment/Fortification's
   "not attached → unattached" is written as ruled, though for an unattached
   Equipment it changes nothing. Named residues: Saga's "not the source of a chapter
   ability that has triggered but not yet left the stack" [CR#704.5s] (as Siege's);
   read ahead's [CR#702.155b,714.3b] entry choice, which needs a number range with
   an `Amount` bound. Siblings `semantics-v2-counter-annihilation-sba` and
   `semantics-v2-devour-quality-keyword` had not integrated; the Saga row is written
   against the current `Conferral.stateBased`. No glossary gap found.
5. **Tests** (`zvpvuvul`): `cargo test -p deckmaste_semantics_v2` pass;
   `cargo test -p deckmaste_construction_core` pass; `cargo test -p xtask` pass after
   re-spelling one pin (407 + 4 + 14, 1 ignored pre-existing); `cargo clippy --tests
   -- -D warnings` clean for all three; `lake build --wfail`, `cargo xtask lean
   check`, `lean definitions`, `lean macros check`, `facts check`, `cite check` (0
   stale, 0 non-compliant) pass. Restored 0; re-spelled 2
   (`rules_defined_conferrals_stay_on_their_subtype_declarations`,
   `a_keyword_declaration_builds_its_wrapper`); ignored 0; added 13 Lean theorems
   (4 subtype definitions in `Proofs/Rules.lean`, 9 host/chapter/readAhead/Enchant
   in `Proofs/Attachments.lean`); removed 0.
6. **Timing**: `cargo xtask lean check` 828 seconds, one run, lake default workers on 24
   cores under shared load.
