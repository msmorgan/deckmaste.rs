---
needs: []
---
# Read a partitive headed by a quantity phrase: if two or more of those cards have the same name / prevent 1 of that damage / twice that many of those tokens

## Why

The Cardinal-headed partitive takes a bare cardinal word only: NP "two of
those cards" 1, "one of that damage" 1, but "2 of those cards" 0, "1 of that
damage" 0, "one or more of those cards" 0, "two or more of those creatures"
0, "up to two of them" 0, "that many of those tokens" 0, "twice that many of
those tokens" 0. Each of those quantities reads as a Determiner before a noun
("one or more creatures", "up to two cards", "twice that many tokens" 1,
"prevent 1 damage" 2). Blocker report at trunk `ymqwzour`
(`target/english-v3/blockers-wave8.txt`): keys `at:many of` 11, `at:more of`
10, `at:1 of` 10, part of failing token `of` 77.
`cargo xtask english-v3 grep -i` over the union of the four shapes finds
**59** faces, all unread; **50** have that line as their only failing line:

- Arabic numeral + non-count oblique, `\b\d+ of th`: 21 faces, 20 sole
  (*prevent 1 / 2 of that damage* 17 faces, 16 sole; the other 4 are *all
  but 1 of that damage*, which also need `english-v3-all-but-exception`).
- numeral + *or more*, `\b(one|two|…|\d+|X) or (more|fewer|less) of\b`: 15
  faces, 11 sole.
- *that many of*, `\bthat (many|much) of\b`: 14 faces, 11 sole (all under a
  multiplier: *twice / three times / half that many of*).
- *up to* N *of*, `\bup to (one|two|…|X|\d+) of\b`: 9 faces, 8 sole.

## Goal

The quantity position of the Cardinal-headed partitive admits the quantity
phrases that already function as a Quantitative Determiner, and an Arabic
numeral before a non-count oblique: *two or more of those creatures*, *up to
two of them*, *twice that many of those tokens*, *1 of that damage*, with
byte-exact realisation.

## Analysis

CGEL Ch. 5 §9.2, p. 411: "Here the head is followed by a complement
consisting of *of* + a partitive oblique, an NP which can be plural,
non-count, or, under restricted conditions, count singular"; [5ii] "[*Few* of
her friends] *knew she was ill*" (explicitly partitive). Ch. 5 §11(d)–(e),
p. 432: "[*More* than one] *glass was broken*" and "[*At the very most
twenty*] *people will agree to help*" show the modified quantifier as one
determinative phrase. Those back a fused head realised by a determinative
phrase and the plural / non-count oblique. Admitting each Oracle quantity
phrase as that head is a project ruling. Glossary: **Partitive Noun Phrase**,
**Quantitative Determiner**.

Existing routes (`crates/deckmaste_english_v3/src/declarations.rs`):
`PartitiveNounPhrase` (`[quantity: Cardinal, " ", complement:
PrepositionPhrase, modifiers: repeat(PartitiveModifier, "")]`,
`partitive_oblique(complement.ObliqueNumber)`, `export number =
quantity.number`). The quantity phrases are `QuantitativeDeterminer`s fed by
`CardinalDeterminer`, `ComparativeQuantityDeterminer` (over
`NumeralComparativeCoordination`, *one or more*), `PrepositionDeterminer`
(over `QuantitativePrepositionPhrase`, *up to two*) and
`DegreeDeterminativePhrase` (*that many*). Arabic numerals are
`NumericQuantityConjunct` (`numeral_kind = Arabic`), the quantity of
`CardinalMeasuredNominal` (*1 damage*).

Orchestrator rulings 2026-10-10:

- PQ1. Refine `PartitiveNounPhrase`: its `quantity` slot takes the category
  that already unifies these phrases (`QuantitativeDeterminer`), or added
  forms on the same Construction for `ComparativeQuantity` and
  `QuantitativePrepositionPhrase`. No new Construction. The exported number
  comes from the quantity's declared agreement (`DeterminerAgreementNumber` /
  number), so *one or more of those cards* stays as flexible as *one or more
  cards* is today.
- PQ2. The Arabic numeral is admitted through `QuantityConjunct`, and only
  with a Singular (non-count) oblique, read from `ObliqueNumber`: *1 of that
  damage*. *2 of those cards* is unattested (0 faces for `\b\d+ of (those|
  them)`): do not add a check whose only job is to block it, and do not build
  it either.
- PQ3. The multiplier stays where `english-v3-multiplier-predeterminer` put
  it: *twice* + [*that many of those tokens*]. If the predeterminer's host
  test rejects a partitive host, extend its declared host table; do not mint
  a second multiplier route.
