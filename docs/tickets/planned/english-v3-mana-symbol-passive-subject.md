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
