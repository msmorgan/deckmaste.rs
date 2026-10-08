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
12. Tests: `cargo test -p <crate>` and clippy for every crate changed; Lean checks only if Lean sources or plugin definitions changed.

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

### STOP — predicative-as widening revives excluded Readings (2026-10-07)

**Unfinished; not integrated.** The focusing implementation and its four required
witnesses work, but the final full census exposed wrong Readings caused by the
assigned clause-final predicative-*as* widening. These are defects, not coverage
gains. No ruling has been superseded and no excluded Reading is accepted here.

The claim parent is `pvmzprsz`; `before.json` reuses the source-equivalent full
census at `nplzxtwkzkyslwmlysqrnpmmmuvnknxv`, covered **21,486**
(`baseline-provenance.json`). The one authorized full feature census at
`pnqvxvwqwkxqnqzrwsyppzyonqxlunrw` reports a raw covered count of **22,124**:
32,828 faces, 10,704 unread, 7,472 unique and 14,652 multiple; 912,136 Readings.
Face-by-face comparison finds zero lost faces and zero Reading-count decreases.
The raw 638 newly admitted faces are **not a validated gain claim**.

| Witness | Claim-parent Readings | Attempt Readings | Defect |
| --- | ---: | ---: | --- |
| Counterbore (`1198ec53-9050-417d-812f-692e1378d346#card`) | 0 | 39 | *as that spell* is a predicative PP Adjunct instead of the comparative Complement in *the same name as that spell*. |
| Clone (`42226b87-0746-4ebf-9fd0-108d508462af#card`) | 0 | 18 | *as a copy of any creature* is an Adjunct instead of Enter's selected Complement. |
| Aven Shrine (`09c450cf-5f22-45c2-86ed-21b85552a96f#card`) | 0 | 9 | The same comparative-to-Adjunct defect. |

The [fixed-cost-phrases STOP and orchestrator resolution of 2026-10-06](../done/english-v3-fixed-cost-phrases.md#stop--comparative-admission-2026-10-06-resolved-by-orchestrator)
expressly reject the comparative Adjunct analysis and defer comparative licensing.
The [systemic-residuals predicative-as ruling](../planned/english-v3-systemic-residuals.md)
requires Enter's *as a copy* to be a selected Complement, never a trailing Adjunct.
The ticket's assigned position widening cannot waive either ruling. A declared-feature
exclusion/selection repair needs a scope ruling; no word-naming guard or preference
can repair admission. The request to extend scope and authorize one replacement
full census is pending. The binding one-full-run budget has been exhausted, so
no additional `--all` run has been performed.

Evidence is under `target/english-v3/english-v3-focusing-only/`: `after.json`,
`comparison.json`, `as-without-focus-gains.json` (all 78 newly admitted faces
without focusing), `as-audit.json` (selector-only census), and the three named
`*-as-tree.txt` extracts. The three previously covered increases are Deputy of
Detention **96 → 396**, Banishment **96 → 396**, and Phyrexian Processor **2 → 5**;
their added Readings also require audit before any landing.

The full command was exactly
`cargo xtask english-v3 --all --workers 12 --samples-per-face 0 --output target/english-v3/english-v3-focusing-only/after.json`.
At the measured change / raw covered 22,124, wall time was **183,119,704,184 ns**,
with 12 workers and host load **[7.3154296875, 5.84033203125, 8.9130859375]**;
per-byte thread CPU was **501,981 ns/B**. The wall time exceeds the quiet-host
**16,260,000,000 ns** advisory; it was not a quiet-host measurement. Structural
validation found zero issues, duplicate Readings, cyclic derivations or internal
failures; this does not establish that the admitted analyses are correct.

Before these discoveries, `cargo test -p deckmaste_english_v3` passed **381** tests,
`cargo test -p deckmaste_lexical_source` passed **56**, and
`cargo clippy -p deckmaste_english_v3 -p deckmaste_lexical_source --tests -- -D warnings`
passed. Those green results missed the excluded Readings. Two additional regression
tests retain the Counterbore and Clone exclusions and currently fail
(`as-regressions-red.log`); they are not ignored or weakened. No test was removed.
No xtask tests, Lean checks or derived gate were run. A pre-census Kata refresh
changed no grammar, lexicon, runtime or census sources (`pre-census-refresh-proof.json`).
The STOP tests fail as expected: **2 failed, 381 filtered out**, with neither
ignored. Nightly formatting passes; `cargo xtask cite check` reports **0 stale**.
