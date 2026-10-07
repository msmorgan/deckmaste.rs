---
needs: []
---
# Read causative *have* with an Object and bare infinitival: you may have target player mill two cards

## Why

Causative *have* never reads. On change `xxknlzypsnwy` (32,828 supported faces,
17,322 covered, 15,506 unread; recon of 2026-10-07), *(may) have* + NP + bare
infinitival touches **253** unread faces and is the sole cause on **101**
(recon bucket "causative have": 307 / 114, which also counts non-causative
*have all activated abilities*). Counts are unread faces *touched* (at least
one localised failing unit matches) / *sole* (every failing unit matches and no
other recon STRONG bucket does). They are surface counts, not gain forecasts.

Probes (admitted roots): "You may have target player mill two cards." 0; "Have
target player mill two cards." 0. *Have* declares `Predicate([Role("Object"),
Role("VerbPhrase")])` in `verbs.ron`; it is the `core-verb:Have` row
`Complement(Object), Complement(VerbPhrase)` of the unsupported-inventory table
in the done `english-v3-generic-frame-consumption` ("unsupported slot category
Object (Complement)"). This ticket takes that row over from
`english-v3-systemic-residuals`.

## Goal

*have NP VP* reads with *have* as a causative catenative: the NP is the Object
and the bare infinitival clause is the catenative Complement, with the Object
understood as its Subject (*have target player [mill two cards]*). It reads
under *may*, in imperatives and in finite clauses. The legacy `Role("Object"),
Role("VerbPhrase")` frame is replaced by typed slots the generic consumer
admits, and retired (Method 9).

## Analysis

Only a small number of catenatives take bare infinitivals; among the causatives
they are *have*, *let* and *make* (CGEL, Ch. 14, §5.6.2, p. 1244). The
intervening NP of *have target player mill …* is therefore the Object of *have*
in a complex catenative construction, not the Subject of a finite clause.

## Witnesses

Each witness's failing units all match this construction and no other recon
STRONG bucket (`xxknlzypsnwy`):

- Quill-Slinger Boggart: "Whenever a player casts a Kithkin spell, you may have
  target player lose 1 life."
- Rage Forger: "Whenever a creature you control with a +1/+1 counter on it
  attacks, you may have that creature deal 1 damage to target player or
  planeswalker."
- Joraga Bard: "Whenever this creature or another Ally you control enters, you
  may have Ally creatures you control gain vigilance until end of turn."
- Mirror Image: "You may have this creature enter as a copy of a creature you
  control."
- Ebon Dragon: "When this creature enters, you may have target opponent discard
  a card."

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, gate scope,
cite check). Deltas:

1. Before: `cargo xtask english-v3 --all --workers 12 --samples-per-face 0
   --output target/english-v3/english-v3-causative-have-before.json` on the
   claim parent, stamped with its change id and covered count; after: the same
   command to `target/english-v3/english-v3-causative-have-after.json` on the
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
   Reading. The bare infinitival read as a reduced relative on the Object, or
   *have* read as possessive with an Adjunct, is a defect. A wrong analysis
   that parses is a defect, not a gain.
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

- Possessive and granted-ability *have* (*has flying*, *have all activated
  abilities of …*): existing frames, untouched.
- Perfect *have* with an Object Gap (*spells you've cast*): owned by
  `english-v3-systemic-residuals`.
- *enter as a copy of …* inside the infinitival: reuse the existing Enter
  frames; do not build *as a copy* here.

## Landing record

In addition to the standard record: the replacement Have frame and the retired
legacy role slots; Reading counts of each witness; before/after counts stamped
with change ids; timings as integer ns and ns/B with host load and worker
count.