- PQ4. One analysis per string. *two of those cards* keeps exactly one
  Reading: the bare cardinal must not read once as `Cardinal` and once as a
  `CardinalDeterminer`-fed quantity. *one or more of your opponents* has one
  structure ([*one or more*] + *of* PP), not *one* coordinated with [*more of
  your opponents*].
- PQ5. Newly covered faces are gains. No Reading-count increase on a covered
  face is expected: 190 covered lines contain *one of (those|them)* and must
  keep their counts. Report any increase with its shape; under PQ4 it is a
  defect.
- PQ6. If one of the four shapes needs more than the slot refinement, land
  the others and return that shape with its evidence; do not STOP the whole
  ticket.

## Witnesses

Each is the only failing line of its face in `blockers-wave8.json` (census
`ymqwzour`, 0 Readings). Fragment probes ran 2026-10-10 on the `ymqwzour`
binary (`--readings 60`, enumeration complete); trunk then moved to
`pxmykwnl` and the prebuilt probe no longer loads plugin data, so (census)
controls give the covered line's `reading_count`. Re-probe on the claim
parent first.

- Urza's Armor: "If a source would deal damage to you, prevent 1 of that
  damage." (probed 0; control "… prevent 1 damage." 2)
- Scalpelexis: "Whenever this creature deals combat damage to a player, that
  player exiles the top four cards of their library. If two or more of those
  cards have the same name, repeat this process."
- Mangara, the Diplomat: "Whenever an opponent attacks with creatures, if two
  or more of those creatures are attacking you and/or planeswalkers you
  control, draw a card."
- The Mana Rig: "{X}{X}{X}, {T}: Look at the top X cards of your library. Put
  up to two of them into your hand and the rest on the bottom of your library
  in a random order."
- Parallel Lives: "If an effect would create one or more tokens under your
  control, it creates twice that many of those tokens instead."

Controls that must not change: NP "two of those cards" 1; NP "one of your
opponents" 1; NP "twice that many tokens" 1; NP "that many tokens" 1;
"Destroy one of them." 1; Bruvac the Grandiloquent "If an opponent would mill
one or more cards, they mill twice that many cards instead." 1 (census);
Commune with Evil "Look at the top four cards of your library. Put one of
them into your hand and the rest into your graveyard. You gain 3 life." 3
(census); Sanguine Indulgence "Return up to two target creature cards from
your graveyard to your hand." 1 (census).

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching

    \b(one|two|three|four|five|six|seven|eight|nine|ten|\d+|X|many|much|more|fewer|less) of\b

case-insensitively (410 faces: the 59 plus the bare-cardinal partitive
control class) plus 300 random covered controls, `--workers 6
--samples-per-face 0`, compared with `cargo xtask english-v3 diff` against
the claim parent. Zero lost; PQ5 for increases. Full `cargo test -p` and
`cargo clippy -p … --tests -- -D warnings` for `deckmaste_english_v3` (and
`deckmaste_lexical_source` if data changes), once, at the end; a rebuild
after a grammar edit takes about 2–3 minutes, the six test binaries about 75
seconds. Tests assert the role (the quantity phrase as fused head, the *of*
PP as partitive Complement, the PQ4 negatives), never an incidental count or
a Debug-printed tree. Attestation by `cargo xtask english-v3 grep` only.

## Out of scope

- Determinative heads (*any of*, *all of*, *either of*):
  `english-v3-partitive-determinative-heads`.
- *all but 1 of that damage*, *all but one of them*:
  `english-v3-all-but-exception`.
- *twice / three times as much of that mana* (3 faces, the *as much* equative).
- *a third of*, *a tenth of* (3 faces), *in excess of* (2), *to a maximum of*
  (1), *one of the same mana symbol* (Jegantha).
- Lines with a second blocker: *put up to one of them on top … and the rest
  on the bottom* (2 faces), Celes, Party Dude, Regicide, Innkeeper's Talent
  (*twice that many of each of those kinds of counters*).

## Landing record

