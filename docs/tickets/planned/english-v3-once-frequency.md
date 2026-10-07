---
needs: [english-v3-focusing-only]
---
# Read bounding frequency *once* with a distributive NP: only once each turn

## Why

*Once each turn* never reads. On change `xxknlzypsnwy` (32,828 supported faces,
17,322 covered, 15,506 unread; recon of 2026-10-07), *only once each turn*
touches **281** unread faces and is the sole cause on **80**; *once* without
*only* occurs on 11 (recon bucket "activation/timing restriction", read for
this string). Counts are unread faces *touched* (at least one localised failing
unit matches) / *sole* (every failing unit matches and no other recon STRONG
bucket does). They are surface counts, not gain forecasts.

Probes (admitted roots): "Draw a card each turn." 1; "Draw a card once." 0;
"Draw a card once each turn." 0; "Draw a card twice." 0; "Activate only once." 0.
The *each turn* NP Adjunct reads; *once* does not.

## Goal

*once* (and *twice*) read as a bounding frequency Adjunct, optionally followed
by a distributive temporal NP (*once each turn*), clause-finally. Combined with
`english-v3-focusing-only` (the `needs:` edge), *Activate only once each turn.*
and *Do this only once each turn.* read with *only* focusing the frequency
Adjunct.

## Analysis

The bounding frequency expressions are the adverbs *once* and *twice*, NPs
headed by *times*, and *on* + *occasion* PPs (CGEL, Ch. 8, §9, p. 715). *Once*
can also express temporal location (*I once liked this kind of music*; p. 715,
[8ib]); only the frequency use is attested in these witnesses, so the
temporal-location use is not added (pruning rule). How *each turn* combines
with *once* (a separate NP Adjunct or a dependent of *once*) is for the landing
to settle and record.

## Witnesses

Each witness's failing units all match this construction and no other recon
STRONG bucket except the restriction bucket (`xxknlzypsnwy`):

- Kozilek's Translator: "Activate only once each turn."
- Jeskai Devotee: "Activate only once each turn."
- Basking Rootwalla: "Activate only once each turn."
- Twinblade Slasher: "Activate only once each turn."
- Ondu Spiritdancer: "Do this only once each turn."

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, gate scope,
cite check). Deltas:

1. Before: `cargo xtask english-v3 --all --workers 12 --samples-per-face 0
   --output target/english-v3/english-v3-once-frequency-before.json` on the
   claim parent, stamped with its change id and covered count; after: the same
   command to `target/english-v3/english-v3-once-frequency-after.json` on the
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
   Reading. *once* read as a temporal-location Adjunct or as a subordinator
   (*once X happens*), or *each turn* read as the Object, is a defect. A wrong
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

- Subordinator *once* with a clause, *the first time each turn*, *N times*,
  *twice that many*.
- *Activate only once each turn and only if …*: coordination of the two
  restrictions is ordinary coordination; note it, do not build it.

## Landing record

In addition to the standard record: the frequency Adjunct's category and the
combination with *each turn*; whether *twice* is attested and added; Reading
counts of each witness; before/after counts stamped with change ids; timings as
integer ns and ns/B with host load and worker count.
