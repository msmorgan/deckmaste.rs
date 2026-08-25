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
while both `target <plural>` and `other target <plural>` require an explicit
higher quantifier.

The 427-row production candidate pool has source fingerprint
`e85359d7b8c578df13dff2fdf7c743a520a5b367d5ed25ab0a5f03cb8b3637dd`
and SHA-256
`8b9870dba508681ecc25dc43b21500d82cd941d2408456810ad39f799ffb425f`.
Every complete OracleText/context pair has one authenticated classification in
`crates/xtask/src/english_v2/plan08_candidate_results.tsv`; that 427-row
manifest has SHA-256
`918d7eaf1d0d66342b308b7290a2ab19621325749c59de9a4341a6a56bfbeac4`
and the exact partition is:

- 67 Plan 08 selected: 56 `ability.activated` and 11
  `ability.triggered`;
- 355 still missing a Plan 09 predicate boundary;
- 5 still requiring a Plan 10 attachment boundary;
- 0 Plan 08 grammar defects.

The frozen production delta is the 127-row
`crates/xtask/src/english_v2/plan08_targets.tsv`, with the same source
fingerprint and SHA-256
`4f6f749bcb93acb534b830a5393672a41a31ccc9c7a2deb4df7a8fb28ea3275e`.
It contains the 67 selected candidate rows plus 60 independently explained
out-of-pool additions. Its exact family counts are 56 `ability.activated`, 28
`ability.plain-modal`, 12 `ability.triggered`, 23 `clause.coordination`, and
1 `finite.auxiliary`, and 7 `nominal.demonstrative-possessive`; no out-of-pool
addition is unexplained.

The two new authenticated triggered rows are Mask of Intolerance, whose
structural existential condition is now parsed directly, and the out-of-pool
Karmic Justice, selected by the same generic determiner-scoped nominal
coordination that replaced the unconstrained stored article spelling.

The seven nominal additions are Smash to Smithereens, Peak Eruption,
Destructive Revelry, Poison the Well, Melt Terrain, Consign to the Pit, and
Cryoclasm. Each selects through the generic demonstrative-possessive form
`that <nominal>'s controller`; none is a card-specific production or a
targeting-semantic category.

The schema-2 coverage lock is an exact add-only 608→735 ratchet (+127).
Every baseline ID remains, all and only the 127 frozen targets are added, and
the lock equals the complete production `SelectedCovered` set. The baseline
ID-set SHA-256 is
`35ea73406725742d51b92df60bb07a261ed7570d6767caf59306a88785de87c2`;
the final lock SHA-256 is
`4138c19adff3822903d03d254c17af2810d31eabe1251a2c58550383d2bc4968`.
The final corpus census is exactly 32,641 total / 735 selected, covered,
unique, byte-exact, and totally owned / 31,906 ordinary parse failures.
The exact ownership census is 3,940 claims / 19,516 claimed bytes: 2,122
form-literal claims / 7,664 bytes, 386 vocabulary claims / 1,765 bytes,
1,037 lexeme claims / 7,954 bytes, 271 codec claims / 539 bytes, and 124
identity claims / 1,594 bytes.

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

- construction core: 319 library tests and its documentation tests passed;
- construction proc macro: 2 catalog tests, all 71 trybuild fixtures, 30
  compiled-consumer tests, and its documentation tests passed;
- English v2: 131 library tests, 162 integration tests, and 2 documentation
  tests passed;
- xtask English v2: 171 library tests and 9 CLI tests passed;
- the seven focused stale-construction-fixture checks passed exactly 7/7; the
  structural deferred-boundary census, complete candidate partition, and
  frozen target/full-lock focused checks each passed exactly 1/1;
- the finite-subject coordination/exclusion regression passed exactly 1/1
  across the seven concrete review witnesses, and the existential-condition
  regression preserves Mask of Intolerance's complete `there are` condition
  while changing only its later predicate. Both assert AST, visit, render,
  agreement, ownership, claims, capitalization/spacing, and reciprocal
  malformed negatives;
- all 12 admitted compound nominal-modifier families and all 12 excluded
  negative counterparts are behaviorally witnessed, including singular
  `type` and plural `types`; a separate full-noun-phrase witness derives `a`
  and `an` independently from the onset of each coordinated member;
