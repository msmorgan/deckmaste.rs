---
needs: []
---
# Read *become* with an adjectival passive Complement: whenever this creature becomes blocked

## Why

*Becomes blocked* never reads. On change `xxknlzypsnwy` (32,828 supported
faces, 17,322 covered, 15,506 unread; recon of 2026-10-07), it touches **178**
unread faces and is the sole cause on **109** (recon bucket "becomes blocked /
blocks or becomes": 180 / 109). Counts are unread faces *touched* (at least one
localised failing unit matches) / *sole* (every failing unit matches and no
other recon STRONG bucket does). They are surface counts, not gain forecasts.

Probes (admitted roots): "Whenever this creature becomes blocked, draw a card."
0; "… becomes blocked by a creature, …" 0; "Whenever this creature is blocked,
draw a card." 1; "Whenever this creature becomes tapped, draw a card." 1;
"Whenever this creature blocks, draw a card." 1.

## Goal

A past participle reads as the predicative Complement of *become* (an
adjectival passive), with an optional *by* phrase where attested (*becomes
blocked by a creature*), in finite clauses and coordinated with *blocks*
(*blocks or becomes blocked*). Whatever route already reads *becomes tapped* is
either this route or is retired in its favour, recorded (Method 8): one
analysis per string.

## Analysis

A past participle that is the Complement of *become* is adjectival: *It became
magnetised* is an adjectival passive, with the change of state supplied by
*become* (CGEL, Ch. 16, §10.1.3, p. 1438, [39ii]). Adjectival passives also
occur with other verbs taking predicative complements (p. 1437). *By* phrase
complements are found in adjectival as well as verbal passives but are much
more restricted in the adjectival construction (p. 1439, [41]); whether
*becomes blocked by a creature* is admitted under that restriction is the
landing's decision, recorded.

## Witnesses

Each witness's failing units all match this construction and no other recon
STRONG bucket (`xxknlzypsnwy`):

- Flint Golem: "Whenever this creature becomes blocked, defending player mills
  three cards."
- Vedalken Ghoul: "Whenever this creature becomes blocked, defending player
  loses 4 life."
- Trained Cheetah: "Whenever this creature becomes blocked, it gets +1/+1 until
  end of turn."
- Somberwald Alpha: "Whenever a creature you control becomes blocked, it gets
  +1/+1 until end of turn."
- Talruum Champion: "Whenever this creature blocks or becomes blocked by a
  creature, that creature loses first strike until end of turn."

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, gate scope,
cite check). Deltas:

1. Before: `cargo xtask english-v3 --all --workers 12 --samples-per-face 0
   --output target/english-v3/english-v3-become-adjectival-passive-before.json`
   on the claim parent, stamped with its change id and covered count; after:
   the same command to
   `target/english-v3/english-v3-become-adjectival-passive-after.json` on the
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
   Reading. A *becomes blocked* Reading with *blocked* as a finite or
   Object-taking verb, or two analyses of *becomes tapped*, is a defect. A
   wrong analysis that parses is a defect, not a gain.
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

- *for each creature blocking it beyond the first* (Jungle Wurm and twins).
- *becomes the target of*, *becomes a copy of*, *becomes untapped* beyond what
  the same route gives; record any that start to read.
- *is blocked* (already reads) and the verbal *be* passive.

## Landing record

In addition to the standard record: the route that reads *becomes tapped* and
*becomes blocked*, and any retired route; the *by*-phrase decision; Reading
counts of each witness; before/after counts stamped with change ids; timings as
integer ns and ns/B with host load and worker count.
