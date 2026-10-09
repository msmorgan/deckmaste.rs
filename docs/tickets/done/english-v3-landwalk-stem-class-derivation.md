---
needs: []
---
# Derive the landwalk stem class from the Land-category subtype declarations

Priority: medium.

## Why

Hygiene ticket; no new Construction. The landwalk landing
(`english-v3-landwalk-bound-suffix`, +175 faces) supplied the bound stems of
*swampwalk*, *forestwalk* … from a hand-written list in the lexicon.
Owner principle (2026-10-08): a word list in the lexicon that mirrors a
declaration class is wrong; the class must derive from the declarations.

Line numbers are those of `qsupksky`.

## Current state

`crates/deckmaste_lexical_source/lexicon/core.ron:4–11`:

```ron
// Declared singular noun forms supply the lowercase bound stems.
// Desertwalk is attested on Desert Nomads; only attested hosts belong here.
bound_keyword_stems: [
    "lexeme:land_subtype/swamp", "lexeme:land_subtype/forest",
    "lexeme:land_subtype/island", "lexeme:land_subtype/mountain",
    "lexeme:land_subtype/plains", "lexeme:land_subtype/desert",
    "lexeme:type/land",
],
```

The field is `Inventory::bound_keyword_stems`
(`crates/deckmaste_lexical_source/src/native.rs:28`), consumed at
`src/lib.rs:103` by `native::add_bound_keyword_stems` (`native.rs:37–83`),
which mints one Prefix-bound noun `"{owner}/bound-keyword-stem"` per listed
owner from its declared singular Word Form, lowercased, with the
`BoundKeywordStem` frame. The landwalk declaration
(`plugins_v2/builtin/macros/keyword_abilities/landwalk.ron`) declares
`grammar: FixedKeyword(surface: "landwalk", bound_suffix: (surface: "walk"))`;
`BoundSuffixGrammar` (`crates/deckmaste_construction_core/src/macro_def.rs:424–428`)
has only `surface`.

## Change

1. The landwalk keyword declaration states its stem class: Land subtypes plus
   the Land card type. Spell it as a declared field of `bound_suffix` (a new
   `BoundSuffixGrammar` field naming the class, e.g. land-subtype declarations
   plus the `land` Type declaration), not as a member list.
