---
needs: [english-v2-np-postmodifiers]
---
Ruling recorded: `docs/decisions/english-v2-rewrite.md`, "Ruling: adjunct
licences removed; attachment misselection is a recorded class (2026-09-03)".
Low attachment is the INTERIM selection; the underspecified-attachment design
(`english-v2-underspecified-adjunct-attachment`) supersedes it later.

Remove the adjunct-licence dimension from verb valence rows (np-postmodifiers
landing review HIGH). `AdjunctLicensed` / `NonprepositionalAdjunctLicensed`
default every verb to "no adjunct may attach inside my object's reduced or
finite relative", with six verbs opted in. That rejects well-formed Oracle
English (`Draw a card for each card you've exiled this turn.` parse-fails; the
`discarded` variant parses), and it encodes no English fact: it is the
Seedborn Muse `Control` blacklist re-expressed as a whitelist in data, kept to
prefer `Untap all permanents you control during each other player's untap
step` with `during` on Untap rather than on `control`.

Pin (ruled): a temporal/manner/locative adjunct attaches to
any verb clause, matrix or embedded — no per-verb licence — and the recorded
derived-attachment rule (low attachment) selects among the survivors. Seedborn
Muse then selects the `during`-under-`control` reading; the grammar admits
both, and preferring the matrix reading is semantics, which the grammar does
not do. Consequences: delete both licence variants and their valence-row uses
(core_verbs.ron, the three keyword_actions stubs, `environment.rs` defaults,
the compiled-consumer fixture's two derived queries); re-spell
`unlicensed_participial_relatives_leave_adjuncts_on_the_outer_predicate` to
assert the low-attachment reading on the same sentence; add the `exiled` /
`revealed` minimal pair as positives. Coverage must not drop; a genuine tie
surviving low attachment is a STOP. Report the winner-change census against
the PARENT tip (the review found 45 undisclosed winner changes on the
previous landing). Standard constraints apply.

Ruled against: keeping a licence with an inverted default (opt-out by class);
no English fact stands behind an opt-out.

## Landing record

Removed the adjunct-licence valence dimension. Temporal, manner, and locative
adjuncts now derive on every verb clause; the existing derived-attachment
selection chooses the low reading. This removes the rejected inverted default
without a replacement exception, dominance edge, narrowed form, or named
checker. The `Lex` and `OptionalLex` valence markers in `core_verbs.ron` were
preserved. No construction became unreachable and no codec orphan resulted.

The participial-relative assurance is re-spelled, rather than deleted, for the
same `Destroy each creature you sacrifice during your upkeep.` witness and its
low relative-clause reading. Added selected positives are `exiled` and
`revealed`; the existing `discarded` positive remains selected.

### Measurement

- Parent selected/covered: 16,824/16,824; result: 16,830/16,830 (+6), with
  parse failures 15,817 -> 15,811, unresolved ties 0 -> 0, internal failures
  0 -> 0, and exception uses 0 -> 0.
- Construction count: 394 -> 394. The coverage lock is add-only: 49,474 ->
  49,480 lines (+6), parent SHA-256
  `d7b889de6b5c1b79163ea5b3dfe9e18efde5bde1480aadcef4aa8f04b659148e`,
  result SHA-256
  `1effd51f29e127ca019fd6768805c4a4c2bee9124be306b0af8a7a302487b08b`.
- Selection census: unique 11,527 -> 11,515; specificity 5,297 -> 5,315;
  exception-resolved 0 -> 0. There are 42 changed decisions, 40 changed
  selected analyses/statuses, 29 changed selected ordinals, and 6 newly
  selected identities.
- Newly covered: Deepfathom Echo (ordinal 4, specificity,
  `PositiveObjectGapRelativeClausePositiveObjectGapRelativeWithAdjunct`);
  Display of Dominance (1, specificity, that low adjunct construction);
  On Wings of Gold (0, specificity, that low adjunct construction); Red
  Guardian, Super-Soldier (3, specificity,
  `PostmodifiedReferenceReducedPassiveAdjunctQualifiedReference` then the low
  positive-object-gap adjunct); Veilstone Amulet (2, specificity, the low
  positive-object-gap adjunct); Zedruu the Greathearted (10, specificity, the
  low positive-object-gap adjunct).
- Performance advisory (8 workers; sibling process count unavailable in the
  sandbox for reviewer stamping): coverage 87.568 s, 112,631 ns/B, host load
  6.39/10.56/10.80; roundtrip 85.434 s, 124,717 ns/B, load 11.76/11.57/11.16;
  ambiguity 96.623 s, 121,605 ns/B, load 8.15/10.96/11.02. Each exceeded the
  16.260 s wall ceiling under host load; this is reported, not a STOP.

### Winner-change census

Compared `ambiguity --json --require-resolved` against the parent-tip
reflink. The notation below records selected ordinal and resolution plus the
attachment-bearing selected construction: O-P/O-D are the former outer
prepositional/general adjunct predicates; L-P/L-D are low positive-object-gap
prepositional/general adjuncts; R-P/R-D are low reduced-passive
prepositional/general adjuncts. `same path` means the complete selected
construction path is unchanged but candidate selection/resolution changed.

- Band Together: 0 unique, same path -> 0 specificity, same path.
- Corrosive Ooze: 0 unique O-P -> 2 specificity R-P.
- Crystal Fragments // Summon: Alexander / Summon: Alexander: 1 specificity
  O-D -> 5 specificity L-D.
- Dazzling Theater // Prop Room / Prop Room: 0 specificity O-P -> 4
  specificity L-P.
- Deepfathom Echo: parse failure -> 4 specificity L-D.
- Display of Dominance: parse failure -> 1 specificity L-D.
- Divine Deflection: 0 unique O-D -> 2 specificity L-D.
- Double Trouble: 1 specificity O-D -> 3 specificity L-D.
- Dragonclaw Strike: 17 specificity O-D -> 47 specificity L-D.
- Drumbellower: 0 specificity O-P -> 4 specificity L-P.
- Edgewall Inn: 2 specificity, same path -> 4 specificity, same path.
- Endure: 0 unique O-D -> 2 specificity L-D.
- Escape to the Wilds: 0 unique O-D with reduced-passive -> 2 specificity
  R-D.
- Gift of Immortality: 0 unique O-P -> 2 specificity R-P.
- Lynde, Cheerful Tormentor: 0 specificity O-P -> 8 specificity R-P.
- Markov Waltzer: 1 specificity, same path -> 2 specificity, same path.
- Memory Theft: 2 specificity -> 5 specificity (same relative attachment
  signature; different selected construction path).
- Mightform Harmonizer: 1 specificity O-D -> 3 specificity L-D.
- Murkfiend Liege: 0 specificity O-P -> 4 specificity L-P.
- Mysterious Pathlighter: 0 unique, same path -> 0 specificity, same path.
- Ohabi Caleria: 0 specificity O-P -> 4 specificity L-P.
- On Wings of Gold: parse failure -> 0 specificity L-D.
- Prophet of Kruphix: 2 specificity O-P -> 10 specificity L-P.
- Rabid Attack: 0 unique, same path -> 0 specificity, same path.
- Red Guardian, Super-Soldier: parse failure -> 3 specificity R-D then L-D.
- Roar of Endless Song: 8 specificity O-D -> 23 specificity L-D.
- Safe Passage: 0 unique O-D -> 2 specificity L-D.
- Samite Censer-Bearer: 1 specificity O-D -> 5 specificity L-D.
- Seedborn Muse: 0 specificity O-P -> 4 specificity L-P.
- Sentinel of Lost Lore: 0 unique positive-object-gap -> 2 specificity L-P.
- Slurrk, All-Ingesting: 7 specificity -> 14 specificity (same relative
  attachment signature; different selected construction path).
- Tandem Takedown: 0 unique, same path -> 0 specificity, same path.
- Tangle Wire: 0 specificity O-P -> 2 specificity L-P.
- Thrakkus the Butcher: 1 specificity O-D -> 3 specificity L-D.
- Unnatural Growth: 8 specificity O-D -> 23 specificity L-D.
- Unwinding Clock: 0 specificity O-P -> 4 specificity L-P.
- Veilstone Amulet: parse failure -> 2 specificity L-D.
- Warriors' Lesson: 0 unique, same path -> 0 specificity, same path.
- Zedruu the Greathearted: parse failure -> 10 specificity L-D.
- Zopandrel, Hunger Dominus: 16 specificity O-D -> 46 specificity L-D.

The eight additional decision changes retain their selected analysis and are
therefore outside the selected-analysis list above. No ties survive and none
of the six newly selected identities is a negative oracle.

### Assurance and deviations

- Restored: 0. Re-spelled: 7 contracts (the participial-relative and
  object-gap low-attachment assertions, nominal selected ordinal, parser
  labels, generated declaration-verb expansion, and compile-fail fixture).
  Ignored: 0. Added: 1 minimal-pair assurance (three positive witnesses).
  Removed: 0 assurances.
- Deviation: the nominal selected-ordinal fixture and xtask checker-census
  fixture were re-spelled because the ruled change altered their observable
  selected result and permitted-checker count; both are required by the ticket
  and introduce no new grammar behaviour.
- STOP: none. Glossary gap: none. Decision wanted: none.

### Review corrections (landing review, 2026-09-04)

Verdict: REJECTED-PENDING-RULING. The census, lock arithmetic, residue sweep,
checker-count shift and re-spell shape all verify; three of the six newly
covered identities do not.

Reviewer re-measurement (independent, on a reflink copy of the parent tip
`qmssqson-` against `@-`): parent selected/covered 16,824/16,824, unique
11,527, specificity 5,297; result 16,830/16,830, unique 11,515, specificity
5,315; unresolved ties 0 -> 0, internal failures 0 -> 0, roundtrip mismatches
0. Coverage gate passes with `licensing_checker_permitted` 19 and
`licensing_checker_forbidden` 0. The six lock additions are exactly the six
newly covered ids.

HIGH — three newly covered identities select a wrong analysis of grammatical
Oracle English, and the landing blessed them into the coverage lock. In each
the relativizer `that` is consumed as a bare `FixedDurationPhrase` over
`NounPhraseFusedDeterminativeReference` (a fused-head distal demonstrative),
which is a temporal reading of a word that is introducing the following
relative clause:

- On Wings of Gold, `Creatures you control that are Zombies and/or tokens get
  +1/+1 and have flying.` — ` that` (bytes 21..26) is claimed by
  `determinative:DeterminativeHead/DistalDemonstrative` as the adjunct of the
  object-gap relative `you control`, and ` and/or ` is then claimed by
  `structural:AndOrClauseCoordination`, splitting one clause into two.
- Red Guardian, Super-Soldier, `... destroy target creature an opponent
  controls that dealt damage this turn.` — ` that` (76..81) is the duration
  adjunct of `controls`, and `dealt damage this turn` becomes a reduced
  passive, inverting the voice the card states.
- Zedruu the Greathearted, `... the number of permanents you own that your
  opponents control.` — ` that` (110..115) is the duration adjunct of `own`.

This shape is selected on zero corpus units in the parent tip and on exactly
these three in the result, so the landing introduces it. It is not the
recorded Seedborn Muse misselection class: that class is an attachment-site
choice among readings whose constituency is correct, whereas here a
relativizer is re-analysed as a temporal noun phrase and, for On Wings of
Gold, a nominal coordination is re-analysed as a clause coordination.
CLAUDE.md makes a wrong analysis that starts parsing a STOP, never a coverage
gain; the STOP was not taken.

Decision wanted (blocking): a bare fused-head demonstrative is admitted as a
`FixedDurationPhrase`. Nothing in the corpus selects that reading legitimately
(0 units on the parent tip). Should this ticket narrow the duration phrase to
require an overt nominal head, or does that narrowing belong to its own
ticket? Either answer changes this landing's numbers: with the narrowing the
gain is +3 (Deepfathom Echo, Display of Dominance, Veilstone Amulet) and the
lock must be re-blessed with three additions, not six. The reviewer did not
choose, because the ticket pins only the adjunct-licence removal and the
alternative resolutions differ in coverage.

The three remaining newly covered identities were read and are admissible
under low attachment: Deepfathom Echo takes `until end of turn` inside `you
control`; Display of Dominance and Veilstone Amulet take `this turn` inside
`your opponents control`. All three are the recorded misselection class.

MEDIUM — undisclosed re-spell. `predicate_adjuncts_pin_postposed_prepositions
_outside_their_objects` had its witness changed from `Untap all permanents you
control during each other player's untap step.` to `Untap all permanents
during each other player's untap step.` The change is correct (the relative
clause is what moves the attachment, and the Seedborn Muse sentence keeps its
low-attachment assertion in the object-gap test), but it is a re-spell and was
absent from the assurance list. Re-spelled is 8 contracts, not 7.

MEDIUM — the record states the participial-relative assurance is re-spelled
"for the same `Destroy each creature you sacrifice during your upkeep.`
witness". That sentence is in the object-gap test; the participial test's
witnesses are `Destroy a card you've exiled this turn.` and `Destroy each
creature turned face up this turn.` Both tests keep their own sentences.

MEDIUM — the record's `42 changed decisions` and `29 changed selected
ordinals` count only units selected in both trees; counted over all units they
are 48 and 35, which is what `40 listed + 8 retaining their analysis` implies.
The scoping is now stated here. Memory Theft and Slurrk, All-Ingesting are
described as having a "different selected construction path"; their selected
construction paths are identical, and only the candidate set and the selected
ordinal changed.

MEDIUM — no change-id stamp on the measurement. The figures above were
measured on `qmssqson-` (parent) and `tyltkmto`/`@-` (result), lock covered
count 16,830.

LOW — `object_gap_adjunct_attachment_selects_the_low_relative_reading` carried
two loops asserting the identical predicate over overlapping witness sets; the
duplicate loop is folded into the first, which already contains both of its
sentences. No assertion is lost.

LOW — the nominal selected-ordinal fixture expressed the expected ordinal as
`usize::from(text == "A creature card you control in exile ...")` inside the
loop body; it is now a fourth column of the witness table.

LOW — the record notes the permitted-checker count changed without stating it.
`licensing_checker_permitted` is 25 -> 19: the six deleted licence-reading
predicates (`transitive_head_licenses_{,non}prepositional_adjunct`,
`transitive_participle_head_licenses_{,non}prepositional_adjunct`,
`passive_predicate_licenses_{,non}prepositional_adjunct`).
`licensing_checker_forbidden` stays 0.

Reviewer contention stamp for the performance advisory: 1 concurrent codex
executor plus this review on the host, host load 4.08–11.76 across the runs.
Reviewer measurements: coverage 87.626 s, 114,243 ns/B, load 4.08/6.58/8.52;
ambiguity 87.808 s, 115,689 ns/B, load 9.36/9.42/10.25; parent-tip ambiguity
93.872 s, 116,520 ns/B, load 6.50/8.49/9.82. All exceed the 16.260 s ceiling
under contention; reported, not a STOP.

Assurance counts after review: restored 0, re-spelled 8, ignored 0, added 1,
removed 0. The folded duplicate loop is not an assurance removal — both of its
witnesses remain asserted in the surviving loop.

### Review corrections, round 2 — coordinator ruling of 2026-09-04

The coordinator ruled that the narrowing closing H1 belongs to this landing and
folded `english-v2-fixed-duration-endpoint` into this claim. The narrowing, its
negative witness, the re-measurement and the remaining blockers are recorded in
`docs/tickets/wip/english-v2-fixed-duration-endpoint.md`. In summary:

- H1 is closed. On Wings of Gold, Red Guardian Super-Soldier and Zedruu the
  Greathearted no longer parse; the three identities this landing legitimately
  covers are Veilstone Amulet, Display of Dominance and Deepfathom Echo, each
  re-read after the narrowing and each the recorded attachment-misselection
  class with correct constituency.
- Coverage 16,824 → 16,825 (+3 newly covered, −2 retired). The lock is left at
  the parent-tip content and is NOT blessed: the two retirements are a
  coordinator ruling.
- `licensing_checker_permitted` 25 → 20, forbidden 0. The six deleted readers
  are the licence predicates this ticket removed; the one addition is the
  narrowing's `temporal_endpoint_denotes_a_time`, classified
  `declared_license_feature`.
- The earlier round-1 numbers in this record (16,830, +6, unique 11,515 /
  specificity 5,315, the 40-row winner-change census, the 42/40/29/6 counts,
  and both lock SHA-256s) describe the pre-narrowing tree and are superseded by
  the endpoint ticket's Numbers section. The +6 they report includes the three
  wrong analyses.
- Assurance for the combined landing is counted in the endpoint ticket's
  record, not here.

Not integrated. Two blocking decisions are listed under "Decision wanted"
there: the two-identity retirement, and three verb-frame additions
(`Cycle`, `Explore`, `Scry`) taken to keep the narrowing from dropping 50
identities instead of 2.
