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
