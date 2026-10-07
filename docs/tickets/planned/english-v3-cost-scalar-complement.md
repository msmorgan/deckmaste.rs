---
needs: []
---
# Read *cost* with its scalar Complement and hollow infinitival: costs {1} less to cast

## Why

Cost modification never reads. On change `xxknlzypsnwy` (32,828 supported
faces, 17,322 covered, 15,506 unread; recon of 2026-10-07), the sentence shape
*X cost(s) {N} less/more to cast/activate* touches **654** unread faces and is
the sole cause on **351** (recon bucket "cost modification": 672 / 360). Counts
are unread faces *touched* (at least one localised failing unit matches) /
*sole* (every failing unit matches and no other recon STRONG bucket does). They
are surface counts, not gain forecasts.

Probes (admitted roots): "This spell costs {1} less to cast." 0; "Spells you
cast cost {1} less." 0; "This spell costs {1} more." 0; even "This spell costs
{1}." 0. The verb is declared
(`crates/deckmaste_lexical_source/lexicon/verbs.ron`, `identity: Cost`) with
one frame of three legacy role slots, `Role("ManaAmount"),
Role("ComparisonDirection"), Role("ControlledCostAction")`. That frame is the
`core-verb:Cost` row of the unsupported-inventory table in the done
`english-v3-generic-frame-consumption`, which reports it as "unsupported slot
category ManaAmount (Complement)"; no consumer admits it.

This ticket takes over from `english-v3-systemic-residuals` (a) the Cost row of
that inventory (the `ManaAmount`/`ComparisonDirection`/`ControlledCostAction`
slot categories) and (b) Neonate's Rush, the one inherited frame-coordination
face recorded there as failing on cost reduction.

## Goal

*Cost* reads with a scalar-location Complement (a mana amount, optionally with
comparative *less*/*more*) and an optional hollow *to*-infinitival Complement
whose Object Gap is the Subject (*This spell costs {1} less to cast —*). The
Cost row is replaced by ordinary typed slots the generic frame consumer admits;
its three legacy role slots are retired (Method 9). Trailing *if* and *for
each* Adjuncts compose as they do elsewhere.

## Analysis

*Cost* is one of the verbs that express location on a scale with an NP
Complement: *A jar of coffee costs $12* beside *is $12* (CGEL, Ch. 8, §5.4, p.
693, [12ib]); the same passage notes that *cost* also permits an Object for the
payer (*That jar of coffee cost me $12*). *Cost* is among the transitive verbs
that take a hollow *to*-infinitival internal Complement: *The car cost over
$1,000 to repair —* (Ch. 14, §6.3(c), p. 1250, [17ii]); the gap in the
infinitival is linked to the Subject. NPs can modify adjectives as degree
expressions, including comparatives (*a great deal smaller*; Ch. 6, §3.2, p.
549, [38]). Whether *{1} less* is a measure NP modifying comparative *less*, or
*less* heads the Complement with *{1}* as its modifier, is for the landing to
settle from the declarations and to record; CGEL does not analyse this
mana-symbol string.

## Witnesses

Each witness's failing units all match this construction and no other recon
STRONG bucket (`xxknlzypsnwy`):

- Wizard's Lightning: "This spell costs {2} less to cast if you control a
  Wizard."
- Ghoultree: "This spell costs {1} less to cast for each creature card in your
  graveyard."
- Herald of the Pantheon: "Enchantment spells you cast cost {1} less to cast."
- Blossoming Tortoise: "Activated abilities of lands you control cost {1} less
  to activate."
- Glowrider: "Noncreature spells cost {1} more to cast."
- Inquisitive Glimmer (no infinitival): "Unlock costs you pay cost {1} less."

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, gate scope,
cite check). Deltas:

1. Before: `cargo xtask english-v3 --all --workers 12 --samples-per-face 0
   --output target/english-v3/english-v3-cost-scalar-complement-before.json` on
   the claim parent, stamped with its change id and covered count; after: the
   same command to
   `target/english-v3/english-v3-cost-scalar-complement-after.json` on the
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
   Reading. The infinitival read as a purpose Adjunct with an ordinary
   (non-hollow) Object, or *{1}* read as an Object of *cast*, is a defect. A
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

- Purpose *to*-infinitival Adjuncts in general (*Pay {1} to draw a card.* 0;
  *Spend this mana only to cast …*): a separate construction, not built here.
- *costs {X} less, where X is …*: `english-v3-where-variable-clause`; the
  *where* clause is not built here.
- *the greatest/least*, *tied for*, *rather than*, *without paying its mana
  cost* (Cost Noun, not the verb).
- Comparative quantity Determiners and comparative governors:
  `english-v3-comparative-quantity-determiners`,
  `english-v3-comparative-complements`.

## Landing record

In addition to the standard record: the replacement Cost frame(s) and the
disposition of each retired legacy slot; the internal analysis chosen for *{1}
less* and its basis; Neonate's Rush status; before/after counts stamped with
change ids; timings as integer ns and ns/B with host load and worker count.
