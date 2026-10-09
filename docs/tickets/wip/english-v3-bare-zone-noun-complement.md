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

1. **Census delta**: bounded `mqrlzszk` census snapshot (source-equivalent to
   `qsupksky` and `vltmyvrw` at claim time) → `plkpqnqw`: 4,456 → 4,615
   covered faces; 653,907 → 2,364,419 Readings. Gained 159; lost none;
   decreases 0, increases 0, digest-only 0. Selector: supported analysed text
   matching case-insensitive `exile| hand|on top` (6,185 faces), unioned with
   300 covered controls selected by `english-v3 diff`, seed 20261008:
   6,424 identities in `target/english-v3/english-v3-bare-zone-noun-complement/manifest.json`.
   `diff.json` compares that identical manifest; no identity or source-hash
   differences. All 6,424 faces completely enumerated at `plkpqnqw`, with
   exact realization and structural traversal checked: 0 issues, internal
   failures, duplicate Readings, limited or undetermined faces. The required
   post-refresh bounded recheck at `uwtynnll` retained exactly those totals:
   `diff-trunk.json` reports all 6,424 faces unchanged, including digests;
   lost/gained/decreases/increases/digest-only/source changes all 0.
   The final integration refresh recheck at `pxnrzlqm` again enumerated all
   6,424 faces with 0 issues: covered 4,615 and Readings 2,364,419 unchanged;
   lost/gained/decreases/increases/source changes 0, digest-only 50, unchanged
   6,374 (`diff-integrate-refresh.json`). The 50 planeswalker digest changes
   come from the incoming loyalty-sign analysis in `okwzvwqr`.
2. **Witnesses**: probes before implementation at the `vltmyvrw` source all
   had 0 admitted roots/Readings; each control admitted a Reading. Complete
   witness-unit counts at `plkpqnqw`: Sea Gate Wreckage 7; Norin, Swift
   Survivalist 5; Wild-Magic Sorcerer 8; Umbris, Fear Manifest 7; Ennis,
   Debate Moderator 2. Full-face counts at `plkpqnqw`: respectively 7, 5,
   8, 322, 54. R1b's attested constituent *the first spell you cast from
   exile* retains both VP and NP attachment among 20 Readings at `plkpqnqw`.
3. **Spot checks**: ten newly covered faces at `plkpqnqw`, each with an
   inspected byte-exact Reading: Sea Gate Wreckage (7), Akki Underling (2)
   and Barren Glory (3): *in* + bare count NP *hand*; Norin, Swift Survivalist
   (5), Wild-Magic Sorcerer (8) and Appa, Steadfast Guardian (20): *from* +
   bare count NP *exile*; Umbris, Fear Manifest (322) and Goliath Daydreamer
   (1,253,700): *in* + bare count NP *exile*; Ennis, Debate Moderator (54)
   and Ashnod's Intervention (4): passive *put* + *into* + bare count NP
   *exile*. Each bare NP has its declared noun owner and no elided possessor.
4. **Deviations and STOPs**: at `plkpqnqw`, refined the existing Boolean
   licences to matching Location/SpatialEdge permissions on both grammar
   and lexicon sides: the data-only experiment admitted unattested *on exile*.
   Added 1 Construction, `BareNominalPrepositionPassive`, because the
   existing selected passive requires an AccusativePhrase, excluding bare
   NPs; the compiler requires every form to retain the same field Categories.
   The new route admits only licensed bare complements and is disjoint from
   the existing determined route. No frames or existing Constructions added
   or retired beyond that addition; obsolete Boolean values removed on both
   sides, legacy BareLocative declarations retained as scoped. Added control,
   exclusion, R1b and independent-value assertions alongside the required
   witnesses. An initial test build was queued prematurely and failed before
   test execution because updated tests saw the older grammar artifact; the
   completed source's final build passed. No STOPs. No glossary gaps remain:
   updated Bare Preposition Use and Bare Nominal Complement; the spatial-edge
   analysis remains attributed to its project ruling. Refresh brought in
   landwalk derivation (`wtnuopxm`) and the face-down semantics landing
   (`qnqztyrl`); the required bounded recheck at `uwtynnll` found no inherited
   Reading decreases or other per-face changes. The integration refresh
   brought pruning changes `okwzvwqr` and `uppoqxqy`; the bounded recheck
   at `pxnrzlqm` found no inherited Reading decreases, only the 50 attributed
   loyalty-sign digest changes.
5. **Tests**: at `plkpqnqw`, `cargo test -p deckmaste_english_v3` passed
   (588 tests; initial premature build failed as disclosed above);
   `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings` passed;
   `cargo test -p deckmaste_lexical_source` passed (61 tests);
   `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings` passed.
   Assurance counts at `plkpqnqw`: restored 0, re-spelled 0, ignored with
   blockers 0, added 8, removed 0. The same four commands passed again after
   dependency changes from refresh, at `uwtynnll`: English-v3 588 tests;
   lexical-source 62 tests, including the incoming class-selection test from
   `wtnuopxm`. All four commands passed again after the final integration
   refresh, at `pxnrzlqm`: English-v3 593 tests; lexical-source 62 tests;
   both clippy runs passed with warnings denied. Independently constructed
   PPs and the passive retain both
   roundtrip laws. No xtask tests or Lean checks run.
6. **Timing**: the landing's bounded census before refresh, at `plkpqnqw`,
   took 609.01 seconds
   wall time with 12 workers and `--samples-per-face 0`.
