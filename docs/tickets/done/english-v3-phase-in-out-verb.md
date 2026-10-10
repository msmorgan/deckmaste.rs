---
needs: []
---
# Declare the verb *phase* with its *in* / *out* particle: This creature phases out.

## Why

There is no verb *phase*: only the noun (`CommonNoun` `Phase`) and the keyword
*phasing*. Blocker report at trunk `ymqwzour`
(`target/english-v3/blockers-wave8.json`): key `at:out .` 26 sole-blocker
faces, and more under other keys. `cargo xtask english-v3 grep
'\bphase[sd]? (in|out)\b'` finds **57** faces, all unread; 57 failing lines
match, **45** of them the only failing line of their face. Forms: *phase(s)
out* on all 57, *phase(s) in* on 10 of them, *phased out* / *phased-out* on
one each.

## Goal

*phase* reads as an intransitive verb with an obligatory Intransitive
Preposition Complement *out* or *in*, in its plain, third-person, preterite
and past-participle forms, with byte-exact realisation.

## Analysis

CGEL Ch. 4 §6, p. 272: "We include in the category of prepositions not only
words like *to* … but also *in* … and *up*"; "one major distinction is
between transitive and intransitive constructions". P. 273: "A large number
of verbal idioms contain intransitive prepositions, as in the *give up* of
[1iii]". Those back verb + intransitive preposition as a lexically specified
combination. Where the declaration lives is a project ruling. Glossary:
**Intransitive Preposition**.

Existing declarations (`deckmaste_lexical_source/lexicon`): `verbs.ron`,
`Round` with `Predicate([Lex("Adverb", "Up")])` / `Predicate([Lex("Adverb",
"Down")])` ("This creature rounds up." 1) is the nearest model: a frame whose
only Complement is a named lexeme. `Put` has `Lex("Preposition", "Back")`.
*out* is declared as `vocab:Adverb/Out` only; *in* as `vocab:Preposition/In`
(transitive). Construction `IntransitivePreposition`
(`deckmaste_english_v3/src/declarations.rs`) builds a PP from a bare
Preposition. `plugins_v2/builtin/macros/keyword_abilities/phasing.ron` declares
`grammar: FixedKeyword(surface: "phasing")` with no `verb:`.

Orchestrator rulings 2026-10-10:

- PH1. Data ticket. Declare `Phase` in `verbs.ron` (third person *phases*,
  preterite and participle *phased*) with two frames, one per particle, on the
  `Round` model. It is a core verb: the keyword plugin's `verb:` slot derives a
  verb from the keyword surface (*saddle*, *exploit*), and *phasing* is not
  *phase*; moving it into `phasing.ron` belongs to
  `english-v3-plugin-term-definitions` (held for the owner), not here.
- PH2. Particle category: *out* and *in* are Intransitive Prepositions (CGEL
  p. 272). Use `Lex("Preposition", …)` if an intransitive *out* / *in* can be
  declared without changing what transitive *in* reads; otherwise use the
  existing `vocab:Adverb/Out` on the `Round` precedent and record the category
  mismatch as a glossary/lexicon residue in the landing. Do not declare both.
- PH3. The licence is the verb's frame. No construction or check names
  *phase*. If the frame consumer needs a grammar change to read a
  particle-only frame for a verb other than `Round`, that is in scope; report
  it as a deviation.
- PH4. One analysis per string. *phases out* is verb + selected particle; it
  must not also read as the noun *phases* + *out*, nor with *out* as a free
  Adjunct of an intransitive *phase* (declare no bare `[]` frame).
- PH5. *phased-out* as a hyphenated premodifier (*all phased-out creatures*,
  Time and Tide; 1 face) is in scope only if the participle + particle
  compound follows from an existing hyphenated-premodifier route; otherwise
  list it and leave it.
- PH6. Newly covered faces are gains. No Reading-count increase on a covered
  face is expected: no covered face matches the pattern. The noun *phase*
  (*main phase*, *combat phase*) must keep exactly its Readings; an increase
  there means the verb is being read where the noun stands and is a defect.

## Witnesses

Each is the face's only failing line at `ymqwzour`. Probed 2026-10-10 with the
`ymqwzour` binary (`--category sentence --readings 3`): all 0. Re-probe on the
claim parent first.

