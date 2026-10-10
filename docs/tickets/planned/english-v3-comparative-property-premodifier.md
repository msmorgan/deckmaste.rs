---
needs: []
---
# Read a comparative adjective before a property noun: return target creature card with lesser mana value / creatures with greater power

## Why

A property noun reads with a value after it (NP "a creature card with mana
value 3 or less" 2; "a creature with power greater than this creature's
power" 3, from `english-v3-postpositive-comparative-property`) and under a
determined superlative or *same* ("a creature with the greatest power" 1; "a
card with the same mana value" 1). A comparative adjective before the bare
property noun reads nowhere: NP "a creature card with lesser mana value" 0,
"a creature with lesser power" 0, "a creature with greater power" 0, "a
creature card with greater mana value" 0, "lesser mana value" 0, "a creature
with greater power than this creature" 0. Blocker report at trunk `ymqwzour`
(`target/english-v3/blockers-wave8.txt`): keys `at:lesser mana` 14,
`at:greater power` 8, failing tokens `mana` 17, `power` 12.
`cargo xtask english-v3 grep -i '\b(lesser|greater) (mana value|power|toughness)\b'`
finds **44** faces, all unread; **42** have that line as their only failing
line: *lesser* 32 (31 sole), *greater* 12. 30 follow *with*; 4 are *has
greater power*; 5 continue with *than* + NP.

## Goal

A scalar comparative adjective premodifies a property noun, the standard of
comparison left to context or given by a following *than* phrase: *with
lesser mana value*, *with greater power*, *has greater power than this
creature*, with byte-exact realisation.

## Analysis

CGEL Ch. 13 §4.1, p. 1126, "The double comparative *lesser*": [14ii] "*We
think this is a lesser risk than taking no action at all*"; "*Lesser* is
found only in attributive function". That backs *lesser* as an attributive
comparative and the *than* phrase after the head noun. That the property noun
then needs no Determiner (as it needs none before a value) is a project
ruling.

Existing routes (`crates/deckmaste_english_v3/src/declarations.rs`):
`PremodifiedNominal` (`[modifier: AdjectivePhrase, " ", head: Nominal]`,
exporting `PropertyValuePosition` through
`premodified_property_value_position(modifier.AdjectiveOrigin,
head.PropertyValuePosition)`); `MeasuredAttribute` (property noun + value);
the schema `DiscontinuousComparativeNominal` (`[governor: lexical(node), " ",
head: Nominal, " ", marker: lexical(Preposition), " ", complement:
AccusativePhrase]`) with its policies `DiscontinuousComparativeModifier`
(*same … as*) and `DiscontinuousComparativeDeterminer` (*more … than*).
Lexicon (`lexicon/core.ron`): `vocab:ScalarDegree/Greater`
(`ScalarComparativeUse: Yes`, Predicate frame *than* +
`ComparativeComplement`), `vocab:ScalarDegree/Lesser` (the same frame; no
`ScalarComparativeUse`), `lexeme:CommonNoun/Power` (`PropertyValuePosition:
First`).

Orchestrator rulings 2026-10-10:

- CP1. Refine the property-value route, no new Construction: a comparative
  adjective that declares the attributive scalar use fills the property
  noun's value from the left, through `PremodifiedNominal`'s
  `premodified_property_value_position` table or one added form on
  `MeasuredAttribute`. The result is the valued property Nominal that *with*
  and *have* already take (as *with mana value 3*).
- CP2. Licences are declared: the adjective's attributive scalar-comparative
  feature (`ScalarComparativeUse` or a sibling value) and the noun's
  `PropertyValuePosition`. *lesser* gets the attributive value only: its
  declared *than* Predicate frame (*is lesser than*) has 0 attested faces
  and CGEL calls it attributive-only; apply the pruning rule to that frame
  and report the measurement.
- CP3. The *than* phrase (5 faces) is one added instance row of
  `DiscontinuousComparativeNominal` whose policy reads the governor's
  declared comparative frame and `marker.ExpandedComparisonMarker =
  Ordering`. If it needs more than a row and a policy, land CP1 and return
  the *than* faces with evidence.
- CP4. One analysis per string. *with power greater than X* keeps exactly its
  Readings; *the greatest power* and *the same mana value* keep theirs. The
  twins to avoid: *greater* as an ordinary attributive adjective of a bare
  singular count noun (*with greater creature* must not start reading), and
  *lesser* premodifying *mana* inside *mana value*.
- CP5. Do not open the gate for every adjective: NP "a creature with
  different power" 0 and "a creature with equal power" 0 today, both
  unattested; they stay whatever the declared feature makes them. No check
  whose only job is to block them.
- CP6. Newly covered faces are gains. No Reading-count increase on a covered
  face is expected (0 of the 44 are covered); the 356-face control class
  below must keep its Readings. Report any increase with its shape.

## Witnesses

Each is the only failing line of its face in `blockers-wave8.json` (census
`ymqwzour`, 0 Readings). Fragment probes ran 2026-10-10 on the `ymqwzour`
binary (`--readings 60`); trunk then moved to `pxmykwnl` and the prebuilt
probe no longer loads plugin data, so (census) controls give the covered
line's `reading_count`. Re-probe on the claim parent first.

- Orah, Skyclave Hierophant: "Whenever Orah or another Cleric you control
  dies, return target Cleric card with lesser mana value from your graveyard
  to the battlefield."
- Riveteers Ascendancy: "Whenever you sacrifice a creature, you may return
  target creature card with lesser mana value from your graveyard to the
  battlefield tapped. Do this only once each turn."
- Shadowfax, Lord of Horses: "Whenever Shadowfax attacks, you may put a
  creature card with lesser power from your hand onto the battlefield tapped
  and attacking."
- Prehistoric Pet: "This creature can't be blocked by creatures with greater
  power."
- Death's Oasis (CP3): "Whenever a nontoken creature you control dies, mill
  two cards. Then return a creature card with lesser mana value than the
  creature that died from your graveyard to your hand."

Controls that must not change: Return to the Ranks "Return X target creature
cards with mana value 2 or less from your graveyard to the battlefield." 3
(census); Spitfire Handler "This creature can't block creatures with power
greater than this creature's power." 6 (census); Triumph of Gerrard "I, II —
Put a +1/+1 counter on target creature you control with the greatest power."
3 (census); Ashiok's Erasure "Your opponents can't cast spells with the same
name as the exiled card." 3 (census).

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching

    (?i)\b(lesser|greater|greatest|least|less|equal|same|highest|lowest) (mana value|power|toughness)\b|\b(mana value|power|toughness) (less|greater)\b

(356 faces) plus 300 random covered controls, `--workers 6
--samples-per-face 0`, compared with `cargo xtask english-v3 diff` against
the claim parent. If CP2 prunes *lesser*'s Predicate frame the selector also
takes every face containing `\blesser\b`. Zero lost; CP6 for increases. Full
`cargo test -p` and `cargo clippy -p … --tests -- -D warnings` for
`deckmaste_lexical_source` and `deckmaste_english_v3`, once, at the end; a
rebuild after a grammar edit takes about 2–3 minutes, the six test binaries
about 75 seconds. Tests assert the role (the comparative as the property
noun's value-bearing Premodifier; the *than* phrase as its Complement; the
CP4 negatives), never an incidental count or a Debug-printed tree.
Attestation by `cargo xtask english-v3 grep` only.

## Out of scope

- *greater power or toughness than Hulkling* (2 faces: coordinated property
  nouns under one comparative).
- *no other creature has greater power* (Getaway Glamer) if *no other*
  blocks it; *a creature with greater power also attacks* (Okk, two failing
  lines).
- Superlative fused heads (*the greatest*, *tied for greatest*) and
  `at:the most`.

## Landing record

To be written by the claimant (six items, `docs/tickets/README.md`).