1. **Census delta** (`pusrqwml`; BEFORE uses the claim-parent `ukysqxnm`
   declaration and unchanged lexical sources): 710 faces, **572 → 615**
   covered, **26,331 → 78,740** Readings; 43 gains, zero lost faces,
   decreases or increases, zero issues. The 262 digest-only changes retain
   the existing analyses with the canonical `CardinalDeterminer` wrapper.
   Conqueror's Pledge retains 2 Readings. Selector: the Method's 410 faces
   plus Conqueror's Pledge (*twelve of*), and 300 random covered controls
   selected by `english-v3 diff --manifest --random 300 --seed 20261008`
   from `baseline-wave8.json`; one overlap gives 710 identities. Evidence:
   `target/english-v3/english-v3-partitive-quantity-phrases/{manifest,before,after,diff}.json`.
   After refresh, the same selector retains all counts and coverage: zero
   losses, gains, decreases or increases relative to the verified AFTER,
   with seven digest-only changes from the imported scalar-comparative and
   supertype work (`vszmnwup`, `ooumkosu`). The claim-parent-to-refreshed
   summary remains 43 gains and 262 digest-only changes; reports are
   `after-refresh.json`, `refresh-diff.json` and `final-diff.json` alongside
   the original evidence. A second refresh imported phrasal-keyword work
   (`zvoyutom`) and subtype/plugin definitions. Its bounded recheck adds
   Ellie, Brick Master (29), Ellyn Harbreeze, Busybody (13), and Safana,
   Calimport Cutthroat (3), with zero losses, decreases, increases or
   digest-only changes relative to the first refreshed AFTER. These three
   gains are attributed to trunk, not this partitive refinement. Final
   totals: **572 → 618** covered, **26,331 → 78,785** Readings, 46 gains
   (43 feature + 3 trunk), 262 digest-only changes, zero issues. Evidence:
   `after-final-refresh.json`, `final-refresh-diff.json`, `final-diff.json`.
2. **Witnesses** (`pusrqwml`, complete fragment probes): all five were 0 on
   the claim-parent grammar; after: Urza's Armor 2, Scalpelexis 9, Mangara 1,
   The Mana Rig 3, Parallel Lives 3. Their matched controls have respectively
   2, 9, 1, 3, 3 Readings: independent ambiguity is preserved under W9-A.
   The five quantity fragments each have one route. The ticket's controls
   retain their counts: four NP controls and Destroy one of them 1 each;
   Bruvac 1, Commune with Evil 3, Sanguine Indulgence 1. No already-readable
   witness was dropped.
3. **Spot checks** (`pusrqwml`): Urza's Armor and Guardian Seraph use a
   numeric fused head with a non-count oblique; Scalpelexis and Mangara use
   the numeral/comparative Coordination as fused head; The Mana Rig, Nael,
   and Slimefoot's Survey use the quantitative PP as fused head; Parallel
   Lives, Doubling Season, and Adrix and Nev keep the external multiplier
   above the degree-headed partitive. All retain the *of* PP as Complement.
   No wrong gain was observed. Nael and Slimefoot each contain one partitive
   per affected-line Reading; their other attachment ambiguities remain
   available.
4. **Deviations and STOPs**: no new Construction, Frame, Lexeme or glossary
   term. Refined the existing partitive's forms, agreement and Definiteness;
   retired its bare `Cardinal` slot. Cardinal vocabulary retains its other
   functions; there was no separate lexical partitive route to retire.
   Expanded the selector for the attested higher cardinal, then rebuilt
   BEFORE from the claim-parent declaration and reran the bounded census.
   Under PQ6, return Ojer Taq's counted multiplier *three times*: its control
   *three times that many tokens* and *three times that many of those tokens*
   both probe 0, while *that many of those tokens* probes 1 (`pusrqwml`).
   That modifier needs work beyond this slot refinement. No STOPs.
5. **Tests** (`pusrqwml`): `cargo test -p deckmaste_english_v3` passed all
   975 tests. `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings`
   passed after fixing a needless-by-value parameter in the new witness
   helper; its exact value comparison is preserved. Restored 0 / re-spelled
   0 / ignored 0 / added 8 / removed 0. The module is registered once in
   `suite_6`. Citation checks report zero non-compliant strings and zero
   stale citations; the diff audit selects no new citation sites. Refresh
   rebuilt xtask and reran the full suites with
   `cargo test -p deckmaste_english_v3 -p deckmaste_construction_v3_core -p deckmaste_lexical_source`:
   984 English v3, 39 construction-core and 86 lexical-source tests passed,
   zero failed or ignored. The matching three-package clippy command
   (`--tests -- -D warnings`) passed. After the second refresh,
   `cargo build -p xtask` and the same full three-package test and clippy
   commands passed again: 987 English v3, 39 construction-core and 86
   lexical-source tests, zero failed or ignored.
6. **Timing** (`pusrqwml`): bounded AFTER census corpus wall **96.65 s**,
   **6 workers**, `--samples-per-face 0`, run under `/usr/bin/time -v`.
   The post-refresh bounded census used the same settings and selector:
   **50.10 s** corpus wall, zero issues or undetermined faces. The final
   refreshed bounded census took **82.79 s** corpus wall with those same
   settings, again zero issues or undetermined faces.
