---
needs: []
---
# Read restrictive focusing *only* on clause-final Adjuncts: activate only as a sorcery, cast only during your turn

## Why

Focusing *only* never reads. On change `xxknlzypsnwy` (32,828 supported faces,
17,322 covered, 15,506 unread; recon of 2026-10-07), *only* before a
clause-final Adjunct (*only during*, *only if*, *only before*, *only as a
sorcery*) touches **930** unread faces and is the sole cause on **472** (recon
buckets "activation/timing restriction" 1,138 / 84 and "only if / only during /
only before" 938 / 30, read together). Counts are unread faces *touched* (at
least one localised failing unit matches) / *sole* (every failing unit matches
and no other recon STRONG bucket does). They are surface counts, not gain
forecasts.

Probes (admitted roots): "Draw a card during your turn." 1; "Draw a card only
during your turn." 0; "Draw a card if you control a Zombie." 1; "… only if you
control a Zombie." 0; "Activate during your turn." 1; "Activate only during
your turn." 0; "Activate as a sorcery." 0; "Draw a card only." 0.

This ticket takes over from `english-v3-systemic-residuals` the
only-if/only-during item ("Only-if/only-during has no other live owner and
stays here") and the orchestrator resolution of 2026-10-06 that left *Activate
only as a sorcery* with that residual, including its deferred widening: the
position licence for clause-final predicative *as* (the fixed-cost landing
licensed only the preposed, comma-separated Adjunct).

## Goal

*only* reads as a restrictive focusing modifier of the clause-final Adjunct it
precedes (PP *during your turn*, *as a sorcery*; conditional *if* clause;
temporal *before* clause), in imperative and finite clauses. Separately and
measured, the clause-final position is licensed for the predicative *as* PP
(*Activate as a sorcery.*), reusing the declaration the fixed-cost landing made
for the preposed position; the bare form gains no second analysis (Method 8).
Coordinated restrictions (*only once each turn and only if …*) compose through
ordinary coordination.

## Analysis

*Only* is a restrictive focusing modifier (CGEL, Ch. 6, §7.3.1, p. 587, [41]).
Focusing modifiers combine with a wide range of constructions, among them PPs
(*only in America*), declarative content clauses, *to*-infinitivals and
imperative clauses (p. 587, [42]); the relation between the modifier's linear
position and its focus is treated at p. 589. The predicative *as* PP is the one
recorded in the fixed-cost landing; extending its position licence is an
orchestrator-assigned item, not a CGEL claim.

## Witnesses

Each witness's failing units all match this construction and no other recon
STRONG bucket except the two restriction buckets (`xxknlzypsnwy`):

- Mind Slash: "Activate only as a sorcery."
- Celestial Enforcer: "Activate only if you control a creature with flying."
- Hammer of Bogardan: "Activate only during your upkeep."
- Flywheel Racer: "Activate only if this permanent is a creature."

*Cast this spell only during …* faces are mostly multi-cause at this stamp
(Surprise Deployment's "Cast this spell only during combat." also needs
`english-v3-combat-interval-noun`); use them as composition checks, not gain
claims.

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, gate scope,
cite check). Deltas:

1. Before: `cargo xtask english-v3 --all --workers 12 --samples-per-face 0
   --output target/english-v3/english-v3-focusing-only-before.json` on the
   claim parent, stamped with its change id and covered count; after: the same
   command to `target/english-v3/english-v3-focusing-only-after.json` on the
   final tree. Evidence lives under the workspace's ignored
   `target/english-v3/`, never `/tmp`.
2. Write each witness as a test first. Iterate on `--face-id` selectors; verify
   on `--all` at the end.
3. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling.
4. Compare per-face Reading counts against the claim-parent census. Any
   decrease on a previously covered face is restored, or named and justified in
   the landing record.
5. Spot-check at least 10 newly covered faces and list them by name with their
   Reading. *only* attached to the verb or Object when the focus is the
   following Adjunct, or *as a sorcery* read as an Object-oriented depictive,
   is a defect. A wrong analysis that parses is a defect, not a gain.
6. No guard may name a lexeme, construction, verb, noun, preposition or card;
   read declared features only. Tests are re-spelled, never deleted. No new
   xtask subcommands, flags, fixtures or tooling.
7. May edit `grammar:` frame fields in `plugins_v2` keyword actions and lexicon
   frames in `crates/deckmaste_lexical_source/lexicon/*.ron`; plugin bodies
   untouched. Lexicon additions must be attested in the supported corpus
   (pruning rule).
8. A new frame or construction must not overlap an existing one on the same
   string (two labels for one constituency is a spurious duplicate, not an
   ambiguity); when it supersedes one, retire the old route and re-spell its
   tests.
9. Retire a superseded route on both the lexicon and the grammar side; do not
   leave unreachable declarations.
10. A CGEL citation may back only what the cited passage itself says; a project
    or orchestrator ruling is cited as a ruling, never attributed to CGEL.
11. Timings as integer ns and ns/B, with host load and worker count.
12. Gate: `cargo xtask gate --changed --from <claim> --run`.

## Out of scope

- Frequency *once each turn* (*Activate only once each turn*, 80 sole):
  `english-v3-once-frequency`, which needs this ticket.
- *before attackers are declared*: *declared* is a vocabulary gap at
  `xxknlzypsnwy`; note the faces, do not add the verb here.
- Focusing *only* on NPs inside Determiners or on Subjects (*Only creatures
  …*), and additive *also*/*even*.
- Spend restrictions (*Spend this mana only to cast …*): they also need a
  purpose infinitival Adjunct that does not read (*Pay {1} to draw a card.* 0);
  count them as residue.

## Landing record

In addition to the standard record: the attachment chosen per focus category,
and the as-position widening reported separately (its own before/after counts
and Reading counts on the fixed-cost witnesses); the residual items discharged;
before/after counts stamped with change ids; timings as integer ns and ns/B
with host load and worker count.
