---
needs: []
---
# Read a preposed frequency Adjunct: Once during each of your turns, you may cast … / Once each turn, you may pay {0} …

## Why

*once each turn* and *once during each of your turns* read after the verb
("You may cast an instant or sorcery spell from the top of your library once
each turn." 3+) and their interval reads preposed on its own ("Each turn,
you may …" 3+; "During each of your turns, you may …" 3+), but the two
together before the clause read 0. Blocker report at trunk `uoqyuxtn`
(`target/english-v3/blockers-wave7.txt`): keys `at:once each` 25 and
`at:once during` 18 (both mixed with the VP-final use).
`cargo xtask english-v3 grep '\bOnce (each|during) [^,.]*,'` finds **32**
faces, all unread; **29** have that line as their only failing line: *Once
during each of your turns,* 22, *Once each turn,* 10. No other preposed
frequency is attested (`'\b(Twice|Only once) (each|during)'` 0 faces;
`'\bOnce (on|per|a|every|this)\b'` 0).

## Goal

*Once* followed by its interval Adjunct reads before a comma and the main
clause (*Once each turn, you may …*, *Once during each of your turns, you
may …*), with byte-exact realisation. This is expected to be a data ticket.

## Analysis

CGEL Ch. 8 §9, p. 715: "The class of bounding frequency expressions contains
the adverbs *once* and *twice*"; [9iii] "*once a day*, *once every
half-hour*, *twice a year*, *three times each month*"; and of *on three
occasions each year*, "*each year* is probably a separate adjunct rather than
a postmodifier (as suggested by its position in *Each year we had lunch
together on about three occasions*)". Those back *once* as a frequency
Adverb, its combination with an interval expression, and the two-Adjunct
analysis of such a combination. Front position of *once* + interval is a
project ruling.

Today's VP-final analysis is two stacked Adjuncts: "Draw a card once each
turn." 1 is `NominalAdjunctPredicate` [*each turn*] over `AdverbPredicate`
[*once*] (`docs/tickets/done/english-v3-once-frequency.md`:
`vocab:FrequencyAdverb/Once` has `VPFinalAdjunct: Yes, ClauseInitialAdjunct:
No`). `FrequencyPhrase` is only `CountedFrequency` (*three times*). The
preposed routes (`crates/deckmaste_english_v3/src/declarations.rs`):
`InitialAdverb` (`[modifier: AdverbPhrase, " ", clause: Clause]` and the
comma form, `require modifier.ClauseInitialAdjunct = Yes`) and
`InitialPreposition` (PP + ", " + Clause; and `[dependent:
NominalAdjunctPhrase, ", ", clause: Clause]` with `NominalAdjunctClass =
Temporal`, from `docs/tickets/done/english-v3-next-time-relative-adjunct.md`).

Orchestrator rulings 2026-10-10:

- PF1. Mirror the VP-final analysis: two preposed Adjuncts, *Once* through
  `InitialAdverb`'s space form over the `InitialPreposition` clause that
  already reads *each turn, …* / *during each of your turns, …*. The only
  change is the declared `ClauseInitialAdjunct` of the frequency Adverb in
  `lexicon/core.ron`. No new Construction and no *once* + interval unit.
- PF2. If the declaration alone does not read the witnesses, the fallback is
  one added form on `InitialAdverb` or `InitialPreposition` licensed by a
  declared feature of the Adverb (its frequency class), never by the word;
  record it under Deviations. Anything larger is a STOP.
- PF3. One analysis per string. *Once each turn, you may …* reads with one
  structure; it must not also read *Once* as a subordinator with a clause
  (that use stays out, as the once-frequency landing left it), nor *each
  turn* as Subject of a following clause.
- PF4. *twice* shares the lexical class; whether it gets the same value
  follows the pruning rule (0 preposed faces): declare it only if the class
  is shared by construction of the data, and say which.
- PF5. Bare *Once, …* and *Once you …* are unattested in this function
  (`'\b[Oo]nce (you|it|that|a|an|the|this|they)\b'`: 1 face, The Chain Veil,
  *once this turn*, VP-final). Do not add a check whose only job is to block
  them; report any covered face they change.
- PF6. Newly covered faces are gains. No Reading-count increase on a
  previously covered face is expected: 363 faces contain *once*, 227 of them
  covered with *once each / during* after the verb, and those must keep
  exactly their Readings (*Activate only once each turn.* 1). Report any
  increase with its shape before landing; under PF3 it is a defect.

## Witnesses

Each is the face's only failing line at `uoqyuxtn`. Probed 2026-10-10 with
the `uoqyuxtn` binary (`--category document --readings 3`): witness 0;
control = the same text without *Once* (capitalising the next word), 3+ each
(enumeration capped). Re-probe on the claim parent first and record the
uncapped control counts.

- Johann, Apprentice Sorcerer: "Once each turn, you may cast an instant or
  sorcery spell from the top of your library."
- Darksteel Monolith: "Once each turn, you may pay {0} rather than pay the
  mana cost for a colorless spell you cast from your hand."
- As Foretold: "Once each turn, you may pay {0} rather than pay the mana cost
  for a spell you cast with mana value X or less, where X is the number of
  time counters on this enchantment."
- Lurrus of the Dream-Den: "Once during each of your turns, you may cast a
  permanent spell with mana value 2 or less from your graveyard."
- Kess, Dissident Mage: "Once during each of your turns, you may cast an
  instant or sorcery spell from your graveyard. If a spell cast this way
  would be put into your graveyard, exile it instead."

Controls that must not change: "Draw a card once each turn." 1; "Activate
only once each turn." 1; "This ability triggers only once each turn." 1;
"You may cast a creature spell from your graveyard once during each of your
turns." 3+; "Each turn, you may pay {2}." 1; "Then draw a card." 1;
"Simultaneously, each player draws a card." 1.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching

    \b([Oo]nce|[Tt]wice|[Ee]ach turn,|[Ee]ach of your turns,)

(699 faces: the 363 *once* faces, *twice*, and the preposed-interval
control class) plus 300 random covered controls (`cargo xtask english-v3 diff
--manifest … --random 300 --seed 20261008`), `--workers 6 --samples-per-face
0`, compared with `cargo xtask english-v3 diff` against the claim parent.
Zero lost; PF6 for increases. Full `cargo test -p` and `cargo clippy -p …
--tests -- -D warnings` for `deckmaste_lexical_source` and
`deckmaste_english_v3`, once, at the end: lexicon data loads at runtime, so
PF1 needs no grammar rebuild; a rebuild (PF2 only) takes about 3 minutes and
the test binary about 7. Tests assert the role (the frequency Adverb and the
interval Adjunct as two preposed Adjuncts of the same clause, in that order;
the PF3 negatives; the unchanged VP-final structure), never an incidental
count or a Debug-printed tree. Attestation by `cargo xtask english-v3 grep`
only.

## Out of scope

- The Eighth Doctor (*Once during each of your turns, …*; also `word:else`)
  and the two other faces with a second failing line.
- Subordinator *once* with a clause; temporal-location *once*.
- Preposed *Twice …* / *Only once …* (0 faces) and any other preposed
  frequency expression.
- *only once each turn and only if …* coordination.

## Landing record

1. **Census delta** (`lvpxmkkl`, claim parent `svyrxusu`): bounded selector
   `\b([Oo]nce|[Tt]wice|[Ee]ach turn,|[Ee]ach of your turns,)` reaches 699
   supported faces; add 300 covered controls, seed 20261008 (994 distinct
   faces). Covered 778 → 806; total Readings 37,052 → 45,089; gained 28;
   lost none; decreases, increases and digest-only changes all 0; issues 0.
   The parent subset matches `baseline-wave7.json` on all 994 faces.
   Evidence: `target/english-v3/english-v3-preposed-frequency/{manifest,before,after,diff}.json`;
   comparison uses `cargo xtask english-v3 diff --lines`. No previously
   covered face changes, including bare-*once* contexts under PF5.
2. **Witnesses** (`lvpxmkkl`): all five fragments failed with 0 before the
   edit. After: Johann, Apprentice Sorcerer 18 (whole face 26); Darksteel
   Monolith 90 (91); As Foretold 138 (139); Lurrus of the Dream-Den 10 (15);
   Kess, Dissident Mage 9 (10). Uncapped without-*Once* controls before are
   respectively 18, 90, 138, 10, 9. The seven unchanged controls in the
   Witnesses section are respectively 1, 1, 1, 10, 1, 1, 1. Frequency and
   interval are two ordered preposed Adjuncts; main-Clause ambiguity is
   preserved, with no alternative subordinator or interval-Subject analysis.
3. **Spot checks** (`lvpxmkkl`): Access Maze, Danitha, New Benalia's Light,
   Demon of Fate's Design, Gisa and Geralf, Karador, Ghost Chieftain, One
   with the Multiverse and Vision, Spectral Synthezoid select `InitialAdverb`
   (*once*) over `InitialPreposition` (*during* PP). Assemble the Players,
   Tlincalli Hunter and Warped Space select the same frequency over the
   temporal `NominalAdjunctPhrase` (*each turn*). All ten are newly covered,
   byte-exact and issue-free; none reads only through an unintended analysis.
4. **Deviations and STOPs**: PF1 suffices: only Once's declared
   `ClauseInitialAdjunct` changes. No Construction, frame or route added or
   retired; no PF2 fallback, STOP or glossary gap. PF4 leaves Twice unchanged:
   the entries are independently declared, not generated from a shared
   licence recipe; fresh supported-corpus grep finds 0 preposed *Twice* /
   *Only once* faces. The claim-parent result is 28 gains rather than the
   ticket's earlier estimate of 29; Exploration Broodship, Hauken's Insight,
   Kotis, Sibsig Champion and The Eighth Doctor remain blocked independently
   of *Once*. Three new tests implement the requested structural, independent
   composition/roundtrip and unchanged VP-final contracts, registered once
   in `suite_5`.
5. **Tests** (`lvpxmkkl`): `cargo test -p deckmaste_english_v3` passed 793;
   `cargo test -p deckmaste_lexical_source` passed 82. Both
   `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings` and
   `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings` passed.
   Restored 0 / re-spelled 0 / ignored 0 / added 3 / removed 0.
   Citation checks: 0 noncompliant, 0 stale; bless changes no registrations;
   diff audit selects no new citation sites. No Lean checks or full-corpus run.
6. **Timing** (`lvpxmkkl`): bounded after census, 19.36 s wall under
   `/usr/bin/time -v`, 6 workers, 0 samples per face.
