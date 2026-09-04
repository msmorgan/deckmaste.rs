---
needs: []
---
`fixed_duration_phrase` accepts any `NounPhrase` as a temporal endpoint, so a
stray bare plural can be absorbed as a duration adjunct. That hole was one
half of the `Context Card deals 2 damage to each creatures.` rescue parse
found while landing the general prepositional phrase; the other half is
closed. Narrow the endpoint to the nominals that actually denote a duration,
without inventing a topical noun category. Standard constraints apply.

## Landing record

Folded into `english-v2-adjunct-licence-removal` by coordinator ruling
(2026-09-04): the adjunct-licence landing exposed three wrong analyses that
this hole admits, so the narrowing lands with it.

### What was narrowed

`fixed_duration_phrase` — the marker-less duration adjunct, the only duration
spelling with no preposition bounding it — now requires its endpoint to carry
the declared temporal licence:

```rust
fn temporal_endpoint_denotes_a_time(endpoint: &TemporalEndpoint) -> bool {
    let TemporalEndpoint::Reference(reference) = endpoint;
    matches!(
        locative_temporal_license_for_noun_phrase(reference),
        LocativeTemporalLicense::TemporalLicensed | LocativeTemporalLicense::OfAndTemporalLicensed
    )
}
```

It reads the existing `LocativeTemporalLicense` feature that already
distinguishes a time-denoting noun (`turn`, `step`, `beginning`, `end`) from
every other noun, so no noun list, lexeme name, card, or topical noun category
was invented; `coverage` classifies the new checker as
`declared_license_feature`, not a structural predicate. `until_duration_phrase`
and `restriction_turn` are untouched: they carry an overt marker (`until`,
`only during`) and cannot absorb a stray neighbour.

A first attempt gated the endpoint structurally instead (an overt nominal head,
rejecting a fused determinative and a zero-determiner nominal). It closed two
of the three wrong analyses and left Red Guardian, Super-Soldier reading
`that dealt damage` as a duration, because that phrase does have an overt head.
The declared-feature reading is what the ticket asks for and what holds.

### Negative witness

`Context Card deals 2 damage to each creatures.` — the rescue parse this
ticket names — no longer selects. It is asserted in
`crates/deckmaste_english_v2/tests/determiner_contract.rs`, in the loop that
previously asserted the same three sibling surfaces (`Destroy each creatures.`,
`Destroy one or more target creature.`, `Destroy up to one target creatures.`)
*did* reach selection through the duration reading. That loop is re-spelled to
rejection on the same witnesses rather than deleted.

### The three wrong identities

All three stop parsing, exactly as on the parent tip:

- On Wings of Gold — ` that` was claimed by the distal demonstrative as a
  duration on `you control`, splitting `and/or` into a clause coordination.
- Red Guardian, Super-Soldier — first ` that`, then (under the structural
  attempt) ` that dealt damage`, read as a duration on `controls`.
- Zedruu the Greathearted — ` that` read as a duration on `you own`.

### Numbers

Measured on `qmssqson-` (parent) against the working copy above `txwltrmx`;
lock `covered` count of the measured result tree 16,825. Parent lock SHA-256
`d7b889de6b5c1b79163ea5b3dfe9e18efde5bde1480aadcef4aa8f04b659148e` (49,474
lines); result lock SHA-256
`768840a5f72cfbd36d2864c3a9e2ad4a215bea671c41441a94e3d91a8be27553` (49,475
lines: +3 added, -2 retired).

- Selected/covered 16,824 → 16,825; parse failures 15,817 → 15,816;
  unresolved ties 0 → 0; internal failures 0 → 0; exception uses 0 → 0;
  roundtrip clean 16,825, mismatched 0; ownership gaps/overlaps 0.
- Newly covered 3: Veilstone Amulet
  (`41f9eacc…`), Display of Dominance (`be62af0f…`), Deepfathom Echo
  (`c03db827…`). Each takes a properly headed duration phrase (`this turn`,
  `until end of turn`) low inside an object-gap relative — the recorded
  attachment-misselection class, correct constituency.
- Retired 2, both previously covered only by the duration rescue. The
  obligation line lives in the claimed ticket's `## Landing record`:
  `retirement/re-coverage obligation: 5fbcf36b2e6006fefc9c94d13b8c7d165c436d22f6207f5e83dc03956232681c (Infectious Curse), 72b69a69a8a4add9165cd6a8f5b062801ad06247f3c48e126189904cdbac155d (Absorbing Man and Titania)`
