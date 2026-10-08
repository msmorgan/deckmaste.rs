---
needs: []
---
# Read the *same … as* equality comparative: all other lands with the same name as that land

## Why

*the same N as NP* never reads. On census `xnkuznpo` (docs-only descendant of
`vvxtllnq`; 32,828 supported faces, 23,459 covered, 9,369 unread; wave-4
recon of 2026-10-08), the recon bucket "*same … as*" touches **110** unread
faces and is the sole cause on **75**. It is also one of the two
prerequisites of `english-v3-clause-final-predicative-as` (orchestrator
ruling 2026-10-07). Counts are unread faces *touched* / *sole* as defined in
the recon; surface counts, not gain forecasts.

Probes (admitted roots): "Destroy target land and all other lands with the
same name as that land." 0, while "Destroy target land and all other lands
with that name." 1; "Your opponents can't cast spells with the same name as a
card exiled with Circu." 0, while "Your opponents can't cast spells with that
name." 1.

## Goal

*same* governs an *as* + NP comparative Complement inside the NP it
premodifies (*the same name as that land*, *the same mana value as that
spell*), realised byte-exactly. The *as* phrase is the Complement licensed by
*same*, not an Adjunct.

## Analysis

CGEL, Ch. 13 §1.1, p. 1100: *as* is the main marker of equality comparison,
scalar or non-scalar (*the same bus as last time*). Ch. 13 §5.1, p. 1139
("*Same as* with NP complement"): the comparative complement commonly
consists of *as* + NP. The orchestrator resolution of 2026-10-06 (fixed-cost
phrases) assigns this Complement to *same*'s licence, not to an Adjunct
licence.

Existing route: `Same` is a member of `vocab AttributiveAdjective`
(`vocabulary.rs:113`). The comparative-Complement route already reads
*equal to*, *less than*, *other than* through `ComparativeGovernorHead` and
`NominalComparativeComplement`; `vocab:Preposition/As` declares
`ExpandedComparisonMarker: Equality` (`core.ron:181`). This ticket gives
*same* the equality governor property that route reads.

## Witnesses

Each witness's failing units all match this construction and no other recon
STRONG bucket (`xnkuznpo`, 0 Readings each):

- Wake of Destruction: "Destroy target land and all other lands with the same
  name as that land." (control "… with that name." 1)
- Sever the Bloodline: "Exile target creature and all other creatures with the
  same name as that creature." (control "… with that name." 1)
- Echoing Truth: "Return target nonland permanent and all other permanents
  with the same name as that permanent to their owners' hands."
- Circu, Dimir Lobotomist: "Your opponents can't cast spells with the same name
  as a card exiled with Circu." (control "… with that name." 1)
- Extirpate: "Search its owner's graveyard, hand, and library for all cards
  with the same name as that card and exile them."

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, scope).
Deltas:

1. Verification budget (binding, orchestrator override 2026-10-08): one bounded
   census of every reachable face plus 300 random covered faces (seed 20261008),
   `--identity-manifest M --workers 12 --samples-per-face 0`, compared
   face by face (per-face `readings_sha256` and counts) against the claim
   parent's census; never run `--all`. Tests:
   `cargo test -p` and `cargo clippy -p … --tests -- -D warnings` for every
   crate whose code or consumed data changed (`deckmaste_english_v3` always;
   `deckmaste_lexical_source` if core.ron/verbs.ron/vocabulary.rs changed).
   No xtask tests, no Lean checks, no post-refresh re-census unless refresh
   touched grammar, lexicon or core. New tests go under tests/suite/
   registered in tests/main.rs. Evidence lives under the workspace's ignored
   `target/english-v3/`, never `/tmp`.
2. Demonstrate each witness failing with `cargo xtask english-v3 probe` before
   implementation; write the tests once and run them at the end (orchestrator
   override 2026-10-08).
3. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling.
4. Compare per-face Reading counts against the claim-parent census. Any
   decrease on a previously covered face is restored, or named and justified in
   the landing record; a decrease inherited from trunk is attributed by change
   id.
5. Spot-check at least 10 newly covered faces and list them by name with their
   Reading. No string may receive the same comparative Complement analysis
   twice. Inherited finite *as*-Clause alternatives are retained under the
   user ruling of 2026-10-08; their separate licensing hygiene is routed to recon.
   A wrong analysis introduced here is a defect, not a gain.
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

## Out of scope

- *the same* without *as* (*with the same name*, *the same is true for*):
  separate residual (86 touched).
- Clause-final predicative *as*: `english-v3-clause-final-predicative-as`.

## Landing record

1. **Census delta**: bounded claim-parent grammar `pnrnqolx` (measured on
   `lqzlkrrp` before implementation; source fingerprints verified) →
   `lqzlkrrp`: covered faces 334 → 420; Readings 5,619 → 28,414. Gained
   faces 86; lost faces none; Reading-count decreases none (`lqzlkrrp`).
   Increases 2: Banishment 96→296, Deputy of Detention 96→296 (`lqzlkrrp`).
   Digest-only 12: “the same” faces without *as*, counts unchanged; cause:
   the *Same* leaf now records `frame: Some(0)` because *Same* declares frames;
   no new Readings (`lqzlkrrp`): Bazaar of Wonders, Boxing Ring, Cultural
   Exchange, Endless Atlas, Fear Fire Foes!, Kitesail Skirmisher, Mimeofacture,
   Powerbalance, Sceptre of Eternal Glory, Sphinx of the Chimes, Wild Pair,
   Winnow.
   Per-face counts and `readings_sha256` compared; all 300 random faces
   unchanged. Exact realization, lexical ownership and node/word traversal:
   0 issues and 0 internal failures (`lqzlkrrp`).
