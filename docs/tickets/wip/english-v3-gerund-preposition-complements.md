---
needs: []
---
# Read gerund-participial Complements of prepositions: without paying its mana cost

## Why

Prepositions other than *by* reject a gerund-participial Complement.
On claim parent `pvmzprszmnpt` (32,828 supported faces, 21,486 covered,
11,342 unread; wave-3 recon of 2026-10-07), the recon buckets "without paying" and "instead of + gerund" read
together touch **360** unread faces and are the sole cause on **174**.
Counts are unread faces *touched* (at least one localised failing unit
matches) / *sole* (every failing unit matches and no other recon STRONG bucket
does). They are surface counts, not gain forecasts.

Probes (admitted roots): "Draw a card by paying {2}." 1; "Draw a card after
paying {2}." 0; "You may cast it without paying {2}." 0; "You may cast it
without paying its mana cost." 0, while "You may cast it." 1; "If that spell is
countered this way, exile it instead of putting it into its owner's
graveyard." 0, while "… exile it instead." 1.

This ticket takes over from `english-v3-systemic-residuals` the Instead landing
follow-up of 2026-10-07: the missing selected gerund-participial Complement
beneath *of* inside the compound PP *instead of* (53 faces at that stamp).

## Goal

*without* and *of* (inside *instead of*) take a gerund-participial clause as
Complement, licensed by a declared feature on the preposition the way the
existing `GerundComplementPreposition` route licenses *by*; the gerund's Object
and Adjuncts read as in any clause (*putting it into its owner's graveyard*,
*paying their mana costs*).

## Analysis

Gerund-participials occur very freely as complement of a preposition (CGEL,
Ch. 14, §8.3, p. 1262, [21]). *Instead of* is the compound-PP host recorded in
the [Instead post-landing fix](../done/english-v3-instead-replacement.md#post-landing-fix-2026-10-07);
its nominal Complement route stays.

## Witnesses

Each witness's failing units all match this construction and no other recon
STRONG bucket (`pvmzprszmnpt`, 0 Readings each):

- Storm of Memories: "You may cast it without paying its mana cost."
- Spellweaver Volute: "You may cast the copy without paying its mana cost."
  (control "You may cast the copy." 1)
- Knowledge Exploitation: "You may cast that card without paying its mana
  cost."
- Syncopate: "If that spell is countered this way, exile it instead of putting
  it into its owner's graveyard."
- The Five Doctors: "If this spell was kicked, put those cards onto the
  battlefield instead of putting them into your hand."

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, gate scope,
cite check). Deltas:

1. Verification budget (binding): One full census (`--all --workers 12
   --samples-per-face 0`) on the finished feature for the record, compared
   face by face (per-face `readings_sha256` and counts) against the claim
   parent's census; iterate on selector subsets, never on --all. Tests:
   `cargo test -p deckmaste_english_v3` and clippy on it; plus
   `-p deckmaste_lexical_source` only if core.ron/verbs.ron changed. No xtask
   tests, no Lean checks, no post-refresh re-census unless refresh touched
   grammar or lexicon. New tests go under tests/suite/ registered in
   tests/main.rs. Evidence lives under the workspace's ignored
   `target/english-v3/`, never `/tmp`.
2. Write each witness as a test first.
3. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling.
4. Compare per-face Reading counts against the claim-parent census. Any
   decrease on a previously covered face is restored, or named and justified in
   the landing record; a decrease inherited from trunk is attributed by change
   id.
5. Spot-check at least 10 newly covered faces and list them by name with their
   Reading. The gerund read as a noun Premodifier or nominal Object, or
   the PP attached inside the Object NP, is a defect. A wrong analysis that parses is a defect, not a gain.
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

## Out of scope

- *anywhere else* (*instead of putting it anywhere else*): *else* is a vocabulary
  gap; note the faces.
- *rather than paying*: `english-v3-rather-than-complements`.
- Gerunds with genitive Subjects.

## Landing record

In addition to the standard record: which prepositions gain the licence and
from which declared feature; the residual item discharged; Reading counts of
each witness; before/after counts stamped with change ids; timings as integer
ns and ns/B with host load and worker count.
