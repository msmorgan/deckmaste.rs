---
needs: []
---
# Small frames batch 3: secretly chooses / comes up heads / contains {X} / no longer snow / controlled continuously

## Why

Data ticket. Five ordinary English words (not game terms) are missing or
under-licensed, and each reads through an existing frame or licence once
declared. Blocker report at trunk `qzprwuxz`
(`target/english-v3/blockers-wave9.txt`); counts by `cargo xtask english-v3
grep` and `blockers-wave9.json` (faces with a failing line matching / of
those, faces with exactly one failing line):

| Item | Pattern | Faces | Sole |
|---|---|---|---|
| A. *secretly* before a finite verb | `secretly` | 14 unread (17 total) | 14 |
| B. *come up heads / tails*, *come under your control* | `c[ao]mes? (up\|under)` | 11 | 11 |
| C. *contain* | `\bcontains?\b` | 12 | 9 |
| D. *no longer* | `no longer` | 9 | 7 |
| E. *continuously* | `continuously` | 6 | 6 |

Vocabulary keys: `word:comes` 5, `word:come` 3, `word:came` 2, `word:contains`
6, `word:contain` 2, `word:longer` 7, `word:continuously` 5 sole. *secretly*
is declared, so it has no `word:` key: it hides under `at:secretly chooses`,
`at:votes for` and failing token `a`.

## Goal

Each item reads in its attested frame with byte-exact realisation:

- A. "Each player secretly chooses a number."
- B. "If the coin comes up heads, …"; "… unless it came under your control
  this turn."
- C. "If its mana cost contains {X}, …"; "Your starting deck contains only
  cards with even mana values."
- D. "All lands are no longer snow."; "it's no longer suspected."
- E. "… the active player has controlled continuously since the beginning of
  the turn."

## Analysis

CGEL Ch. 6 §7, p. 579: "Adjuncts of manner, means, and instrument must
normally be within the VP whose head they modify, in either end position
(which is preferred) or central position." That backs *secretly* in central
position (A) and *continuously* in end position (E). Everything else is a
declaration read by an existing route; where each word is declared is a
project ruling.

