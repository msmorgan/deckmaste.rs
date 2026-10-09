---
needs: []
---
# Read transitive *turn … face up/down*: When this creature is turned face up, untap it.

## Why

*turn* has only an intransitive *turn face up* frame, so no passive and no
Object reads. On census `qsupksky` (default `@` `mqrlzszk` is empty; 32,828 supported faces, 25,474 covered, 7,354 unread; wave-5 recon of 2026-10-08), the recon bucket "turn <Object> face up/down
(transitive, active and passive)" touches **167** unread faces and is the sole
cause on **120** (mostly morph *is turned face up* triggers). Counts are unread faces *touched* (at least one localised failing unit matches) / *sole* (every failing unit matches and no other recon STRONG bucket does). They are surface counts, not gain forecasts.

Probes (`--readings 1`): "When this creature is turned face up, untap it." 0,
*is tapped* 1; "Turn this creature face up." 0; "This creature turns face up."
1.

## Goal

*turn* takes an Object followed by *face up* or *face down*, active and
passive, through the general passive routes, with byte-exact realisation.

## Analysis

Existing route: `crates/deckmaste_lexical_source/lexicon/verbs.ron:184–186`
declares Turn with `Predicate([Literal("face"), Literal("up")])` only.
Add `[ObjectNounPhrase, Literal("face"), Literal("up")]` and the *down* frame;
the passive comes from the existing `PassivePredicate` /
`ParticipialAuxiliaryPredicate` routes. Keep the intransitive frame only if
the supported corpus attests it (pruning rule: measure, then decide). No
CGEL citation applies: *face up/down* are declared frame literals, as today.

## Witnesses

Each is the face's only failing unit at `qsupksky` (0 Readings), each
control 1:

- Liege of the Axe: "When this creature is turned face up, untap it."
  (control "When this creature is tapped, untap it.")
- Shaleskin Plower: "When this creature is turned face up, destroy target
  land." (control "… is tapped, …")
- Daru Mender: "When this creature is turned face up, regenerate target
  creature." (control "… is tapped, …")
- Aven Farseer: "Whenever a permanent is turned face up, put a +1/+1 counter
  on this creature." (control "… is tapped, …")
- Zimone, Mystery Unraveler: "Otherwise, you may turn a permanent you control
  face up." (control "Otherwise, you may tap a permanent you control.")

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, scope).
Deltas:

1. Verification budget (binding): one bounded census of every supported face
   whose raw text contains case-insensitive "face up" or "face down", plus 300 random covered controls
   (`cargo xtask english-v3 diff --manifest … --random 300 --seed 20261008`),
   with the identity manifest, 12 workers and `--samples-per-face 0`; compare
   it with `cargo xtask english-v3 diff` against the claim parent's census of
   the same manifest. Never run `--all`. Tests: `cargo test -p` and
   `cargo clippy -p … --tests -- -D warnings` for `deckmaste_lexical_source` and `deckmaste_english_v3`. No xtask tests, no
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
   Reading. *face up* read as a depictive Adjunct or postmodifier instead of the frame, or a passive *turned face up* read with both the old and the new frame, is a defect. A wrong analysis that parses is a defect, not a gain.
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

## Out of scope

- *face-down* as a premodifier (*a face-down creature*) and *manifest*,
  *cloak*, *morph* keyword bodies.

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