2. **Witnesses** (`lqzlkrrp`): Wake of Destruction 4; Sever the Bloodline 4;
   Echoing Truth 2; Circu, Dimir Lobotomist 2,880; Extirpate 112 full-face
   Readings. Quoted-unit probes: respectively 4, 4, 2, 45, 28. All five
   quoted units had 0 Readings before implementation on `lqzlkrrp`, each
   demonstrated with `cargo xtask english-v3 probe` before grammar edits.
3. **Spot checks** (`lqzlkrrp`): selected Discontinuous Comparative Nominals,
   with the *as* + NP Complement licensed by the Premodifier: Wake of
   Destruction — *same name as that land*; Sever the Bloodline, Echoing
   Courage, Echoing Decay, Bile Blight and Homing Lightning — *same name as
   that creature*; Echoing Truth and Maelstrom Pulse — *same name as that
   permanent*; Echoing Calm — *same name as that enchantment*; Echoing Ruin —
   *same name as that artifact*.
4. **Deviations and STOPs**: added DiscontinuousEquality and
   DiscontinuousComparativeNominal because the adjacent governor-marker route
   cannot enclose a Nominal. Same now declares an empty frame for understood
   comparisons and a selected comparative frame; its former unframed lexical
   route is retired, while generic Adjective remains used by other lexemes.
   No loader change or word-naming guard. Added the missing Discontinuous
   Comparative Nominal glossary entry, distinguishing CGEL's *as* + NP account
   from the fixed-cost-phrases licensing ruling. The bounded run replaces
   Method's full census by the explicit orchestrator ruling of 2026-10-08.
   **STOP resolved by user ruling 2026-10-08**: retained-Reading contract wins for inherited finite *as*-Clause alternatives; 2 previously covered faces (Banishment, Deputy of Detention) carry inherited finite as-Clause Readings beside the new Complement Readings; on Deputy’s unit 30 of 106 Readings are as-Clause only and 0 carry both; 0 of 86 gained faces (`lqzlkrrp`). Deputy of Detention's *with the same name as that permanent until this creature leaves the battlefield* retains comparative Complement *as [that permanent]* and finite Adjunct *as [[that permanent until this creature] leaves the battlefield]*; no string receives the same-Complement analysis twice.
   Existing *as*-Clause licensing of a bare NP after *as* (no verb) is routed to recon as [as-Clause verbless NP licensing](../planned/english-v3-as-clause-verbless-np-licensing.md), outside this ticket's scope. Circu's test initially assumed its source PP always fell inside the comparison; corrected it to retain distinct attachments.
5. **Tests** (`lqzlkrrp`): `cargo test -p deckmaste_english_v3` ran once:
   520 passed; the new Circu scope assertion failed, was corrected, and
   `cargo test -p deckmaste_english_v3 --test main
   same_as_comparative::witnesses_keep_the_complement_inside_the_premodified_nominal`
   passed (1 test). The remaining `cargo test -p deckmaste_english_v3 --doc`
   check passed (0 doctests), without rerunning the full suite.
   `cargo test -p deckmaste_lexical_source`: 23 passed,
   1 inherited failure, `card_names::tests::generated_nicknames_equal_the_streamed_computation`
   (Shang-Chi and the Ten Rings), owned by `english-v3-name-lexeme-scope`.
   `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings` passed after
   correcting the new test's collapsible-if diagnostic;
   `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings` passed.
   `kata kanban check` found the inherited quoted dependency in
   `english-v3-derived-punctuation-forms`; unchanged from claim parent.
   Assurance: restored 0, re-spelled 1
   (`selected_and_comparative_as_does_not_become_a_comparative_adjunct`),
   added 3, removed 0, newly ignored 0 (`lqzlkrrp`). Review amendment
   (`rturkpmo`): Circu's witness uses `readings_on_card` with
   *Circu, Dimir Lobotomist* after `english-v3-name-lexeme-scope`, preserving
   every assertion; re-spelled 1, restored 0, added 0, removed 0, ignored 0.
   `cargo test -p deckmaste_english_v3` ran once: 525 passed, 0 failed,
   0 ignored; 0 doctests.
   `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings` passed
   (`rturkpmo`). No census rerun.
6. **Timing** (`lqzlkrrp`): bounded census wall time 8.132 s including setup,
   12 workers, 540 faces: every supported face whose raw text contains *same*
   (240) plus 300 random covered faces, seed 20261008, selected by
   `target/english-v3/english-v3-same-as-comparative/manifest.json`;
   `--identity-manifest … --workers 12 --samples-per-face 0`, output
   `target/english-v3/english-v3-same-as-comparative/after.json`. No full corpus
   census was run.
