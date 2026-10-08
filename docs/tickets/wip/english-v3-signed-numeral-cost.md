---
needs: []
---
# Read a signed numeral as an ability Cost: −2: Destroy target creature

## Why

No ability whose Cost is a signed numeral reads. On claim parent `pvmzprszmnpt` (32,828 supported faces, 21,486 covered,
11,342 unread; wave-3 recon of 2026-10-07), every one of the 314
supported Planeswalker faces is unread, and all 878 failing `+N:`/`−N:`/`0:`
lines fail (518 `−`, 310 `+`, 50 `0`). The recon bucket "loyalty ability line
(+N: / -N: / 0:)" touches **314** unread faces and is the sole cause on **105**.
Counts are unread faces *touched* (at least one localised failing unit
matches) / *sole* (every failing unit matches and no other recon STRONG bucket
does). They are surface counts, not gain forecasts.

Probes (admitted roots): "{1}: Untap target permanent." 1; "+1: Untap target
permanent." 0; "0: Untap target permanent." 0; "−2: Destroy target creature."
0, while "Destroy target creature." 1; "−X: Destroy target creature." 0.

## Goal

An ability line reads with a Cost that is a signed numeral before the colon:
`+` or U+2212 `−` followed by a numeral or *X*, or bare `0`, realised
byte-exactly. Reuse the existing signed `ScalarComponent` forms (`"+"`/`"-"`
plus `UnsignedScalar`) where their signs match the source characters; the
U+2212 sign gets its own declared form, not a normalisation.

## Analysis

This is an Oracle document form, not an English construction CGEL describes;
no CGEL citation applies. The activated-ability Cost/colon structure is the
grammar's existing `ActivatedAbility`/`Cost` route; this ticket adds one Cost
form to it.

## Witnesses

Each witness's failing units all match this construction and no other recon
STRONG bucket (`pvmzprszmnpt`, 0 Readings each):

- Ob Nixilis, the Hate-Twisted: "−2: Destroy target creature." (control
  "Destroy target creature." 1)
- Kiora, Behemoth Beckoner: "−1: Untap target permanent."
- Arlinn, Voice of the Pack: "−2: Create a 2/2 green Wolf creature token."
- Jiang Yanggu, Wildcrafter: "−1: Put a +1/+1 counter on target creature."
- Teyo, the Shieldmage: "−2: Create a 0/3 white Wall creature token with
  defender."

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
   Reading. A signed numeral read as a scalar modifier of the following
   clause, or a Cost Reading that changes the source bytes, is a defect. A wrong analysis that parses is a defect, not a gain.
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

- Ability bodies that fail on their own (e.g. *Amass Zombies 2*): note them.
- Engine semantics of loyalty abilities.

## Landing record

1. **Census delta**: `pvmvwtuq` (source-identical to original claim parent
   `xrlqpono`): 23,216 covered faces / 1,147,462 Readings → `xnkuznpo`:
   23,459 / 1,605,535. Refreshed trunk `qxqootwz`: 23,352 / 1,199,416;
   this ticket adds 107 faces / 406,119 Readings (`xnkuznpo`). Lost faces:
   none; count decreases: none; all previously covered digests unchanged
   against both baselines (`xnkuznpo`). Planeswalkers: 0 (`pvmvwtuq`) →
   107 of 314 (`xnkuznpo`). Final census (`xnkuznpo`): 9,369 no Reading,
   8,170 one, 15,289 multiple; 0 validation/internal failures; exact
   realization and node/word traversal passed.
2. **Witnesses** (`xnkuznpo`, full Documents): Ob Nixilis, the Hate-Twisted: 5; Kiora, Behemoth Beckoner: 2; Arlinn, Voice of the Pack: 14; Jiang Yanggu, Wildcrafter: 5; Teyo, the Shieldmage: 7.
3. **Spot checks** (`xnkuznpo`, first sampled Reading): each sequence below
   is the Cost of an ActivatedAbility with a separate Paragraph body;
   signed entries use SignedScalarCost, bare `0` uses ZeroScalarCost.
   Ajani Vengeant: `+1 / −2 / −7`;
   Ajani, Inspiring Leader: `+2 / −3 / −10`;
   Ajani, Mentor of Heroes: `+1 / +1 / −8`;
   Ajani, Nacatl Avenger: `+2 / 0 / −4`;
   Ajani, the Greathearted: `+1 / −2`;
   Angrath, Captain of Chaos: `−2`;
   Angrath, Minotaur Pirate: `+2 / −3 / −11`;
   Angrath, the Flame-Chained: `+1 / −3 / −8`;
   Arlinn Kord: `+1 / 0`;
   Arlinn, Voice of the Pack: `−2`
4. **Deviations and STOPs**: SignedScalarCost reuses PositiveScalar's `+`
   and NegativeScalar's `-`; U+2212 `−` is a separate NegativeScalar form.
   ZeroScalarCost declares the unsigned-zero exception. ScalarSign and its
   glossary entry distinguish explicit signs; the guard reads that feature.
   No lexicon additions or superseded routes; no grammatical STOPs or glossary
   gaps. Refresh inherited the as-though work (`tvknqzwq`, `xrxurmzr`, measured
   `qxqootwz`), with no inherited decreases. One initial `--all` invocation
   was interrupted during compilation after the refresh source gate failed;
   the census had not started. The landing measurement is the single completed
   full run. No CGEL claim was added for this Oracle notation.
   Unrelated residual: Elspeth, Sun's Champion's emblem instruction still
   fails as a standalone Document (probe `rymrokon`), outside this Cost fix.
5. **Tests** (`xnkuznpo`): `cargo test -p deckmaste_english_v3` passed
   (486 integration tests); `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings`
   passed. Refresh changed `core.ron`, so `cargo test -p deckmaste_lexical_source`
   passed (23 unit / 34 integration tests), and
   `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings` passed.
   Assurance (`xnkuznpo`): restored 0 / re-spelled 0 / ignored with blockers 0 /
   added 8 / removed 0. Five witness tests failed before the fix (`vvxtllnq`);
   independent Cost construction, sign identity and traversal tests now pass.
6. **Timing** (`xnkuznpo`): 214.70 seconds wall time, 12 workers,
   one full `cargo xtask english-v3 --all --workers 12 --samples-per-face 0
   --output target/english-v3/english-v3-signed-numeral-cost/after.json`.
