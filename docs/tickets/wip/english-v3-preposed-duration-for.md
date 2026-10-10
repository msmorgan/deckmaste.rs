---
needs: []
---
# Read a preposed duration *for as long as* Adjunct: For as long as that card remains exiled, its owner may play it.

## Why

Duration *for as long as* + clause reads at the end of a clause ("Its owner
may play it for as long as it remains exiled." 2) but not preposed with a
comma. Blocker report at trunk `ymqwzour`
(`target/english-v3/blockers-wave8.json`): hidden under `at:it .` 6, `at:"
<end>` 3, `at:control .`, `at:cost .`, `at:block .`. `cargo xtask english-v3
grep 'For as long as'` (capitalised, so sentence-initial) finds **28** faces,
all unread; 28 failing lines match, **25** of them the only failing line of
their face. Lower-case *for as long as* is on 183 faces, 154 covered. Clause
shapes after the preposed form (failing-line faces): *that card remains
exiled* / *this creature remains tapped* 18, *… has a … counter on it* 6,
*you control …* 2, others 2.

## Goal

A *for* PP whose Complement is the equative duration phrase *as long as* +
clause reads as a preposed Adjunct, followed by a comma, of an imperative or
declarative clause, with the Readings the same PP has clause-finally.

## Analysis

CGEL Ch. 7 §5.1(d), p. 640 (cited by `AdverbComplementPreposition`) and
Ch. 13 §4.5(d), p. 1134 back the duration analysis already landed
(`english-v3-as-long-as`); this ticket relies on no further passage.
Preposing it is a project ruling. Glossary: **Duration Adverb**, **Scalar
Equality**, **Preposition Function Licence**.

Existing route (`crates/deckmaste_english_v3/src/declarations.rs`):
`AdverbComplementPreposition` (`require head.AdverbComplementClass =
Duration; require complement.DurationUse = Yes`), with *for* declared
`AdverbComplementClass: Duration` and `PrepositionFunctionLicence:
…|Adjunct|…|PreposedAdjunct` in `lexicon/core.ron`. So the lexeme already
licenses the position (`licence_preposed_adjunct`); the PP built by this
construction does not reach it. Conditional *as long as* (`vocab:Preposition/
AsLongAs`, `Adjunct|PreposedAdjunct`) reads preposed: "As long as that card
remains exiled, you may play it." 1.

Orchestrator rulings 2026-10-10 (the 2026-10-06 ruling in
`english-v3-as-long-as` stands: scalar equality with *long* is admitted only
inside *for*'s duration Complement, and conditional *as long as* is a separate
Compound Preposition):

- PD1. Find which export of `AdverbComplementPreposition` keeps its PP out of
  the preposed-Adjunct position (candidates: it exports
  `ClauseComplementBoundary = None` although its Complement ends in a finite
  clause; `InterveningAdjunctLicence`; the comma rule) and correct that
  export. No new Construction.
- PD2. The licence is *for*'s declared `PreposedAdjunct` and the Complement's
  `DurationUse`. No test on *long* or *for*.
- PD3. One analysis per string. *For as long as X, Y* is the duration PP
  preposed. It must not also read as *for* + conditional *as long as*, nor as
  conditional *as long as* with a stray *for*; "As long as X, Y." keeps
  exactly its Readings and gains no duration twin.
- PD4. The preposed PP attaches to the clause, once: not also to each
  coordinate of *loses all abilities and can't attack or block* (Immovable
  Rod). Report coordination scope pairs if they arise; they are the existing
  preposed-Adjunct behaviour, not a new one.
- PD5. Newly covered faces are gains. Reading-count increases on the 154
  covered *for as long as* faces are not expected (all clause-final) and are
  reported with their shape; an increase on a covered *As long as …,* face is
  a defect under PD3.

## Witnesses

Each is the last sentence(s) of the face's only failing line at `ymqwzour`.
Probed 2026-10-10 with the `ymqwzour` binary (`--category sentence --readings
3`; 3 = enumeration capped). Re-probe on the claim parent first. All 0.

- Prowl, Stoic Strategist: "For as long as that card remains exiled, its
  owner may play it." (0)
- Release to the Wind: "For as long as that card remains exiled, its owner
  may cast it without paying its mana cost." (0)
- Preacher: "For as long as this creature remains tapped, gain control of
  target creature." (0; the face adds *of an opponent's choice they control*)
- Minas Morgul, Dark Fortress: "For as long as that creature has a shadow
  counter on it, it's a Wraith in addition to its other types." (0)
- Flameshape: "For as long as they remain exiled, you may play them if you
  control a Wizard." (0)

