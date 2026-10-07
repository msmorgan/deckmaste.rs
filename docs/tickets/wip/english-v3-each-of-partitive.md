---
needs: []
---
# Read explicit partitive *each of*: each of them, each of up to two target creatures

## Why

*Each of* never reads. On change `xxknlzypsnwy` (32,828 supported faces, 17,322
covered, 15,506 unread; recon of 2026-10-07), it touches **356** unread faces
and is the sole cause on **146** (*each of them/those/these* 142 touched, *each
of your …* 99, *each of up to/one/two … target* 90). Counts are unread faces
*touched* (at least one localised failing unit matches) / *sole* (every failing
unit matches and no other recon STRONG bucket does). They are surface counts,
not gain forecasts.

Probes (admitted roots): "Put a +1/+1 counter on each creature." 1; "… on each
of them." 0; "… on each of those creatures." 0; "… on each of two target
creatures." 0; "Each of them gets +1/+1." 0; "Draw a card for each of them." 0;
"You may play an additional land on each turn." 1; "… on each of your turns." 0.

## Goal

*each of NP* reads as an explicitly partitive fused-head NP: *each* as fused
determiner-head with an *of* + partitive-oblique Complement, in every NP
position where *each N* already reads (Object, Subject, Complement of *on*,
*to*, *for*, *from*). The partitive oblique is a plural NP: a pronoun (*them*),
a determined NP (*those creatures*, *your turns*, *your opponents*) or a
targeted NP (*up to two target creatures*). Singular agreement follows the
fused head (*Each of them gets*).

## Analysis

In the explicitly partitive fused-head construction the head is followed by a
Complement of *of* + a partitive oblique, and the matrix NP denotes a subset of
the set the oblique denotes (*some of the books*, *all of them*; CGEL, Ch. 5,
§9.1, p. 411, [6]). CGEL places *each* with the determinatives that occur in
this partitive subtype (§9.2, p. 413). Targeted obliques (*up to two target
creatures*) are Oracle-specific; admitting them is the project's decision and
is recorded as such.

## Witnesses

Each witness's failing units all match this construction and no other recon
STRONG bucket (`xxknlzypsnwy`):

- Succumb to the Cold: "Put a stun counter on each of them."
- Involuntary Cooldown: "Put two stun counters on each of them."
- Reap What Is Sown: "Put a +1/+1 counter on each of up to three target
  creatures."
- Felidar Savior: "When this creature enters, put a +1/+1 counter on each of up
  to two other target creatures you control."
- Exploration: "You may play an additional land on each of your turns."
- Absolute Virtue: "You have protection from each of your opponents."

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, gate scope,
cite check). Deltas:

1. Before: `cargo xtask english-v3 --all --workers 12 --samples-per-face 0
   --output target/english-v3/english-v3-each-of-partitive-before.json` on the
   claim parent, stamped with its change id and covered count; after: the same
   command to `target/english-v3/english-v3-each-of-partitive-after.json` on
   the final tree. Evidence lives under the workspace's ignored
   `target/english-v3/`, never `/tmp`.
2. Write each witness as a test first. Iterate on `--face-id` selectors; verify
   on `--all` at the end.
3. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling.
4. Compare per-face Reading counts against the claim-parent census. Any
   decrease on a previously covered face is restored, or named and justified in
   the landing record.
5. Spot-check at least 10 newly covered faces and list them by name with their
   Reading. *of them* read as a postmodifier of a non-fused *each*, or plural
   agreement with *each of them*, is a defect. A wrong analysis that parses is
   a defect, not a gain.
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

- Distributive *each* after a plural Subject (*they each get*; recon bucket 74
  / 39) and quantificational adjunct *each* (§9.2, p. 413).
- Other partitive heads (*one of*, *any of*, *all of*, *none of*): note how
  many read after this change; widen only if the same declared feature licenses
  them, and record it.
- Coordinated targeted obliques (*up to one target creature, up to one target
  player, and/or …*): composition of the oblique is not built here.

## Landing record

In addition to the standard record: the fused-head structure and the feature
that licenses *each* in it; agreement evidence; per oblique type (pronoun /
determined / targeted) before/after counts stamped with change ids; timings as
integer ns and ns/B with host load and worker count.