- Selection census: unique 11,527 → 13,328; specificity-resolved 5,297 →
  3,497; exception-resolved 0 → 0. 1,809 units move specificity → unique as
  the spurious duration rival disappears; 7 move the other way.
- Winner-change census against the parent tip: 2,668 changed decisions and 648
  changed selected ordinals among units selected in both trees; 97 changed
  selected analyses or statuses, classified exhaustively as 66 "duration rival
  removed", 25 "adjunct lowered" (the licence removal), 5 status changes (the
  3 newly covered and the 2 retired), and 1 other — Sunfire Balm, an
  intransitive verb phrase becoming a base verb phrase, which is the `Cycle`
  frame addition below.
- `licensing_checker_permitted` 25 → 20; forbidden 0 → 0. Six licence readers
  deleted by the adjunct-licence removal
  (`transitive_head_licenses_{,non}prepositional_adjunct`,
  `transitive_participle_head_licenses_{,non}prepositional_adjunct`,
  `passive_predicate_licenses_{,non}prepositional_adjunct`); one added here
  (`temporal_endpoint_denotes_a_time`).
- Construction inventory unchanged: `expand` emits 47,076 construction origin
  lines on both trees.
- Performance advisory (8 workers; 1 concurrent codex executor plus this
  review on the host): coverage 82.891 s, 105,389 ns/B, load 8.92/9.59/9.48;
  ambiguity 78.361 s, 107,507 ns/B, load 7.78/9.21/9.37; roundtrip 76.073 s,
  103,154 ns/B, load 7.37/9.08/9.33; parent-tip ambiguity 95.966 s,
  114,645 ns/B, load 5.66/8.12/8.96. Per-byte cost falls about 8% against the
  parent, so the narrowing is a parse-time improvement. All exceed the
  16.260 s quiet-host ceiling under contention; reported, not a STOP.

### Deviations and additions

Three verb-frame additions, none in either ticket's letter, each required to
stop the narrowing from dropping coverage. Bare-nominal duration absorption was
standing in for a missing verb frame in 48 corpus units. The coordinator ruled
(2026-09-04) that a frame belongs to this landing if and only if it records an
attested printed-Oracle complement shape; each was verified against the corpus
and against the units it covers, and each covered unit's selected analysis was
read.

- **`Cycle`** (core verb) `Predicate([])` -> `Predicate([]), Predicate([ObjectNounPhrase])`.
  Attested witness: `When you cycle this card, put a flying counter on target
  creature you control.` (Avian Oddity). The object-nominal complement is the
  dominant printed shape - `cycle this card` 90, `cycle any card` 36, `cycle a
  card` 23, `cycles a card` 8, `cycle another card` 4, `cycle them` 26. The
  probe claims bytes 8..14 as `core-verb:Cycle` and ` this card` as a
  determined object nominal, not an adjunct.
  Retains 46 identities, every one a cycling card: Astral Slide, Avian Oddity,
  Choking Tethers, Complicate, Death Pulse, Decree of Annihilation, Decree of
  Justice, Decree of Pain, Decree of Savagery, Deem Worthy, Dismantling Wave,
  Drannith Healer, Drannith Stinger, Fleeting Aven, Flourishing Fox, Fractured
  Sanity, Gempalm Incinerator, Gempalm Polluter, Gempalm Strider, Howler's
  Heavy, Invigorating Boon, Krosan Tusker, Lightning Rift, Magmakin
  Artillerist, Prickly Marmoset, Primal Boost, Quakefoot Cyclops, Renewed
  Faith, Resounding Roar, Resounding Scream, Resounding Silence, Resounding
  Thunder, Resounding Wave, Sanctuary Smasher, Shefet Monitor, Slice and Dice,
  Snare Tactician, Solar Blast, Splendor Mare, Stabilizer, Stir the Sands,
  Stoic Champion, Sunfire Balm, Titanoth Rex, Vizier of Tumbling Sands, Void
  Beckoner. All share one analysis shape: `cycle`/`cycles` with a determined
  card nominal as its object, in a trigger prefix (`When you cycle this
  card, ...`, `Whenever a player cycles a card, ...`) or, for Stabilizer, a
  finite clause (`Players can't cycle cards.`).