2. The loader derives the members: every `Subtype(category: Land, …)`
   declaration under `plugins_v2/builtin/macros/subtypes/land/` (17 today:
   Cave, Desert, Forest, Gate, Island, Lair, Locus, Mine, Mountain, Plains,
   Planet, Power-Plant, Sphere, Swamp, Tower, Town, Urza's) plus the
   `lexeme:type/land` lexeme (`plugins_v2/builtin/macros/types/land.ron`).
   Owners are the existing ids (`lexeme:land_subtype/<name>`, minted by
   `legacy/plugins.rs:40–44,528`), so stem ids stay
   `"{owner}/bound-keyword-stem"`.
3. Delete `bound_keyword_stems` from `core.ron` and the `Inventory` field.
   Re-spell `native.rs`'s bound-stem unit tests (`:85–140`) against the
   derived class; delete none.
4. Update the **Bound Keyword Stem** glossary entry
   (`docs/contexts/oracle-english/CONTEXT.md:1269–1274`) to say the class is
   derived from the keyword's declared stem class.

### Recorded tension (resolved by orchestrator, 2026-10-08)

The [pruning clarification](../../decisions/english-lexical-analysis.md#pruning-rule-for-licensed-functions-and-attachments)
exempts lexemes mechanically minted from a declaration class from attestation;
the ticket proceeds with derived per-stem lexemes and unchanged stem identities.

The derived class adds 11 stems with zero supported-corpus tokens (no
*cavewalk*, *gatewalk*, *urza'swalk* … at minting; the only `-walk` words in
supported Oracle text are the six attested land types, *landwalk* and their
quality forms). The `core.ron` comment and the ADR pruning rule ("a lexeme
with zero tokens in the supported corpus is deleted") pointed the other way.
This ticket applies the 2026-10-08 owner principle (class from declarations)
as the later, more specific ruling; if that reading is wrong the alternative is
a declared attestation filter, which this ticket does not build. An implementer
who finds the two rulings irreconcilable STOPs.

## Expected census

Identity on the landwalk faces: 0 lost, 0 count changes, and 0 digest changes
on the 175 faces the landwalk landing gained (its manifest:
`target/english-v3/english-v3-landwalk-bound-suffix/`, or every supported face
containing *walk* plus 300 random covered faces). Stem lexeme ids do not
change, so any digest change is a defect to explain (e.g. lexeme insertion
order leaking into Reading identity), not an expected consequence.

## Method

Standard constraints apply. Deltas:

1. Verification: bounded census of every face the touched lexemes can reach
   (every supported face containing *walk*) plus 300 random covered faces,
   compared with `cargo xtask english-v3 diff` against the claim parent on the
   same manifest. Expected gain 0; zero lost faces; zero count drops.
2. Probe-first: Sheoldred's "Swampwalk", Desert Nomads' "Desertwalk", Goblin
   King's "… have mountainwalk." keep their counts; write any new negative as a
   test first; test once at the end.
3. Tests and clippy on the crates touched (`deckmaste_lexical_source`,
   `deckmaste_construction_core`, `deckmaste_english_v3`). The plugin
   definition changes, so `cargo xtask lean check` runs once.
4. No guard may name a lexeme, construction, verb, noun, preposition or card;
   read declared features only. Tests are re-spelled, never deleted.
5. Record = the six items in CLAUDE.md.

## Out of scope

Route to recon: the landwalk quality residuals, still unread after the landwalk
landing (supported faces at minting, surface count): *snow swampwalk* (Legions
of Lim-Dûl), *snow forestwalk* (Rime Dryad), *snow landwalk* (Barbarian
Guides, Zombie Musher), *legendary landwalk* (Ayumi, the Last Visitor; Livonya
Silone), *nonbasic landwalk* (Dryad Sophisticate, Trailblazer's Boots),
*landwalk of the chosen type* (Giant Slug, Illusionary Presence, Traveler's
Cloak): 11 faces by surface match; the orchestrator's count of 15 was not
reproduced at minting.

## Landing record

1. **Census delta** (`wtnuopxm`, before/after): claim-parent-equivalent `before.json` was measured on the empty child of `xztlzrpk`; the same bounded manifest has 1,345 faces (every supported face containing “walk” plus 300 covered controls, seed 20261008; 8 overlaps). Covered faces 1,074 → 1,074; total Readings 154,828 → 154,828. Built-in `english-v3 diff`: lost/gained/decreases/increases/digest-only/source changes all 0; every selected face is identical. Lost faces: none. Complete roundtrip/traversal validation, internal failures and validation issues: 0. Evidence: `target/english-v3/english-v3-landwalk-stem-class-derivation/`.
2. **Witnesses** (`wtnuopxm`): Sheoldred, Whispering One’s *Swampwalk*: 1 Reading; Desert Nomads’ *Desertwalk*: 1; Goblin King’s *Other Goblins get +1/+1 and have mountainwalk.*: 2. Before/after probes enumerate completely with no validation issues and unchanged counts.
3. **Spot checks** (`wtnuopxm`): no newly covered faces; ten unchanged faces each retain 1 Reading and exact realization. Anaconda: bound swamp + walk; Benthic Behemoth: island + walk; Cat Warriors: forest + walk; Righteous Avengers: plains + walk; Canyon Wildcat: mountain + walk; Bayou Dragonfly: flying and bound swampwalk in one keyword line; Merfolk Raiders: islandwalk followed by phasing; Stalker Hag: coordinated bound swampwalk and forestwalk; Lurking Crocodile: bloodthirst followed by bound islandwalk; Inkwell Leviathan: trample, bound islandwalk and shroud. All landwalk constituents use `BoundQualityKeyword` with the preserved lexical owners.
4. **Deviations and STOPs**: no Construction or frame added or removed. The additional class-selection/deduplication test protects category/type selection and shared-class membership. The earlier lexical-model STOP is resolved by the orchestrator ruling of 2026-10-08: use derived per-stem lexemes; the pruning clarification is recorded in the ADR. Per-form binding on an existing lexeme—distinct free/bound form identity under one owner, form-specific binding and frame lookup—was the preferred shape and is routed to recon as [a lexical-model item](../planned/english-v3-lexical-model-bound-forms.md). The authored member list is retired; the keyword declaration supplies the class. Glossary gap: none; Bound Keyword Stem now records class derivation. At `wtnuopxm`, 17 Land subtype declarations plus the Land type supply 18 stems, 11 additional; supported-corpus grep of those additional formations finds 0 faces and 0 matches, permitted by the ruling.
5. **Tests** (`wtnuopxm`): `cargo test -p deckmaste_english_v3` passed; `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings` passed; `cargo test -p deckmaste_lexical_source` passed; `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings` passed; `cargo test -p deckmaste_construction_core` passed; `cargo clippy -p deckmaste_construction_core --tests -- -D warnings` passed; `cargo xtask lean check` passed. Changed-test accounting: restored 0; re-spelled 4 (the two native stem tests, the template-head bound-suffix fixture, and the builtin landwalk declaration test); ignored with blockers 0; added 1 (class selection/deduplication); removed 0. Existing structural/value assertions and both roundtrip laws are preserved.
6. **Timing** (`wtnuopxm`): final bounded census wall time 44.038 s with 12 workers (setup plus corpus; compilation excluded).
