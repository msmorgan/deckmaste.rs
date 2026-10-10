---
needs: []
---
**Linked Facets, declared relations, merge and a mana-cost term.** Ruling
2026-10-09 ("A linked ability reads back a Linked Facet of a Link Source",
`docs/decisions/semantics-v2.md` §7). Bodies seven census residues: emerge,
offering, harmonize, haunt, soulbond, cipher, mutate. Standard constraints
apply.

## Shape

- Renames: `PaidFacet` → `LinkedFacet`, `PaidCostName` → `LinkSource`
  (Lean `Phrase.lean`/`Words.lean`, the Rust mirror in
  `crates/deckmaste_semantics_v2/src/{phrase,words,ron}.rs`, xtask
  `lean_macros.rs`, `lean/CONTRACTS.md`, `Check/Refusal.lean`). The five
  facets, the macros `chosenAsPaid`, `timesPaid`, `keywordCostPaid`,
  `costWasPaid`, `paidCostRead` and facts gating stay.
- `LinkSource.byDeed (verb)`: the printed ability identified by what it does
  ("the exiled card" of an exiling ability) [CR#607.2a].
- New facets: `objects (verb)` (what the linked ability's named deed acted
  on: "the sacrificed creature", "the exiled cards", "creatures it devoured"
  [CR#702.82b]) and `taken : Bool` ("tribute wasn't paid" [CR#702.104b]). The
  facts column that gates facets extends to them; offering's row gains
  `paidCost := true`.
- Relations: `Definition.relation (label) (left : Kind) (right : Kind)`,
  declared under `plugins_v2/builtin/macros/relations/` as designations are;
  `Instruction.relate (subject) (label) (counterpart)`, `Predicate.related
  (label) (counterpart : Option NounPhrase)`, `NounPhrase.counterpart (label)
  (subject)`. The checker reads the declared kinds.
- `Instruction.merge (subject) (onto) (position : top | bottom)` [CR#730.2];
  `StaticSpec.addsTarget (subject) (description)`, a target the spell gains
  when its alternative cost is chosen [CR#702.140a].
- `CostShift.run` → `CostShift.bySymbols (cost : ManaCostTerm) (rises)
  (coloredOnly)`, `ManaCostTerm = lit (cost) | manaCostOf (object)`. Read
  `coloredOnly`'s meaning from its pins before touching it.

## Re-spells (same asserted outcome)

Every `PaidFacet`/`PaidCostName`/`CostShift.run` site: Lean cards and pins
(`Proofs/GiftVariants.lean`, `Proofs/LinkedChoiceReadback.lean`), RON
helpers under `plugins_v2/builtin/macros/instructions/`, the bench, canon,
and Rust tests. `emerge.ron` and `harmonize.ron` comments name the new facet.

## Bodies

- offering: `bySymbols(manaCostOf(linked(theAdditional,
  objects(sacrifice))))` [CR#702.48a,702.48c].
- emerge: `less(statOf(manaValue, linked(theAlternative,
  objects(sacrifice))))` [CR#702.119a]; "emerge from [quality]"
  [CR#702.119b].
- harmonize: the cost taps a creature, reduction by its power, the exile
  replacement [CR#702.180a].
- haunt: `exile(this)` then `relate(it, haunting, target(creature))`, and
  "the creature it haunts" by `counterpart` [CR#702.55a,702.55b].
- soulbond: the two pairing triggers and `related(paired, …)`
  [CR#702.95a,702.95b].
- cipher: exile encoded, `relate(it, encodedOn, …)`, the granted trigger
  [CR#702.99a,702.99b].
- mutate: alternative cost plus `addsTarget` ("a non-Human creature with
  the same owner") and `merge` [CR#702.140a,730.2]; the "mutates" trigger
  (37 non-keyword cards).

## Witnesses (canon, non-keyword)

Ghoulcaller Gisa ("the sacrificed creature's power"), Spell Queller ("the
exiled card" in a second ability, `byDeed`), a "the chosen creature" card,
a "whenever this creature mutates" card. Pins: `objects` of a deed the
source never performs is refused; `relate` against a declared kind mismatch
is refused; the renamed facets keep their existing pins' statements.

## Siblings

`semantics-v2-off-stack-statics` reads `objects` and `taken` for devour and
tribute, and `semantics-v2-modal-choice` reads `timesPaid` for escalate;
neither blocks. Refresh before integrate; expect conflicts in
`Words`/`Phrase`/`Abilities`.

## Landing record

Series `luqqkxpn`..`rusrqsrr`. Tests, `lean check` and `lean definitions` were
measured on `rusrqsrr` after the refresh that brought in
`english-v3-divided-as-you-choose`; the census on `znnxzluk` (the same series
before that refresh).

1. **Census delta**: the only English-visible change is the new keyword
   declaration `emergeFrom` (spelling "emerge from <Param(0)> <Param(1)>",
   surface "emerge from"). Full `cargo xtask english-v3 --all` with and
   without it on `znnxzluk`: 32828 faces, covered faces 26738 (8608 one,
   18130 multiple) before and after, 903257 Readings before and after; 0
   per-face Readings digests changed (Crabomination's chart grew; it stays
   `no`). Lost faces: none. Lean: `lean check` 200/200 cards (canon 192, nine
   of them this landing's witnesses; testing 8).
2. **Witnesses** (semantic canon cards, each proving `Card.check = []`):
   Mockery of Nature (emerge), Patron of the Akki (offering), Wild Ride
   (harmonize), Blind Hunter (haunt, "the creature it haunts"), Nearheath
   Pilgrim (soulbond, "paired"), Last Thoughts (cipher), Dreamtail Heron
   (mutate and "whenever this creature mutates"), Ghoulcaller Gisa ("the
   sacrificed creature's power"), Spell Queller ("the exiled card", `byDeed`).
   Emerge from [quality] [CR#702.119b] is pinned in Lean only
   (`Bodies.okEmergeFrom`): its one Vintage card, Crabomination, has an
   enters-trigger the model cannot yet write. Pins in
   `Semantics.Proofs.LinkedReadback` (28): `objects` of a deed the source never
   performs is refused (`badObjectsOfADeedTheSourceNeverPerforms`),
   `relate` against a declared kind mismatch is refused
   (`badRelateKindMismatch`), and one per keyword body.
3. **Spot checks**: emerge = alt cost `compound([cost,
   perform(sacrifice(a(creature)))])` and, while `keywordCostPaid(emerge)`,
   `Less(statOf(ManaValue, linkedObject(TheAlternative, sacrifice, thisSpell)))`;
   offering = `addedCost(perform(sacrifice(a(and([quality, permanent])))))`,
   `reducedBySymbols(thisSpell, manaCostOf(linkedObject(TheAdditional,
   sacrifice, thisSpell)))` and an instant-speed cast permission, both while
   the cost was paid [CR#601.3c]; harmonize = graveyard cast for `cost` and
   `tap(counted(upTo(1), untapped creature you control))`, `Less(powerOf(
   linkedObject(TheAlternative, tap, thisSpell)))`, and the exile
   replacement; haunt = class-guarded `exile(it)` then `relate(it, haunting,
   target(creature))`; Blind Hunter triggers on `dies(counterpart(haunting,
   this))`; soulbond = the two `may(relate(…, paired, …))` triggers guarded by
   `not(related(paired))`; Nearheath grants lifelink to `and([thisCreature,
   counterpart(paired, thisCreature)])` only while `related(paired)`; cipher =
   `relate(this, encodedOn, a(creatureYouControl))` and a grant to
   `counterpart(encodedOn, this)` that copies `the(and([card,
   related(encodedOn, thisCreature)]))` and may cast `that(Copy)`; mutate =
   alt cost, `addsTarget(thisSpell, non-Human creature with the same owner)`
   and, while mutate's cost was paid, the enter replacement
   `chooseOne(merge(thisSpell, linkedObject(TheAlternative, Target, thisSpell),
   Top), …Bottom)`; Gisa = `powerOf(linkedObject(ByDeed(sacrifice),
   sacrifice, this))`; Spell Queller = `act(ownerOf(linkedObject(ByDeed(exile),
   exile, this)), may(castWithoutPaying(that(Card))))`. Nearheath's first draft
   granted lifelink to itself unpaired; fixed before landing.
4. **Deviations and STOPs**: no STOPs. `coloredOnly`, read from its two pins
   (Alabaster Leech `bySymbols(lit [W], rises, ¬coloredOnly)`, Edgewalker
   `bySymbols(lit [W,B], ¬rises, coloredOnly)`), is Edgewalker's "this effect
   reduces only the amount of colored mana you pay"; its meaning is unchanged
   and documented. Beyond the ticket's letter:
   - `objects` carries its Link Source and a plurality (`objects (which)
     (deed) (plurality)`, `taken (which)`), as the other facets carry `which`;
     the number is the printed one ("the sacrificed creature" one, "the exiled
     cards" many). Refusals renamed with the facet: `linkedFacetNamed`,
     `linkedFacetKind`, `linkedChoiceInScope`; new `knownRelation`. The
     constructors `NounPhrase.paid`/`Amount.paid` keep their names.
   - `LinkSource.performs` refuses `objects` of a deed a `byDeed` source does
     not perform, or a keyword's own deed under another source.
   - Relations are a declaration family (`meta/Relation.ron`,
     `DeclarationKind`-free: `deckmaste_construction_core` and the lexicon do
     not read `relations/`), with `relationTable` generated by `cargo xtask
     facts`, a `Relations` family in `lean definitions`, and registry-node
     treatment in `lean macros`. `Predicate.related` with a counterpart
     uniquifies "the" ("the encoded card"), as `exiledWith` does
     [CR#702.99b,607.3].
   - `MergePosition = top | bottom` as the ticket says; mutate's controller
     chooses between them with `chooseOne` [CR#702.140c]. Mutate's target is
     read back as `linkedObject(TheAlternative, Target, …)`, since
     `Predicate.targets` reads what a spell targets, not what targets it.
   - `emergeFrom` is its own keyword declaration with facts row `EmergeFrom`;
     its grammar surface is "emerge from" (the declaration name derives from
     the surface). Construction-core inventories extended for it
     (`parameterized` 107→108, keyword templates 266→267).
   - Facts rows: Offering `paid_cost`; Harmonize regime `AtCasting`,
     functions on stack; Cipher regime `AtResolution`, no longer
     `functions_on_stack` (its stack part is a spell ability); `AddsTarget`
     joins the casting-regime statics in `static_regime`.
   - Ability deed `mutate` added to `abilityDeedFacts` for "whenever this
     creature mutates" [CR#702.140d].
   - Rust `reads.rs`: `Facts::relation_sides` (default object/object);
     `NounPhrase::Paid` now reads Player or Object by facet, where it fell to
     Object before (a pre-existing divergence from Lean's `.player`).
   - The "the chosen creature" witness is not written: a "chosen creature"
     linked across abilities (Close Encounter's additional cost, Tyrannical
     Pitlord's entry choice) needs a deed or facet for choosing an object,
     which no `Deed` names; every other "the chosen creature" card reads its
     own ability's target. Routed to a follow-up.
   - Devour's "it devoured" [CR#702.82b] and tribute's "tribute wasn't paid"
     [CR#702.104b]: `semantics-v2-off-stack-statics` had not integrated at the
     final refresh, so both readbacks are that sibling's follow-up
     (`linkedObjects(ByKeyword(devour), OfAbility(devour), …)`,
     `not(choiceWasTaken(ByKeyword(tribute), …))`).
   - No glossary gap: Linked Facet, Link Source, Relation and Merge are
     defined in `docs/contexts/game-model/CONTEXT.md`.
5. **Tests**: `lake build --wfail` clean (94 jobs); `cargo test -p
   deckmaste_semantics_v2` pass (47+7+1+4+45); `cargo test -p xtask` pass
   (398 passed, 1 ignored pre-existing; +4; +14); `cargo test -p
   deckmaste_construction_core` pass; `cargo test -p deckmaste_lexical_source`
   pass (28+36); `cargo clippy -p <crate> --tests -- -D warnings` clean for
   all four; `cargo xtask lean check` 200/200; `cargo xtask lean definitions`
   711/711 (3 relations new); `facts check`, `lean macros check` up to date;
   `cite check` 0 stale, 0 non-compliant. Counts: restored 0; re-spelled 6
   theorems (LinkedChoiceReadback `badKickerChoosesNoPlayer`,
   `badCostlessKeywordChoosesNoPlayer`, `badTheAdditionalChoosesNoPlayer`,
   `badFacetReadAsTheOtherKind`; Description's two `linkedFacetNamed` pins)
   and 5 definitions (`gift`, `kicker`, `giftCost` typed `LinkSource`;
   Alabaster Leech and Edgewalker on `bySymbols (lit …)`), same outcomes;
   ignored 0; added 28 Lean theorems, 3 Rust tests, 9 canon cards; removed 0.
6. **Timing**: one full census 4 m 04 s wall, 16 workers (`after`,
   `znnxzluk`).