- Rainbow Efreet: "{U}{U}: This creature phases out." (sentence "This creature
  phases out." 0)
- Reality Ripple: "Target artifact, creature, or land phases out." ("Target
  creature phases out." 0)
- Taniwha: "At the beginning of your upkeep, all lands you control phase out."
  ("All lands you control phase out." 0)
- March of Swirling Mist: "Up to X target creatures phase out."
- Spatial Binding: "Pay 1 life: Until your next upkeep, target permanent can't
  phase out." (sentence 0)

Also 0 today: "It can't phase in."; "You may have any number of them phase
out." Controls that must not change: "This creature rounds up." 1; "This
creature rounds down." 1; "Put that card back." 1; "This creature can't
block." 1; and the Reading counts of every covered face containing `\bphases?\b`
as a noun.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `\bphase[sd]?\b` (267 faces, verb and noun: the new verb shares
its forms with the noun) plus 300 random covered controls (`cargo xtask
english-v3 diff --manifest … --random 300 --seed 20261010`), `--workers 6
--samples-per-face 0`, compared with `cargo xtask english-v3 diff` against the
claim parent. Zero lost; PH6 for increases. Full `cargo test -p
deckmaste_lexical_source` and `cargo test -p deckmaste_english_v3` with clippy
`--tests -- -D warnings` for both, once, at the end. Lexicon data loads at
runtime, so a data-only change needs no rebuild; a grammar edit costs 2–3
minutes and the six test binaries about 75 seconds. Tests assert the role
(*phase* as the verb, the particle as its selected Complement, the PH4
negatives), never an incidental count or a Debug-printed tree. Attestation by
`cargo xtask english-v3 grep` only.

## Out of scope

- Lines where the remainder has its own blocker (12 non-sole faces).
- *phased out* as a state Adjective after *is* / *remains* (unattested in the
  sole lines).
- Semantics of phasing; the `phaseOutUntil` instruction macro.

## Landing record

1. **Census delta** — `roomopmrpkys`: claim-parent grammar (`purnrskv`)
   BEFORE → implemented AFTER, 467 → 511 covered faces, 12,661 → 13,070
   Readings; no/one/multiple faces 100/121/346 → 56/146/365. 44 gains;
   lost faces none; decreases, increases, digest-only changes and issues all 0.
   Selector: every supported face matching `\b(?:phase[sd]?|out)\b` (267),
   including every retired *out* surface, plus 300 disjoint random covered
   controls, seed **20261008** (binding session direction): **567 faces**.
   Evidence: `target/english-v3/english-v3-phase-in-out-verb/{manifest,before,after,diff}.json`;
   comparison used the stock `english-v3 diff --lines`.
2. **Witnesses** — `roomopmrpkys`: all claim-parent witness probes were 0;
   AFTER face totals: Rainbow Efreet 2, Reality Ripple 1, Taniwha 3,
   March of Swirling Mist 26, Spatial Binding 1. Each listed sentence/ability
   probe is 1 after, including “It can't phase in.” and “You may have any
   number of them phase out.” (Change of Plans). Simplified probes “Target
   creature phases out.” and “All lands you control phase out.” also 0 → 1.
   Round up/down, Put back and can't block controls remain 1 each.
3. **Spot checks** — `roomopmrpkys`: Blink Dog, Clever Concealment, Creeping
   Inn, Crystal Golem, Divine Smite, Dream Fighter, Ertai's Familiar, Frenetic
   Efreet, Galadriel's Dismissal and Guardian of Faith. Every Reading of their
   complete Documents retains *phase* as `SelectedPredicate`, with *out* as
   its selected Preposition Complement (`FrameValue::Marker`); verified
   structurally in `phase_in_out::ten_newly_covered_faces_have_the_selected_particle_analysis`.
   Unintended-only gains observed: 0.
4. **Deviations and STOPs** — `roomopmrpkys`: PH2 replaces the unused Adverb
   *out* declaration with selected-only Intransitive Preposition *out*;
   there is no separate word-specific grammar route to retire. *In* is
   selected directly by the new verb frame; its existing transitive PP
   declarations are unchanged. Added the two PH1 frames; no grammar,
   loader, plugin or glossary edits. No category residue or STOP. PH5 leaves
   Time and Tide's *phased-out* premodifier unread: no existing compound
   route. Corpus grep found 0 *phasing in/out* faces, so the default gerund
   reaches no extra faces outside this selector.
5. **Tests** — `roomopmrpkys`: `cargo test -p deckmaste_english_v3` passed
   (901 tests); `cargo test -p deckmaste_lexical_source` passed (84 tests).
   `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings` and
   `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings` passed.
   The first English run failed only the new fixture requesting bare
   *combat phase* as Noun Phrase; corrected it to Nominal, retaining the
   complete Reading-set comparison, then reran the full suite. Assurance:
   restored 0 / re-spelled 0 / ignored 0 / added 5 / removed 0. The five
   tests cover witness roles, ten complete gained faces, independent
   inflection/particle roundtrips, PH4 negatives and unchanged noun/control
   Reading sets; registered once in `tests/main.rs`. Citation checks:
   0 noncompliant, 0 stale; no new rule registrations.
6. **Timing** — `roomopmrpkys`: bounded AFTER census **38.03 s** wall under
   `/usr/bin/time -v`, **6 workers**, `--samples-per-face 0`. No full-corpus
   census or Lean check was run.
