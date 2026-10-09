---
needs: []
---
# Read a bare zone noun after a preposition: You may play that card from exile this turn.

## Why

*exile* and *hand* never read without a Determiner after a preposition.
On census `qsupksky` (default `@` `mqrlzszk` is empty; 32,828 supported faces, 25,474 covered, 7,354 unread; wave-5 recon of 2026-10-08), the recon bucket "bare zone noun after preposition (*in hand* / *in
exile* / *from exile* / *into exile*)" touches **256** unread faces and is the
sole cause on **190**. Sole failing units by phrase: *in hand* 95, *from
exile* 56, *in exile* 30, *into exile* 23. Counts are unread faces *touched* (at least one localised failing unit matches) / *sole* (every failing unit matches and no other recon STRONG bucket does). They are surface counts, not gain forecasts.

Probes (admitted roots, `--readings 1`): "You may play that card from exile
this turn." 0, with *from your graveyard* 1; "Activate only if you have no
cards in hand." 0, with *in your hand* 1.

## Goal

A bare singular *exile* or *hand* reads as the Complement of *in*, *from*
and *into*, with byte-exact realisation, on the attested nouns and
prepositions only. Determined forms (*your hand*, *a graveyard*) keep their
Readings unchanged.

## Analysis

CGEL, Ch. 5 §8.5(b), p. 409, [20i] (*Ed is in hospital* / *went to school*
/ *went off stage*): singular count nouns that normally need a Determiner
occur bare in fixed location frames. That passage backs the bare-NP analysis;
the choice of nouns and prepositions is the corpus's.

Existing route: `BarePrepositionNounPhrase`
(`crates/deckmaste_english_v3/src/declarations.rs:2819`, reads
`BarePrepositionUse`) and `BareNominalPreposition` (`declarations.rs:2913`,
requires `BareNominalComplement = Yes`). Only *on top of* uses them today
(`lexeme:CommonNoun/Top`, `lexicon/core.ron:1047`; `vocab:Preposition/On`,
`core.ron:224`). Declare the same two features on *exile*, *hand* and on
`vocab:Preposition/In`, `From`, `Into` (`core.ron:220–222`). The legacy
`BareLocativeLicense` / `BareLocativeComplement` entries in
`lexicon/vocabulary.rs:50,55,316–326` have no v3 reader; do not consume
them.

Orchestrator ruling 2026-10-08 (R1b): *from exile* keeps both its verb
attachment and its noun attachment (*the first spell you cast from exile*).
The deferred from-M exclusion recorded in `english-v3-systemic-residuals`
("from-M deferred") stays deferred; this ticket does not re-apply it. The
two attachments are retained ambiguity, not a defect.

*in hand* after a Determiner-less Object (*no cards in hand*) is the same
bare-NP Complement as *in your hand* minus the Determiner, not an elided
possessor.

## Witnesses

Each is the face's only failing unit at `qsupksky` (0 Readings), each
control 1:

- Sea Gate Wreckage: "Activate only if you have no cards in hand." (control
  "… in your hand.")
- Norin, Swift Survivalist: "You may play that card from exile this turn."
  (control "… from your graveyard …")
- Wild-Magic Sorcerer: "The first spell you cast from exile each turn has
  cascade." (control "… from your graveyard …")
- Umbris, Fear Manifest (`--as-card`): "Umbris gets +1/+1 for each card your
  opponents own in exile." (control "… in their graveyards.")
- Ennis, Debate Moderator (`--as-card`): "At the beginning of your end step,
  if one or more cards were put into exile this turn, put a +1/+1 counter on
  Ennis." (control "… into a graveyard …")

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, scope).
Deltas:

1. Verification budget (binding): one bounded census of every supported face
   whose raw text contains case-insensitive "exile" or " hand", plus 300 random covered controls
   (`cargo xtask english-v3 diff --manifest … --random 300 --seed 20261008`),
   with the identity manifest, 12 workers and `--samples-per-face 0`; compare
   it with `cargo xtask english-v3 diff` against the claim parent's census of
   the same manifest. Never run `--all`. Tests: `cargo test -p` and
   `cargo clippy -p … --tests -- -D warnings` for every crate whose code or consumed data changed (`deckmaste_english_v3` always; `deckmaste_lexical_source` if core.ron/verbs.ron/vocabulary.rs changed). No xtask tests, no
   Lean checks, no post-refresh re-census unless refresh touched grammar,
   lexicon or core. New tests go under tests/suite/ registered in
   tests/main.rs. Evidence lives under the workspace's ignored
   `target/english-v3/`, never `/tmp`.
2. Probe each witness before implementation and record the counts; write each
   witness as a test. Run the tests and clippy once, at the end.
3. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling.
4. Compare per-face Reading counts against the claim-parent census. Any
   decrease on a previously covered face is restored, or named and justified
   in the landing record; a decrease inherited from trunk is attributed by
   change id.
5. Spot-check at least 10 newly covered faces and list them by name with their
   Reading. A bare *exile* read as the verb, a bare zone noun accepted after a preposition the ticket did not declare, or *in hand* read with an elided possessor rather than as a bare NP, is a defect. A wrong analysis that parses is a defect, not a gain.
6. No guard may name a lexeme, construction, verb, noun, preposition or card;
   read declared features only. Tests are re-spelled, never deleted. No new
   xtask subcommands, flags, fixtures or tooling.
7. May edit `grammar:` frame fields in `plugins_v2` keyword actions and lexicon
   frames in `crates/deckmaste_lexical_source/lexicon/*.ron`; plugin bodies
   untouched. Lexicon additions must be attested in the supported corpus
   (pruning rule); check attestation with `cargo xtask english-v3 grep`.
   Data over code: a declaration an existing Construction already reads is
   preferred to a grammar change.
8. A new frame or construction must not overlap an existing one on the same
   string (two labels for one constituency is a spurious duplicate, not an
   ambiguity); when it supersedes one, retire the old route and re-spell its
   tests. Prefer refining an existing Construction: the count is above the
   ADR's 250 ceiling.
9. Retire a superseded route on both the lexicon and the grammar side; do not
   leave unreachable declarations.
10. A CGEL citation may back only what the cited passage itself says; a project
    or orchestrator ruling is cited as a ruling, never attributed to CGEL.
11. Per the R1b ruling in Analysis, *from exile* after a noun keeps both its
    VP and NP attachments; do not STOP on it.

## Out of scope

- Re-applying the deferred from-M exclusion.
- *from anywhere* (separate ticket) and *from among cards … in exile* hosts
  whose controls also fail.
- Deleting the legacy `BareLocative*` declarations.

## Landing record

1. **Census delta**: bounded before → after (covered faces, Readings); lost
   faces classified by name; decreases, increases, digest-only changes.
2. **Witnesses**: each witness's Reading count before and after.
3. **Spot checks**: ten newly covered faces with their selected Reading.
4. **Deviations and STOPs**: constructions or tests added or removed beyond
   the ticket's letter, with justification; every STOP and its resolution;
   any glossary gap.
5. **Tests**: commands and outcomes; restored, re-spelled, ignored with
   blockers, added, removed.
6. **Timing**: one bounded census wall time with worker count.
