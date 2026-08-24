---
needs: [english-v2-plan-07-editorial-and-nominal-grammar]
---
**Complete and integrate Stage 5 Plan 08: ability and logic grammar.**
This plan-boundary completion ticket carries only the reviewed Plan 08 work
and must land before Plan 09 implementation starts.

The completed slice declares generated plain, triggered, activated, ordinary
unlabelled U+2022 modal, finite-auxiliary, predicate/finite-clause
coordination, and conditional attachment structure. The Plan 07 modal
deferral is amended only for ordinary unlabelled U+2022 modal abilities;
advanced modal, keyword, ability-word, reminder, and frame syntax remains
deferred. `target` is a linguistic determiner in the English grammar, not a
targeting-semantic noun-phrase category: bare target phrases are singular,
while plural target phrases require explicit quantification.

The 427-row production candidate pool has source fingerprint
`e85359d7b8c578df13dff2fdf7c743a520a5b367d5ed25ab0a5f03cb8b3637dd`
and SHA-256
`8b9870dba508681ecc25dc43b21500d82cd941d2408456810ad39f799ffb425f`.
Every complete OracleText/context pair has one authenticated classification in
`crates/xtask/src/english_v2/plan08_candidate_results.tsv`; that 427-row
manifest has SHA-256
`0020c88cadcc5e892c203a3860a7ee37b09b08a5175ce022854ab9359db6a8c3`
and the exact partition is:

- 66 Plan 08 selected: 56 `ability.activated` and 10
  `ability.triggered`;
- 358 still missing a Plan 09 predicate boundary;
- 3 still requiring a Plan 10 attachment boundary;
- 0 Plan 08 grammar defects.

The frozen production delta is the 118-row
`crates/xtask/src/english_v2/plan08_targets.tsv`, with the same source
fingerprint and SHA-256
`c6dbe1e53031c0f6394d1b0644c7fecdb8391a0fe7484c272ba1cc7fd364818e`.
It contains the 66 selected candidate rows plus 52 independently explained
out-of-pool additions. Its exact family counts are 56 `ability.activated`, 28
`ability.plain-modal`, 10 `ability.triggered`, 23 `clause.coordination`, and
1 `finite.auxiliary`; no out-of-pool addition is unexplained.

The schema-2 coverage lock is an exact add-only 608→726 ratchet (+118).
Every baseline ID remains, all and only the 118 frozen targets are added, and
the lock equals the complete production `SelectedCovered` set. The baseline
ID-set SHA-256 is
`35ea73406725742d51b92df60bb07a261ed7570d6767caf59306a88785de87c2`;
the final lock SHA-256 is
`c79c684025df3ed8efc5b34537758788cdfb2b88f0755b5d3f378820924a51a6`.
The final corpus census is exactly 32,641 total / 726 selected, covered,
unique, byte-exact, and totally owned / 31,915 ordinary parse failures.

The specificity census moved from six specificity-resolved rows to zero.
Each former row was inspected against the pre-modal census and final parser:
By Force
(`3b64537c45c8a85e9a72ccfda500af9c174ad443545b7e4916fdefe71c601ec3`),
Hex (`99fcb52e28fb1c1237f50f57c87866ae3e4ca052f586d23932dbd4c4c134acb8`),
Peace and Quiet
(`a3dcd93656d5f2ce285bba169425ce76e6de71b5bfe283a471f6cab07ffae09d`),
Rack and Ruin
(`c52cbc561c5e02ef54686898503e4381f31980983b87f41e164260e3f04e36fa`),
Rain of Salt
(`592e3b0145fc335c64db7c0b3dde48fa65536dfca83f844ecc6eea679da20c3c`),
and Violent Ultimatum
(`d3860af64b6bf5eadbb5bf76be4dbf99367c9efc7d50fc27afc22f3b64f8c168`)
each moved from two candidates resolved by specificity to one unique candidate
while retaining the same selected quantified-plural construction path. No
dominance rule, selection exception, lexical-number heuristic, or targeting
semantics was added. Specificity-resolved, exception-resolved, unresolved-tie,
internal-failure, exception-use, selected-uncovered, round-trip-mismatch,
ownership-failure, gap, overlap, synthetic-claim, and provenance-plan-mismatch
counters are all zero.

Completion evidence on the task-closed tree:

- construction core: 315 library tests and its documentation tests passed;
- construction proc macro: 2 catalog tests, all 69 trybuild fixtures, 28
  compiled-consumer tests, and its documentation tests passed;
- English v2: 131 library tests, 154 integration tests, and 2 documentation
  tests passed;
- xtask English v2: 167 library tests and 9 CLI tests passed;
- the seven focused stale-construction-fixture checks passed exactly 7/7; the
  structural deferred-boundary census, complete candidate partition, and
  frozen target/full-lock focused checks each passed exactly 1/1;
- strict all-target/all-feature Clippy for the four closure crates, the five
  Plan 04 authority tests, expand/report/probe/inspect smokes, coverage check,
  and citation checks passed; citation check inspected 16,248 citations with
  0 stale, and the changed-file citation audit found 0 sites;
- candidate-pool fingerprint, partition counts, selected identity/render/
  ownership/claims/family, structural deferred boundaries, target membership/
  disjointness/additivity, out-of-pool explanation, and complete lock equality
  each have an independent literal oracle and mutation-authenticated RED. The
  deferred census hash is
  `cae10d975c30177e7b9970b459b9adb9058d5250c3e48a6bfe845c3c80b4047d`;
- the Plan 04 scratch-copy perturbation changed one declared surface and one
  declared invariant in a single expansion build. Scanner, renderer, claims,
  and constructor behavior tracked both edits; both old behaviors produced no
  consumed mirror or selected parse. The live workspace diff hash remained
  `534c5a0c6b868526fb27f47e0a41dff40272ac20ae36e763e6dafaf0b1d33089`
  before and after.

The seven separate Plan 08 gate samples were expand 1.338185062s, report
0.907188130s, parse 14.073277505s, roundtrip 12.695260193s, ambiguity
12.377303111s, coverage 14.238597634s, and require-complete 12.894259355s.
Every sample stayed at or below 16.26 seconds. Expand emitted
`WARNING english-v2-plan08-relative-slowdown gate=expand elapsed_seconds=1.338185062 warning_seconds=1.1815357365`;
report emitted
`WARNING english-v2-plan08-relative-slowdown gate=report elapsed_seconds=0.907188130 warning_seconds=0.7659729825`.
No other gate emitted a slowdown warning. The require-complete child reported
the expected incomplete 726-of-32,641 Stage 5 boundary with
`expected_incomplete=true`; the authenticated manifests, coverage lock, and
closure gates themselves are complete and exact.

Plan 08 closure also repaired stale construction evidence exposed by the
enlarged generated plan: the exact representative inventory is 93 rather than
92, the synthetic inventory is 112 rather than 110, and the generated
Agreement helper name is used by the AST harness. Literal name/origin/order
assertions and deletion/substitution mutations authenticate those updates. A
test-only fixture's unconditional Agreement writer, which contradicted an
admitted intrinsic value, was removed after the full construction gate exposed
it; invalid cross-pair rejection remains covered. Standard constraints apply.
