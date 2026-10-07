---
needs: []
---
# Read the *where X is* clause that defines a variable

## Why

*Where X is …* never reads. On change `xxknlzypsnwy` (32,828 supported faces,
17,322 covered, 15,506 unread; recon of 2026-10-07), it touches **1,119**
unread faces and is the sole cause on **149**. Counts are unread faces
*touched* (at least one localised failing unit matches) / *sole* (every failing
unit matches and no other recon STRONG bucket does). They are surface counts,
not gain forecasts.

Probes (admitted roots): "This creature gets +X/+X until end of turn." 1; "…
until end of turn, where X is its power." 0; "Create X 1/1 green Saproling
creature tokens." 1; "Draw X cards, where X is your life total." 0. The host
with *X* reads; the clause does not. `vocab:Adverb/Where` exists in `core.ron`.

## Goal

A clause-final, comma-separated *where X is NP* reads as a supplementary
dependent of the clause (or of the keyword label, *Firebending X, where X is
…*) whose content clause is a specifying *be* clause with Subject *X*. One
analysis, attached once; *X* in the host stays a numeral-like quantity.

## Analysis

CGEL lists *where* among the items that govern non-expandable content clauses
(Ch. 11, §4.8, p. 971, [57]). In the fused-relative discussion of *when*,
*where* and *while*, it notes an alternative analysis treating them as
prepositions that take content clauses as complements, like *before* or
*whereas* (Ch. 12, §6, p. 1078, [30]). Neither passage discusses the
variable-defining use; treating *where X is NP* as a preposition with a
content-clause Complement in supplementary function is the project's analysis,
to be recorded as such. The relative *where* of Ch. 12, §3.5.3, p. 1050, [51]
takes locative antecedents and is not this construction.

## Witnesses

Each witness's failing units all match this construction and no other recon
STRONG bucket (`xxknlzypsnwy`):

- Chameleon Colossus: "{2}{G}{G}: This creature gets +X/+X until end of turn,
  where X is its power."
- Wild Beastmaster: "Whenever this creature attacks, each other creature you
  control gets +X/+X until end of turn, where X is this creature's power."
- Hemosymbic Mite: "Whenever this creature becomes tapped, another target
  creature you control gets +X/+X until end of turn, where X is this creature's
  power."
- Elenda, the Dusk Rose: "When Elenda dies, create X 1/1 white Vampire creature
  tokens with lifelink, where X is Elenda's power."
- Tip the Scales: "When you do, all creatures get -X/-X until end of turn,
  where X is the sacrificed creature's toughness."

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, gate scope,
cite check). Deltas:

1. Before: `cargo xtask english-v3 --all --workers 12 --samples-per-face 0
   --output target/english-v3/english-v3-where-variable-clause-before.json` on
   the claim parent, stamped with its change id and covered count; after: the
   same command to
   `target/english-v3/english-v3-where-variable-clause-after.json` on the final
   tree. Evidence lives under the workspace's ignored `target/english-v3/`,
   never `/tmp`.
2. Write each witness as a test first. Iterate on `--face-id` selectors; verify
   on `--all` at the end.
3. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling.
4. Compare per-face Reading counts against the claim-parent census. Any
   decrease on a previously covered face is restored, or named and justified in
   the landing record.
5. Spot-check at least 10 newly covered faces and list them by name with their
   Reading. The *where* clause read as a relative clause on the nearest NP
   (*end of turn, where …*) or as a locative Adjunct is a defect. A wrong
   analysis that parses is a defect, not a gain.
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

- What *X* is equated to when that NP fails on its own (*the greatest power
  among …*, *the number of …*, *the amount of life you gained this turn*): note
  how many become readable; those NPs have other owners.
- Magma Sliver's scalar Subject *X* (`english-v3-systemic-residuals`).
- Semantic binding of *X*: `docs/decisions/oracle-text-is-forward-anaphoric.md`
  governs it; this ticket is grammar only.

## Landing record

In addition to the standard record: the category and attachment chosen and
which section of the record states it as a project analysis; how many faces
remain on the equated NP; before/after counts stamped with change ids; timings
as integer ns and ns/B with host load and worker count.