- **`Explore`** (keyword-action stub) `Intransitive` ->
  `Custom([[], [ObjectNounPhrase]])`. The coordinator's premise was that the
  keyword action is intransitive and a transitive frame admits English that
  does not exist. The keyword action is indeed intransitive almost everywhere,
  but the transitive complement is attested in printed Oracle text, on one
  card, twice: Nicanzil, Current Conductor -
  `Whenever a creature you control explores a land card, you may put a land
  card from your hand onto the battlefield tapped.` and
  `Whenever a creature you control explores a nonland card, put a +1/+1
  counter on Nicanzil.` A corpus sweep of every `explores <NP>` surface returns
  exactly those two; everything else is intransitive
  (`target creature you control explores`, `it explores`, `explores again`).
  The complement is a real object nominal, not an absorbed adjunct: the probe
  claims ` a land card` as an indefinite article plus `lexeme:type/Land` plus
  `lexeme:CommonNoun/Card`, immediately after
  `lexeme:keyword_action/Explore/third_person_singular`. The intransitive frame
  is retained beside it.
  Retains 1 identity: Nicanzil, Current Conductor.

- **`Scry`** (keyword-action stub) `Custom([[], [Amount]])` ->
  `Custom([[], [Amount], [ObjectNounPhrase]])`. The coordinator's instruction
  was to correct the complement kind to a measure complement, or to find that
  the existing measure frame already suffices. Neither applies: the measure
  frame `[Amount]` already exists and already spells `Scry 2` / `scry X`, and
  the unit this addition covers has a noun-phrase complement, not a numeral -
  Eligeth, Crossroads Augur,
  `If you would scry a number of cards, draw that many cards instead.` The
  probe claims ` a number of cards` as an indefinite article,
  `CommonNoun/Number`, `Preposition/Of` and `CommonNoun/Card` plural - a
  relational noun phrase that no `Amount` reading covers. `scry a number of
  cards` is the only non-numeral scry complement in the corpus. The addition is
  therefore an object frame beside the measure frame, not instead of it.
  Retains 1 identity: Eligeth, Crossroads Augur.

Without all three the narrowing loses 50 identities instead of 2.

### Rulings taken

Both blockers were put to the coordinator and ruled on (2026-09-04).

1. **Retirement approved.** `Infectious Curse` and `Absorbing Man and
   Titania` were covered only by a wrong analysis, which CLAUDE.md classes as a
   defect, so retiring them is the correct ratchet direction. The lock is
   blessed with `--bless --retire`; the obligation line naming both identities
   is in the claimed ticket's `## Landing record`, and each obligation is
   routed to its owning planned ticket with the card, the sentence and the
   analysis that must select: `english-v2-target-verb-subject-selection`
   (Infectious Curse - a subject-gap relative whose head is the Target Verb)
   and `english-v2-relative-clause` (Absorbing Man and Titania - an object-gap
   relative whose body carries an auxiliary).
2. **Frames kept, conditionally, and the condition is met** - see Deviations
   and additions above. Two of the coordinator's premises needed correcting
   against the corpus: `Explore`'s transitive complement is attested (twice, on
   Nicanzil), and `Scry`'s added complement is a noun phrase rather than the
   numeral a measure frame would take, so the existing measure frame does not
   cover it.

### Assurance

Restored 0. Re-spelled 21 test contracts (6 directly, 15 through a delegate
under a no-deletion brief), across `environment.rs`, `parser/materialize.rs`,
`ability_logic.rs`, `determiner_contract.rs`, `nominal_grammar.rs`,
`parser.rs`, `predicate_grammar.rs`,
`deckmaste_construction_core/tests/builtin_v2_keyword_actions.rs` and
`xtask/src/english_v2/coverage.rs`. Renamed 1
(`any_one_is_one_closed_determiner_and_beats_the_generic_duration_rival` →
`..._with_no_generic_duration_rival`, because the rival it named no longer
exists). Ignored 0. Added 3 witnesses: the
`Context Card deals 2 damage to each creatures.` negative, `Scry 2.` and
`Draw two cards.` as positives replacing surfaces that turned out to be
ill-formed. Removed 0 — every witness sentence present before is still
exercised; per-target test counts are unchanged (nominal_grammar 27,
predicate_grammar 101, parser 39).

Several re-spells flip a witness from selecting to rejecting. Each such
sentence is ungrammatical and was reaching selection only through the hole
this ticket closes: `Connive target player.`, `Scry target player.`,
`Scry 2 target player.`, `Draw 2 cards.`, `Destroy two target creature.`,
`Search your library a creature card.`, `Discard up to one cards.`,
`Destroy each X target creatures.`, `Destroy each of X target creature.`,
`Destroy up to one target creatures.`,
`You didn't create this way a token.` Every one of them sits in a test whose
stated purpose is rejecting exactly that malformation, so the flip restores
the contract rather than weakening it.

STOP: 1 taken and resolved by coordinator ruling — the retirement and the frame additions above.
Glossary gap: none.
