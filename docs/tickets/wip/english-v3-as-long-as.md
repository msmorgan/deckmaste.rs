---
needs: []
---
# Read "as long as" conditional adjuncts

## Why

*As long as* + clause, before or after its host clause, never reads. On change
`wlvwtnppyovn` (32,828 supported faces, 13,716 covered, 19,112 unread) it
occurs in a failing unit of 1,083 unread faces and is the only recognised
cause on 323. These are surface-bucket counts, not gain forecasts. Minimal
pairs on that change (admitted roots): "This creature has flying as long as
you control an artifact." 0, "… if you control an artifact." 1; "As long as
you control a Dragon, this creature has flying." 0, "If you control a Dragon,
…" 1.

## Goal

*As long as* + finite clause reads as a conditional Adjunct, clause-final and
clause-initial (with its comma), with the same host breadth as *if*. *For as
long as* + clause ("gain control of target artifact for as long as you control
this creature") reads as a duration Adjunct. Every ambiguity the corpus does
not resolve stays as multiple Readings; for example, clause-final *as long as*
may attach to a coordinated predicate or to one Conjunct.

## Analysis

CGEL lists *as long as* (with *so long as*) among the items that govern a
content-clause protasis in place of *if*. It excludes the subordinator *that*
and expresses a necessary and sufficient condition (CGEL, Ch. 8, §14.4,
p. 758, [61i], [62ii]). Like *if*, the governing item is a Preposition taking a
Clause Complement (glossary: Subordinator, "A Preposition taking a Clause
Complement is still a Preposition"). Its phrase functions as a conditional
Adjunct. Two lexical analyses are possible: one multiword lexeme *as long as*
(a Compound Preposition), or composition from the scalar-equality *as … as*
governor (CGEL, Ch. 13, §1.3, p. 1104, [15i]) with *long*. The conditional
meaning is not compositional from duration, so test both against the corpus.
If they give different Reading sets on the witnesses, STOP and report before
choosing. Its Preposition Function Licence admits only the clause Adjunct
function unless the corpus attests another; the pruning rule in
`docs/decisions/english-lexical-analysis.md` governs any exclusion.

## Witnesses

- Aeronaut Tinkerer: "This creature has flying as long as you control an
  artifact."
- Kargan Dragonrider: "As long as you control a Dragon, this creature has
  flying."
- Arisen Gorgon: "This creature has deathtouch as long as you control a
  Liliana planeswalker."
- Pristine Angel: "As long as this creature is untapped, it has protection
  from artifacts and from each color."
- Master Thief: "When this creature enters, gain control of target artifact
  for as long as you control this creature."
- Manor Gargoyle: "This creature has indestructible as long as it has
  defender." (also has an unrelated failing unit: "{1}: Until end of turn, this
  creature loses defender and gains flying.")

The first five witnesses each have only the quoted failing unit.

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, gate scope,
cite check). Deltas:

1. Before: `cargo xtask english-v3 --all --workers 12 --samples-per-face 0
   --output target/english-v3/as-long-as-before.json` on the claim parent,
   stamped with its change id.
2. Write the witnesses as tests first, each with its *if* twin. The two must
   have the same Reading count and structure modulo the governing Preposition.
3. Iterate on `--face-id` selectors; verify on `--all` at the end.
4. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling.
5. Spot-check every newly covered face's selected analysis. The landing record
   lists at least 10 by name with their Reading. *long* read as an Adjective
   predicated of a participant, or *as long* read as a degree Modifier of the
   preceding clause, is a defect, not a gain.
6. No guard may name a lexeme, construction, verb, noun, preposition or card;
   read declared features only. Tests are re-spelled, never deleted. No new
   xtask subcommands, flags, fixtures or tooling.
7. May edit `grammar:` frame fields in `plugins_v2` keyword actions and lexicon
   frames in `crates/deckmaste_lexical_source/lexicon/*.ron`; plugin bodies
   untouched. Lexicon additions must be attested in the supported corpus
   (pruning rule).
8. Gate: `cargo xtask gate --changed --from <claim> --run`.
9. A CGEL citation may back only what the cited passage itself says; a project
   or orchestrator ruling is cited as a ruling, never attributed to CGEL.
10. Retire a superseded route on both the lexicon and the grammar side; do not
    leave unreachable declarations.

## Out of scope

- *as though* (264 touched / 63 sole) and *for as long as* outside a duration
  Adjunct.
- Host-clause causes unrelated to the Adjunct. Count a face only when it
  wholly reads.

## Landing record

In addition to the standard record: the lexical analysis chosen, with both
candidates' Reading counts on the witnesses; the *if*-twin comparison;
timings as integer ns and ns/B with host load and worker count.
