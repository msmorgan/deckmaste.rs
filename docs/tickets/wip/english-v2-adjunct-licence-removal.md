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