Also 0 today: "For as long as you control this creature, it can't attack."
Controls that must not change: "Its owner may play it for as long as it
remains exiled." 2; "Gain control of target creature for as long as you
control this creature." 1; "As long as that card remains exiled, you may play
it." 1; "As long as that creature has a shadow counter on it, it's a Wraith
in addition to its other types." 3; "Until end of turn, you may play that
card." 1; "That card remains exiled." 1.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `as long as` case-insensitively (duration and conditional: PD3
is about their boundary)
plus 300 random covered controls (`cargo xtask english-v3 diff --manifest …
--random 300 --seed 20261008`), `--workers 6 --samples-per-face 0`, compared
with `cargo xtask english-v3 diff` against the claim parent. Zero lost;
PD5 for increases. Full `cargo test -p deckmaste_english_v3`
and `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings`,
once, at the end. A rebuild after a grammar edit takes 2–3 minutes and the six test binaries
about 75 seconds, so batch edits.
Tests assert the role (the *for* PP as preposed Adjunct of the main clause, *as long as* +
clause as its duration Complement, the PD3 negatives), never an incidental count or a Debug-printed
tree. Attestation by `cargo xtask english-v3 grep` only.

## Out of scope

- *it has "…"* after the comma where the quoted ability fails for its own
  reason (Mathas, Obsidian Fireheart, Makeshift Mannequin: `at:" <end>`).
- *It can't phase in for as long as you control this Saga.*
  (`english-v3-phase-in-out-verb`).
- Clause-final *for as long as* faces that are unread for another cause (29
  unread lower-case faces).

## Landing record

1. **Census delta**: claim-parent tree (`rtzolrtx`, before the edit) → refreshed
   implementation (`vlvswmnl`, empty child of `rtzolrtx` when measured): bounded
   manifest of every supported face matching `as long as` case-insensitively
   anywhere, plus 300 covered controls sampled by `english-v3 diff` with seed
   20261008; 1,372 distinct faces (11 overlaps). Covered **1,193 → 1,218**;
   total Readings **21,601 → 25,998**. Final diff: 25 gained, 0 lost,
   0 decreases, 0 increases on previously covered faces, 0 digest-only;
   1,347 unchanged, no source-hash changes, no issues. Lost faces: none.
   The duration change alone (`rtzolrtx`, pre-refresh) gained 23 faces,
   reaching 1,216 covered and 25,986 Readings. Refresh imported
   `roomopmrpkys`: The Pandorica gained 8 Readings and The Phasing of Zhalfir
   gained 4; no other face changed in the refresh diff. No unintended-only
   gains found. Evidence:
   `target/english-v3/english-v3-preposed-duration-for/{manifest,before,after,diff}.json`;
   preserved `after-pre-refresh.json`, `diff-pre-refresh.json` and
   `refresh-diff.json` separate the trunk gains. Both comparisons used
   `diff --lines`. Corpus grep found no `for long` or `for so long` uses
   outside this selector on `rtzolrtx`.
   Review audit (`kwpypvst`): the sole eligible Preposition head is
   `vocab:Preposition/For` (`AdverbComplementClass = Duration`); the sole
   duration Adverb is `vocab:Adverb/Long` (`DurationUse = Yes`). The identity
   union includes all 154 faces using `AdverbComplementPreposition` in the
   default workspace's `baseline-wave8.json`, all 213 supported faces matching
   `\bfor\b[^\n]*\blong\b` case-insensitively, and the original manifest
   with its 300 seeded controls. All are already among the same 1,372 faces;
   the original before-census identities exactly equal the audited manifest.
   Re-running the after-census and stock `diff --lines` confirms 25 gains
   (23 own, two phase-in/out), zero lost/decreases/increases/digest-only and
   1,347 unchanged. No covered face changed, and no other head is licensed.
   Evidence: `wide-{manifest,selector,grep,after,diff}.json` and corresponding
   logs in the same evidence directory. Review refresh (`kxpkwpno`) imported
   `nzmrrqkysxto`: Archon of Coronation gained 3 Readings and The Masamune
   gained 15. The bounded recheck has 1,220 covered, 26,016 Readings:
   claim-parent delta 27 gains (23 own, four attributed to trunk), zero lost,
   decreases, increases or digest-only; 1,345 unchanged. The refresh-only
   diff has two gains and 1,370 unchanged. Evidence: `review-refreshed-after`,
   `review-refresh-diff` and `review-final-diff` JSON/log files. The next
   refresh imported named-token apposition (`tlpwnxqn`); rechecking the same
   manifest on `ynosvvpz` yielded all 1,372 faces and all lines unchanged,
   including counts and digests (`token-refresh-diff.json`). Final totals
   remain 1,220 covered and 26,016 Readings, with 23 own gains. The subsequent
   kind-of-counter refresh (`wkooyuko`) was measured on `kmzutkzt`:
   `counter-refresh-diff.json` likewise reports all 1,372 faces and all lines
   unchanged, with zero count or digest changes. The same final totals stand.
