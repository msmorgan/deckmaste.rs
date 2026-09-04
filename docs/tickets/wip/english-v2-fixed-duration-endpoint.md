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
lock `covered` count of the measured result tree 16,825.

- Selected/covered 16,824 → 16,825; parse failures 15,817 → 15,816;
  unresolved ties 0 → 0; internal failures 0 → 0; exception uses 0 → 0;
  roundtrip clean 16,825, mismatched 0; ownership gaps/overlaps 0.
- Newly covered 3: Veilstone Amulet
  (`41f9eacc…`), Display of Dominance (`be62af0f…`), Deepfathom Echo
  (`c03db827…`). Each takes a properly headed duration phrase (`this turn`,
  `until end of turn`) low inside an object-gap relative — the recorded
  attachment-misselection class, correct constituency.
- Lost 2, both previously covered only by the duration rescue:
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
stop the narrowing from dropping coverage. Bare-nominal duration absorption
was standing in for a missing transitive frame:

- `Cycle` (core verb) was declared `Predicate([])` only, so every
  `When you cycle this card` parsed only by absorbing `this card` as a
  duration. Adding `Predicate([ObjectNounPhrase])` restores 46 identities.
- `Explore` (keyword-action stub) `Intransitive` →
  `Custom([[], [ObjectNounPhrase]])`, for `explores a land card`.
- `Scry` (keyword-action stub) `Custom([[], [Amount]])` →
  `Custom([[], [Amount], [ObjectNounPhrase]])`, for
  `scry a number of cards`.

Without them the narrowing loses 50 identities instead of 2. All three are
transitive in printed Oracle text.

### Decision wanted (blocking, not integrated)

1. **The two retirements.** `Infectious Curse` needs `target` as a
   relative-clause verb (`Spells you cast that target enchanted player`), which
   is a homograph decision against the closed-class-single-owner ruling, since
   `target` is a TargetingMarker vocabulary member. `Absorbing Man and Titania`
   needs an object-gap relative over an auxiliary (`all damage that creature
   sources you control would deal`). Neither is reachable from this ticket.
   Both were covered only by a wrong analysis, so retiring them removes a
   defect rather than losing a reading — but a retirement is a coordinator
   ruling, so the lock is left at the parent tip content, unblessed.
2. **The three verb-frame additions.** They are outside both tickets' letter
   and were taken to avoid a 50-identity drop. If they are not wanted here,
   the narrowing has to land with them as a separate ticket first.

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

STOP: 1 taken, unresolved — the retirement and the frame additions above.
Glossary gap: none.
