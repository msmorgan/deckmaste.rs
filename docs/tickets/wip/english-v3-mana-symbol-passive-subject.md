---
needs: []
---
# Read a Mana Phrase as the Subject of a mana verb's passive: sacrifice it unless {U} was spent to cast it.

## Why

*mana* as a noun reads as the Subject of passive *spend* ("If no mana was
spent to cast it, draw a card." 2; "Mana from a Treasure was spent to cast
it." 2). Symbol notation in the same position reads nowhere ("{U} was spent
to cast it." 0; "{U} was paid." 0), and *spend* does not take a symbol Object
either ("You spend {U}." 0, while "You pay {2}." 1). Blocker report at trunk
`qzprwuxz` (`target/english-v3/blockers-wave9.json`): hidden under `at:it .`
(about 12 of 36), `at:spell .` (10 of 11), `at:card .`, `at:hand .`,
`at:turn .`, `at:step .`. `cargo xtask english-v3 grep -i '\} (was|wasn.t|were)
spent to cast'` finds **43** faces, all unread; 48 failing lines match,
**38** the only failing line of their face. Sole lines by shape: preposed *if
{U} was spent to cast this spell/it,* 23; trailing *… if {U} was spent to cast
this spell* 9; *unless {U} was spent to cast it* 6.

## Goal

Symbol notation the grammar already calls a Mana Phrase reads as the Subject
of the passive of a verb whose declared frame takes a Mana Phrase, with the
passive's existing dependents (the purpose *to*-infinitival, negation),
byte-exact.

## Analysis

No CGEL passage is relied on for symbols; the passive itself is the existing
analysis. Symbol licensing is governed by the settled rulings MS2–MS4 of
`english-v3-mana-symbol-nominal` (restated in
`docs/tickets/planned/english-v3-mana-symbol-verbless-subject.md`): a symbol
class is licensed through the governing frame's declared feature, bare
symbol notation stays outside Noun Phrase, and existing Mana Phrase and Cost
Readings are preserved.

Existing routes (`crates/deckmaste_english_v3/src/declarations.rs`): Category
`ManaPhrase` (`SymbolSequence` over `ManaSymbol`, `NamedManaSymbol`); frame
`ManaComplement = Predicate(Complement(ManaPhrase))` with `FrameUse = Mana`
and schema `SharedManaComplement`, selected by the lexicon frame
`Predicate([Role("ManaPhrase")])` that *add* and *produce* declare
(`lexicon/verbs.ron`; *pay* selects `Role("CostSymbols")`, a different
frame); the passive family `PassiveSelectedHead`
(`FrameUse = PassiveNominal | PassiveInternalised`), `SelectedFramePassive`,
`PassivePredicate`; finite clauses take `subject: NominativePhrase`. Lexicon
(`crates/deckmaste_lexical_source/lexicon/core.ron`): `lexeme:Verb/Spend`
declares one frame, `Predicate[Object: NounPhrase]`; the purpose Adjunct
after *spent* comes from `english-v3-purpose-infinitival` (landed).

Orchestrator rulings 2026-10-11:

- MP1. Two steps, in order. (a) Data: *spend* declares the Mana Phrase
  Complement frame *add* and *produce* declare (`Role("ManaPhrase")`), so
  "You spend {U}." reads. (b) Grammar: the passive of a `ManaComplement` frame takes a
  Mana Phrase Subject. (a) alone covers no witness and lands only with (b).
- MP2. The licence for (b) reads the passive head's declared frame (its
  active Complement Category is `ManaPhrase`), never the verb's identity and
  never a symbol's spelling. Refine the existing passive route
  (`PassiveSelectedHead` / the finite passive clause) with a Subject form for
  that frame; no new clause family. If a Subject projection Construction is
  unavoidable it is ONE, shared with
  `english-v3-mana-symbol-verbless-subject` if that ticket has landed or is in
  flight (coordinate, do not build two); say so up front (R-budget): it buys
  38 sole faces.
- MP3 (R-class). Every verb that declares the Mana Phrase Complement frame
  gets the passive Subject (*spend*, *add*, *produce*), not a row for
  *spent*. Unattested passives (*{U} was added.*) may read; add no check to
  block them. Mint no Lexeme. The `CostSymbols` passive (*{U} was paid.*) is not
  built here.
