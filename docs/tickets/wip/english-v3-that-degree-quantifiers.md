---
needs: []
---
# Read degree *that* with *many*/*much*: that many cards, that much life

## Why

*That many* / *that much* never read. On change `xxknlzypsnwy` (32,828
supported faces, 17,322 covered, 15,506 unread; recon of 2026-10-07), the two
strings touch **590** unread faces and are the sole cause on **228** (recon
bucket "many/much": 602 / 229; *that many* 378 touched, *that much* 224, *twice
that many* 38). Counts are unread faces *touched* (at least one localised
failing unit matches) / *sole* (every failing unit matches and no other recon
STRONG bucket does). They are surface counts, not gain forecasts.

Probes (admitted roots): "You gain that much life." 0; "Mill that many cards."
0; "Destroy that many creatures." 0; "You gain 2 life." 1. The lexemes exist:
`vocab:Determinative/Many`, `vocab:Determinative/Much` and
`vocab:SingularDemonstrative/That` in `core.ron`. The gap is the Determinative
Phrase with a degree modifier, not vocabulary.

## Goal

*that many* and *that much* read as a Determinative Phrase (head *many*/*much*,
degree modifier *that*) in Determiner function of a plural count or non-count
NP (*that many cards*, *that much life*, *that much damage*). The DP composes
through the shared Quantitative Determiner interface used by numerals and *up
to N*; do not build a second counted-NP family. Number agreement follows the
head: *many* with plural count, *much* with non-count.

## Analysis

The degree determinatives *many*, *much*, *few* and *little* take degree
modifiers like gradable adjectives, including *very*, *so*, *too*, *how*,
*this* and *that*, forming a Determinative Phrase (CGEL, Ch. 5, §11(c), p. 431,
[3]). *That* used anaphorically as a degree modifier (*Kim is about that old
too*) is described at Ch. 6, §3.2, p. 549. The anaphoric resolution of the
degree (the damage dealt, the life gained) is semantics, outside this grammar
ticket.

## Witnesses

Each witness's failing units all match this construction and no other recon
STRONG bucket (`xxknlzypsnwy`):

- Tamanoa: "Whenever a noncreature source you control deals damage, you gain
  that much life."
- Mourning Thrull: "Whenever this creature deals damage, you gain that much
  life."
- Crosstown Courier: "Whenever this creature deals combat damage to a player,
  that player mills that many cards."
- Guilty Conscience: "Whenever enchanted creature deals damage, this Aura deals
  that much damage to that creature."
- Firedrinker Satyr: "Whenever this creature is dealt damage, it deals that
  much damage to you."

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, gate scope,
cite check). Deltas:

1. Before: `cargo xtask english-v3 --all --workers 12 --samples-per-face 0
   --output target/english-v3/english-v3-that-degree-quantifiers-before.json`
   on the claim parent, stamped with its change id and covered count; after:
   the same command to
   `target/english-v3/english-v3-that-degree-quantifiers-after.json` on the
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
   Reading. *that* read as a demonstrative Determiner of a fused-head
   *many*/*much*, or *that much life* read with *life* outside the NP, is a
   defect. A wrong analysis that parses is a defect, not a gain.
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

- *twice that many* / *three times that much* (multiplier before the DP): note
  how many become readable; a multiplier modifier is a separate dependent.
- Comparative *as much X as …*, *how many*, *the same number of*.
- *that many* as a fused head without a noun (*draw that many*): include only
  if attested and the fused-head route already exists for numerals; record it.

## Landing record

In addition to the standard record: the DP structure chosen and the interface
it composes through; agreement evidence for *many* vs *much*; the disposition
of *twice that many*; before/after counts stamped with change ids; timings as
integer ns and ns/B with host load and worker count.