- strict all-target/all-feature Clippy for the four closure crates, the five
  Plan 04 authority tests, expand/report/probe/inspect smokes, coverage check,
  and citation checks passed; citation check inspected 16,248 citations with
  0 stale, and the changed-file citation audit found 0 sites;
- candidate-pool fingerprint, partition counts, selected identity/render/
  ownership/claims/family, structural deferred boundaries, target membership/
  disjointness/additivity, out-of-pool explanation, and complete lock equality
  each have an independent literal oracle and mutation-authenticated RED. All
  360 deferred rows reach their exact frozen failure span and match exactly one
  of 46 exercised boundary rules. Every Plan 09 rule also selects through a
  controlled supported substitution at the reached predicate or cost
  boundary; finite `cycles` and `scry` are classified by their predicate
  syntax, while keyword-reference and ability-word scope is relationally
  authenticated against the frozen generated catalogs;
- the Plan 04 scratch-copy perturbation changed one declared surface and one
  declared invariant in a single expansion build. Scanner, renderer, claims,
  and constructor behavior tracked both edits; both old behaviors produced no
  consumed mirror or selected parse. The live workspace diff hash remained
  `aa90113e71d4f1b36d5b169ff2b43c26680a6bf14d0381837c6ef97d2cc716dc`
  before and after.

The seven separate Plan 08 gate samples were expand 1.492025159s, report
0.829736313s, parse 14.428164598s, roundtrip 13.206698021s, ambiguity
14.327824531s, coverage 14.506157883s, and require-complete 13.490116639s.
Every sample stayed at or below 16.26 seconds. Expand and report emitted their
named relative-slowdown warnings against the older 1.1815357365s and
0.7659729825s warning thresholds; the other five gates emitted none. The
require-complete child reported the exact structured incomplete 735-of-32,641
Stage 5 boundary with `expected_incomplete=true`; the wrapper remained nonzero
after emitting the usable sample. The marker is accepted only when schema,
source fingerprint, total, accepted, parse-failure, ambiguity, and internal-
failure fields all match the frozen census and the child exits normally with a
nonzero status. Arbitrary nonzero exits, signals, malformed or wrong censuses,
and successful children are ordinary outcomes and are never mislabeled. The
authenticated manifests, coverage lock, and closure gates themselves are
complete and exact.

The final review also closed both parked compiler debts instead of leaving
checked declarations bypassable. Every stored Agreement-constrained role and
its stored derivation dependencies are private, use the generated accessor
surface, and can be built only through the checked constructor boundary.
Agreement-bearing sums in abstract products derive their exact selected
intrinsic Agreement for required, optional, and sequence fields; sums with a
reachable contextual alternative are rejected at the authored product role
because abstract products cannot declare feature writers. External compile-
fail and compiled-consumer evidence pins the privacy, exact match/mismatch,
intrinsic render, and all three contextual-wrapper boundaries. Legitimate
English test consumers now use checked constructors or accessors.

The ordinary plural guard no longer admits `OtherTargetPluralSelector`, so
bare `other target creatures` and the homonymous `other target Equipment` are
ordinary parse failures. `all`, fixed-cardinal, `X`, `up to`, and `any number
of` wrappers remain unique, render-exact, visit-exact, and totally owned. This
repair did not change the 735 selected identities, either frozen manifest, the
coverage lock, the expansion inventory, or any zero-defect counter.

Plan 08 closure also repaired stale construction evidence exposed by the
enlarged generated plan: the exact representative inventory is 93 rather than
92, the synthetic inventory is 112 rather than 110, and the generated
Agreement helper name is used by the AST harness. Literal name/origin/order
assertions and deletion/substitution mutations authenticate those updates. A
test-only fixture's unconditional Agreement writer, which contradicted an
admitted intrinsic value, was removed after the full construction gate exposed
it; invalid cross-pair rejection remains covered. The final English-v2
expansion inventory is exactly 1,134 item/origin pairs, including 5 optional
roles, 32 sequence roles, and 14 uniform separators. Standard constraints
apply.