Existing declarations (`crates/deckmaste_lexical_source/lexicon/`):
`core.ron`, `vocab:Adverb/Secretly` with `CentralAdjunctLicence:
"Imperative"` against `vocab:Adverb/Simultaneously` with
`"Preverbal|Imperative"` ("Each player simultaneously chooses a number." 1);
`vocab:Adverb/Still` with `"Preverbal|PostCopula"` ("All lands are still
snow." 1, "It's still suspected." 1); `verbs.ron`, `Round`
(`Predicate([Lex("Adverb", "Up")])`), `Get` (`Predicate([Role("CostSymbols")
])`, "You get {E}{E}." 1), `Control` (`Predicate([ObjectNounPhrase])`);
`vocab:Preposition/Up`, `vocab:Preposition/Under`; NP "heads" 1. Grammar
(`crates/deckmaste_english_v3/src/declarations.rs`): schema
`CentralAdverbHead` reads `CentralAdjunctLicence` (`central_preverbal`,
`central_postcopula`).

Orchestrator rulings 2026-10-11:

- SF1 (A). Add `Preverbal` to `Secretly`'s `CentralAdjunctLicence`. That is
  the whole change. Three of the 14 faces (*each player secretly votes for …,
  then those votes are revealed*) also need the noun *vote* of
  `english-v3-tied-for-adjective`.
- SF2 (B). Declare the verb `Come` (*comes*, preterite *came*, participle
  *come*) with two frames: the particle *up* followed by a predicative
  Complement (the coin's face: *heads*, *tails*), on the `Round` particle
  model, and a locative *under* PP Complement (*under your control*). The
  particle follows PH2 of `english-v3-phase-in-out-verb`: a selected
  Intransitive Preposition if *up* can be selected without changing what
  *up* reads elsewhere, else the Adverb on the `Round` precedent; do not
  declare both. No bare intransitive frame: *come* alone is unattested.
- SF3 (C). Declare the verb `Contain` (*contains*, *contained*) with
  `Predicate([ObjectNounPhrase])` and the existing symbol role
  (`Predicate([Role("CostSymbols")])`, as `Get` has) for *contains {X}*. The
  licence is the frame.
- SF4 (D). Declare the multiword Adverb *no longer* (`surface_structure:
  Multiword`) with `CentralAdjunctLicence: "PostCopula"` on the `Still`
  model. It is one lexeme: do not derive it from a determinative *no* and a
  comparative *longer*, and mint no lexeme *longer*. If the post-copula
  position after a clitic (*it's no longer suspected*) or under *become*
  (*have it become no longer suspected*, Deadly Complication) needs a grammar
  change, list those faces and leave them.
- SF5 (E). Declare the Adverb `Continuously` with `VPFinalAdjunct: "Yes"`
  and nothing else (no central or initial use is attested; that is a licence
  list, not a guard).
- SF6. One analysis per string (R-one), by item: *secretly chooses* has the
  adverb as a preverbal Adjunct only; *comes up heads* is verb + selected
  particle + Complement, never *up heads* as a PP; *contains {X}* never reads
  `{X}` as a cost of an activated ability; *are no longer snow* is copula +
  adverb + predicative; *controlled continuously since …* attaches the adverb
  to *controlled*.
- SF7. Newly covered faces are gains. No Reading-count increase is expected
  on covered faces: the three covered *secretly* faces (imperative) keep
  their counts; covered faces containing *up*, *under* or *no* are in the
  selector because their lexemes are neighbours of the new frames. Any
  increase is reported with its shape.
- SF8. Each item lands independently; an item that turns out to need a
  Construction is returned with its evidence, not forced.

## Witnesses

Each is the only failing line of its face in `blockers-wave9.json`. Whole
faces probed 2026-10-11 on the `qzprwuxz` binary (`--category document
--as-card`): all 0. Re-probe on the claim parent first.

- A. Menacing Ogre: "When this creature enters, each player secretly chooses
  a number. …"; Call to the Void: "Each player secretly chooses a creature
  they control and a creature they don't control. …" (sentence 0; without
  *secretly* 1)
- B. Molten Sentry: "As this creature enters, flip a coin. If the coin comes
  up heads, this creature enters as a 5/2 creature with haste. If it comes up
  tails, …"; Erg Raiders: "… it deals 2 damage to you unless it came under
  your control this turn."; Mad Dog: "… if this creature didn't attack or come
  under your control this turn, sacrifice it."
- C. The First Tyrannic War: "I — … If its mana cost contains {X}, it enters
  with …"; Gyruda, Doom of Depths: "Companion — Your starting deck contains
  only cards with even mana values."; Rosheen Meanderer: "{T}: Add
  {C}{C}{C}{C}. Spend this mana only on costs that contain {X}."
- D. Melting: "All lands are no longer snow."; Arcum's Weathervane: "{2},
  {T}: Target snow land is no longer snow."; Absolving Lammasu: "When this
  creature enters, all suspected creatures are no longer suspected."
- E. Norritt: "{T}: Choose target non-Wall creature the active player has
  controlled continuously since the beginning of the turn. …" (sentence 0;
  without *continuously* 3+)

Controls that must not change: "Each player simultaneously chooses a
number." 1; "Each player chooses a number." 1; "If you win the flip, draw a
card." 1; "It deals 2 damage to you unless it attacked this turn." 2; "You
get {E}{E}." 1; "Your starting deck has only cards with even mana values."
3+; "All lands are still snow." 1; "All lands are not snow." 1; "This
creature rounds up." 1.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching

    (?i)secretly|\bc[ao]mes?\b|\bcame\b|\bcontain(s|ed)?\b|no longer|continuously|\b(heads|tails)\b|\bstill\b|simultaneously

plus 300 random covered controls (`cargo xtask english-v3 diff --manifest …
--random 300 --seed 20261008`), `--workers 6 --samples-per-face 0`, compared
with `cargo xtask english-v3 diff` against the claim parent. If SF2 changes
what `vocab:Preposition/Up` or `vocab:Adverb/Up` is (PH2), add every face
matching `\bup\b` (R-reach). Zero lost; SF7 for increases. Full `cargo test
-p deckmaste_lexical_source` and `-p deckmaste_english_v3` with clippy
`--tests -- -D warnings`, once, at the end. `core.ron` and `verbs.ron` load
at runtime, so the data needs no rebuild; a grammar edit (SF8 returns it
instead) would cost 2–3 minutes; the six test binaries take about 75 seconds.
Tests assert the role (each new word in its frame: the preverbal Adjunct, the
selected particle and Complement, the Object or symbol Complement, the
post-copula Adjunct, the VP-final Adjunct; the SF6 negatives), never an
incidental count or a Debug-printed tree. Attestation by `cargo xtask
english-v3 grep` only.

## Out of scope

Ordinary words from the same report that are NOT a lexeme plus an existing
frame; each needs a construction or a second ticket first:

- *ignore* (12 faces, 9 sole): *for that player to ignore this effect* needs
  a *for*–*to* infinitival clause (4); *roll that many dice plus one and
  ignore the lowest roll* needs the delayed degree arithmetic and a noun
  *roll* (4); *ignore one* needs the fused cardinal head (1). Declaring the
  verb alone covers nothing.
- *how* (11 faces, 9 sole): *You choose how those creatures block* is an
  interrogative content clause.
- *whichever* (8 faces, all sole): *…, whichever is greater* is a
  supplementary fused relative; when it reads, revisit the attributive-only
  gate on *greater* recorded in `english-v3-attributive-scalar-comparative`.
- *worth* (5 faces): *up to five {P} worth of modes*.
- *finish* (4): *finish voting*, *finishes resolving* need a
  gerund-participial catenative Complement frame, which `verbs.ron` does not
  have ("Whenever you begin casting a spell, draw a card." 0).
- *just beneath* (3), *as part of casting a spell* (3), *have a creature type
  in common* (3), *directly above* (2), *divided evenly* (2), *10 or higher*
  (2).
- *used to craft*: `english-v3-plugin-term-exports-tribute-story-craft`.
- *tied*, *true*, *replacing*, *differently*: own tickets
  (`english-v3-tied-for-adjective`, `english-v3-the-same-is-true`,
  `english-v3-replace-with-another`, `english-v3-differently-named-premodifier`).

## Landing record

To be written by the claimant: the six items of `docs/tickets/README.md`.