- MP4. MS2–MS4 stand: no bare symbol becomes a Noun Phrase. "{X}" and "{2}"
  as bare Mana Phrase Readings stay as the test
  `mana::independent_mana_values_preserve_symbol_sequence` pins them; which
  symbols are Mana Phrase is not widened here. `{S}` (*if {S} of any of that
  spell's colors was spent*, Boreal Outrider) and `{C}` follow whatever
  `ManaSymbolUse` already declares.
- MP5 (R-one). One route for the symbol Subject. The noun control reads 2
  ("If no mana was spent to cast it, draw a card."): record the two trees
  first; the witness must read exactly as many ways as that control. The twin
  to avoid: the symbol as a Cost label followed by a subjectless clause.
- MP6. Agreement: a Mana Phrase Subject takes singular *was*/*wasn't* in all
  43 faces, including multi-symbol *{U}{B} was spent*. Read number from a
  declared export of the Mana Phrase, not from the symbol count.
- MP7. Newly covered faces are gains. No Reading-count change is expected on
  covered faces; R-reach applies if a passive Construction's form or exports
  change: then the selector's `spent|paid|added|produced` half is mandatory
  and digest-only changes are reported by class.

- MP8 (R-reach, orchestrator, 2026-10-11). R-reach targets changes to
  what existing inputs build: changed forms, requirements or exports, or a
  renamed/re-shaped Reading. A purely additive form whose new first field
  is a Mana Phrase Subject does not require every finite-clause face.
  Existing `FiniteClause` forms retain their fields, requirements and exports
  byte-for-byte, and existing generated Readings are unchanged. If new
  fields cannot be omitted from old Debug trees, or existing digests move,
  STOP. Use the ticket selector (including `spent|paid|added|produced`) plus
  2,000 random covered faces, seed 20261008; require zero digest-only changes
  among those controls. The orchestrator's next full census is the final
  identity check; the landing records the exact claim that existing finite
  clauses are unchanged. MP1–MP7 stand.

## Witnesses

Each is a sentence of the face's only failing line. Probed 2026-10-11 with the
trunk binary (`--category sentence --readings 8`): all 0.

- Azorius Herald: "When this creature enters, sacrifice it unless {U} was
  spent to cast it."
- Ribbons of Night: "If {U} was spent to cast this spell, draw a card."
- Shrieking Grotesque: "When this creature enters, if {B} was spent to cast
  it, target player discards a card."
- Wumpus Aberration: "When you cast this spell, if {C} wasn't spent to cast
  it, target opponent may put a creature card from their hand onto the
  battlefield."
- Mythos of Brokkos: "If {U}{B} was spent to cast this spell, search your
  library for a card, put that card into your graveyard, then shuffle."

Also 0: "{U} was spent." ; "You spend {U}." ; "Draw a card if {G} was spent
to cast this spell." Controls that must not change: "If no mana was spent to
cast it, draw a card." 2; "Mana from a Treasure was spent to cast it." 2;
"Pay {2} to cast it." 1; "You spend mana." 1; "When this creature enters,
sacrifice it unless you pay {U}." 1; "Search your library for a card, put
that card into your graveyard, then shuffle." 6.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `(?i)\b(spent|spend|spends|paid|added|produced)\b|\}
(was|wasn.t|were|is|are)\b` plus 300 random covered controls (`--random 300
--seed 20261008`), `--workers 6 --samples-per-face 0`, compared with `cargo
xtask english-v3 diff` against the claim parent. Zero lost; MP7 for changes.
Full `cargo test -p deckmaste_english_v3`, `-p deckmaste_lexical_source` and
clippy `--tests -- -D warnings` for both, once, at the end (a grammar rebuild
takes 2–3 minutes, the six test binaries about 75 seconds; lexicon data loads
at runtime). Tests assert the role (the Mana Phrase is the passive's Subject;
the purpose Adjunct keeps its attachment; MP4's negatives), never an
incidental count or a Debug-printed tree. Attestation by `cargo xtask
english-v3 grep` only.

## Out of scope

- *for each mana from a Treasure spent to cast it* (Marut, Bat Colony,
  Coin of Mastery, Verazol; 8 sole lines with *each mana*): *mana* as a count
  noun under *each* ("Create a token for each mana spent." 0).
- *if three or more mana from creatures was spent* (Inga and Esika).
- Coordinated trailing conditions (*… if {G} was spent … and X life if {W}
  was spent …*, Dawnglow Infusion, Firespout, Moonhold): gapped coordination.
- *pay {2} more to cast it* (9 sole; "You pay {2} more." 0).
- `english-v3-mana-symbol-verbless-subject` (*with {X} in its mana cost*).

## Landing record

1. **Census delta.** Claim-parent parser `worzonox`, measured in `onymknyk`:
   2,394 covered faces / 77,490 Readings before; `onymknyk`: 2,435 / 80,269
   after. Lost: none. Official `diff --lines`: 41 gained, zero decreases,
   increases on covered faces, digest-only changes, source changes or
   identity mismatches; 2,497 unchanged. Of the 41 newly covered faces,
   40 have the intended analysis; Dawnglow Infusion's 18 Readings only
   coordinate *this spell and X life* as the Object of *cast*, so it is an
   unintended parse, not an intended gain (hygiene A10). The grep selector
   `(?i)\b(spent|spend|spends|paid|added|produced)\b|\} (was|wasn.t|were|is|are)\b`
   reaches 571 faces (671 matches); its union with 2,000 random covered
   faces, seed 20261008, has 2,538 identities (33 overlap). Evidence:
   `target/english-v3/english-v3-mana-symbol-passive-subject/{manifest,before,after,diff}.json`.
   MP8's exact claim: existing finite-clause Readings are unchanged.
   The original `FiniteClause` forms, requirements and exports are retained
   byte-for-byte; the additive form has a distinct Mana Subject first field
   and adds no fields to existing Reading variants. All 2,000 sampled covered
   faces are unchanged, with zero digest-only changes (hence zero per-class
   changed-line counts). The orchestrator's next full-corpus census is the
   final identity check.
2. **Witnesses.** All eight failed with zero sentence Readings on the
   claim parent `worzonox`, before implementation. On `onymknyk`, Azorius
   Herald 3, Ribbons of Night 2, Shrieking Grotesque 2, Wumpus Aberration 8,
   Mythos of Brokkos 18; the corresponding noun controls have respectively
   3, 2, 2, 8, 18, preserving independent ambiguity under W9-A.
   *{U} was spent.* 1; *You spend {U}.* 1; *Draw a card if {G} was spent to
   cast this spell.* 3 (noun control 3). The six original controls retain
   their claim-parent counts: noun-if 2, noun-source 2, pay-purpose 1,
   spend-noun 1, unless-pay 1, and bare Mythos search 6. Probes under
   `before-probes/`, `after-probes/` record complete enumerations.
3. **Spot checks.** On `onymknyk`, Ribbons of Night, Ogre Savant, Gruul
   Scrapper, Steamcore Weird, Tin Street Hooligan, Rolling Spoil, Seed Spark,
   Mythos of Nethroi, Vigor Mortis and Induce Paranoia each select a Mana
   Phrase as Mana Subject of the passive, a declared Mana Complement frame
   on *spent*, and the purpose Infinitival *to cast*; their preposed or
   trailing condition keeps its existing attachment. These were inspected
   with structured probes under `spot-probes/`.
4. **Deviations and STOPs.** The R-reach STOP was resolved by orchestrator
   ruling MP8 above. MP2's one unavoidable `ManaSubject` projection was
   declared up front (the ticket estimates 38 sole faces); the verbless
   sibling is not in flight. Four passive Categories and one projection
   policy specialize existing selected-head, participial-auxiliary,
   overt-complement and purpose schemas, without a new clause family,
   verb-identity guard, retired route or removed frame. Spend gains the
   declared Mana Complement frame; add/produce automatically receive the
   same passive licence. No symbols gain Noun Phrase distribution. Moonhold
   gains through existing coordination of two complete finite predicates,
   each with its own condition; no gapped coordination was added. Dawnglow
   remains the separately disclosed defect above, also appended under
   systemic residuals / Faces covered by a known wrong analysis; Firespout
   remains unread.
   Glossary additions explain the governed Mana Subject via project rulings
   MP2/MP6. Hygiene A10/B7 record the residual Dawnglow analysis and redundant
   selected-head interface exports. Two inherited citation-check false
   positives in the planned predicative-gap ticket were reworded from
   decimal seconds to equivalent minutes/seconds, preserving their values.
   No Construction, frame or test was deleted.
5. **Tests.** `cargo test -p deckmaste_english_v3`: pass (all six integration binaries;
   one pre-existing ignored test, no failures);
   `cargo test -p deckmaste_lexical_source`: pass (88 tests).
   `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings` and
   `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings`: pass.
   Results measured on `onymknyk`; logs under the evidence directory.
   Restored 0 / re-spelled 1 / newly ignored 0 / added 6 / removed 0.
   The re-spelled
   `purpose_infinitival::auxiliary_predicates_inherit_the_selected_predicates_purpose_licence`
   changes the expected value for *{R} was spent to cast this spell.* from
   the empty Reading set to a nonempty set with typed Mana Subject, selected
   passive and purpose constituents; its other
   auxiliary assertions remain. The six new tests assert structural roles,
   frame-class derivation, agreement/symbol boundaries and both roundtrip
   laws. Citation checks: zero noncompliant, zero stale; stdin diff audit
   zero changed citation sites. No Lean, plugin or xtask source changed.
6. **Timing.** The one bounded AFTER census on `onymknyk` took **82.45 s**
   wall time under `/usr/bin/time -v`, with 6 workers and
   `--samples-per-face 0`. No full-corpus census was run in this workspace.