2. **Witnesses** (`rtzolrtx`, all completely enumerated by sentence probes):
   Prowl, Stoic Strategist **0 → 1**; Release to the Wind **0 → 2**;
   Preacher **0 → 1**; Minas Morgul, Dark Fortress **0 → 6**; Flameshape
   **0 → 2**; “For as long as you control this creature, it can't attack.”
   **0 → 1**. No already-reading witness was retained as a failure.
   The six controls remain **2 / 1 / 1 / 6 / 1 / 1**; the counter conditional's
   claim-parent count is six, superseding the ticket's historical three.
   Immovable Rod **0 → 3**: one duration PP per Reading; the whole-coordination
   versus first-Clause scope pair is the existing `InitialPreposition`
   behaviour permitted by PD4. The equivalent preposed `Until end of turn`
   scope control has the same three shapes (`rtzolrtx`).
3. **Spot checks** (`rtzolrtx`): Prowl, Stoic Strategist; Release to the Wind;
   Preacher; Minas Morgul, Dark Fortress; Flameshape; Hoarding Broodlord;
   Aerial Extortionist; Elite Spellbinder; Soul Partition; Temporal Aperture.
   Each selected Reading has the duration *for* PP in preposed Clause Adjunct
   function, retaining `EquativeAdverb` and its finite Clause Complement.
   Temporal Aperture nests it after the preposed *until* PP; its spot probe
   inspects eight Readings, while the census completely enumerates its line.
   Probes and selected trees are under the evidence directory above.
4. **Deviations and STOPs**: no Construction, Frame or lexical route added or
   deleted; the existing duration construction retains its head's declared
   function licence and exports the existing Adjunct-only Complement Function
   Restriction, preventing nominal/compound leakage. No loader or lexicon edit.
   The Method seed was corrected to the user's binding 20261008. The new Rod
   test initially expected only outer scope; corrected full Reading values
   retain the PD4 pair and test it against the existing *until* route. No
   further duration-Construction change followed the bounded census. Refresh
   imported phase-in/out lexicon data and tests from `roomopmrpkys`, plus
   ticket-only updates; the same bounded manifest and full suites/clippy for
   both affected crates were rechecked. No STOPs or glossary gaps.
5. **Tests** (`vlvswmnl`, refreshed implementation):
   `cargo test -p deckmaste_english_v3` passed all **905** tests;
   `cargo test -p deckmaste_lexical_source` passed all **84** tests;
   `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings` and
   `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings` passed.
   No failures or ignored tests remain. Before refresh (`rtzolrtx`), the first
   full English attempt failed the new Rod scope expectation; the corrected
   full attempt passed **900** tests. The first clippy attempt found a
   needless-by-value test-helper argument; borrowing it retained full Reading
   equality, and corrected clippy passed. The refreshed full suite also tests
   that final borrowed helper. Assurance counts for this ticket: restored 0,
   re-spelled 0, ignored 0, added 4, removed 0; no removal to justify. The five
   additional inherited tests belong to `roomopmrpkys`. The new duration module
   is registered exactly once in `suite_4`. `cargo build -p xtask` passed both
   before and after refresh. `cargo xtask cite check --list-noncompliant`
   reports none; `cargo xtask cite check` reports 0 stale;
   `jj --no-pager diff --git | cargo xtask cite audit --diff` selects no new
   citation sites. No newly cited rules require blessing. No Lean checks or
   full-corpus census ran. After review refresh (`kxpkwpno`),
   `cargo build -p xtask`, `cargo test -p deckmaste_english_v3` (909 tests),
   `cargo test -p deckmaste_lexical_source` (84 tests), and both preceding
   exact clippy commands passed again; no failures or ignored tests.
   Four additional inherited tests belong to `nzmrrqkysxto`. Following the
   named-token grammar refresh (`ynosvvpz`), `cargo build -p xtask`, full
   `cargo test -p deckmaste_english_v3` (913 tests), and
   `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings` passed again.
   The four new inherited tests belong to `tlpwnxqn`; lexical-source files
   were unchanged in that refresh, preserving its full-suite/clippy results.
   After kind-of-counter refresh (`kmzutkzt`), `cargo build -p xtask`,
   `cargo test -p deckmaste_english_v3` (918 tests),
   `cargo test -p deckmaste_lexical_source` (84 tests), and both exact clippy
   commands above passed. Five inherited tests belong to `wkooyuko`.
6. **Timing** (`kmzutkzt`): the final refreshed bounded after-census took
   **99.81 s corpus wall** with **6 workers**, under `/usr/bin/time -v`.
